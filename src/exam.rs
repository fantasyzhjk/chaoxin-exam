use crate::error::ApiError;
use crate::session::{CookieJar, Session};
use crate::tiku;
use maplit::hashmap;
use paris::*;
use playwright::api::{Browser, BrowserContext, Cookie, Page};
use playwright::Playwright;
use serde_json;
use std::io;
use std::path::Path;
use std::time::Duration;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::time;

pub struct Exam {
    _playwright: Playwright,
    pub context: BrowserContext,
    pub tiku: Option<Vec<tiku::题目类型>>,
}

impl Exam {
    pub async fn init() -> Result<Self, playwright::Error> {
        let playwright = Playwright::initialize().await?;
        playwright.install_chromium()?; // Install browsers
        let chromium = playwright.chromium();
        let browser = chromium.launcher().headless(false).launch().await?;
        let context = browser.context_builder().build().await?;
        Ok(Self {
            _playwright: playwright,
            tiku: None,
            context,
        })
    }

    pub async fn main(&self) -> Result<(), ApiError> {
        let page = self.context.new_page().await.unwrap();

        page.goto_builder("https://i.mooc.chaoxing.com")
            .goto()
            .await?;

        warn!("按回车继续 ↲");
        io::stdin().read_line(&mut String::new())?;
        page.close(None).await?;
        for page in self.context.pages()?.iter() {
            if page.title().await?.contains("考试") {
                self.exam(page).await?;
                break;
            }
        }
        Ok(())
    }

    pub async fn exam(&self, page: &Page) -> Result<(), ApiError> {
        let timu = regex::Regex::new(r#"(?ms)<h3 class="mark_name.*?>(\d{1,}\.).*?<span class="colorShallow.*?>(.*?)</span>.*?div.*?>(.*?)</div>"#).unwrap();
        let re = regex::Regex::new(r#"(?ms)(^ {16}$.*?<span.*?num_option.*?">(?P<answer>.)</span>.*?<div class="fl answer.*?">(.*?)</div>){1,}"#).unwrap();
        loop {
            let html = page.content().await?;
            if let Some(cap) = timu.captures(&html) {
                let num = &cap[1];
                let typ = &cap[2];
                let timu = &cap[3];
                let timu = scraper::Html::parse_fragment(timu);
                let timu = timu.root_element().text().collect::<String>();

                info!("{} {} - {}", num, typ, timu);

                let result = re.captures_iter(&html);
                if typ.contains("单选题") {
                    for cap in result {
                        time::sleep(Duration::from_millis(500)).await;
                        let ans = cap.name("answer").unwrap().as_str();
                        info!("{}", ans);
                        page.click_builder(&format!("span:has-text(\"{}\")", ans)).click().await?;
                    }
                } else if typ.contains("多选题") {
                    for cap in result {
                        let ans = cap.name("answer").unwrap().as_str();
                        info!("{}", ans);
                        page.click_builder(&format!("span:has-text(\"{}\")", ans)).click().await?;
                    }
                } else if let Some(tiku) = self.tiku.clone() {
                    if typ.contains("判断题") {
                        let ans = tiku::fuzzy_find(&tiku, &timu, "判断题").1;
                        println!("{:#?}", ans);
                        if let Some(tiku::题目类型::判断题 { content: _, answer }) = ans {
                            let answer = if answer { "A" } else { "B" };
                            page.click_builder(&format!("span:has-text(\"{}\")", answer))
                            .click()
                            .await?;
                        }
                    } else if typ.contains("填空题") {
                        let ans = tiku::fuzzy_find(&tiku, &timu, "填空题").1;
                        println!("{:#?}", ans);
                        if let Some(tiku::题目类型::填空题 { content: _, answer }) = ans {
                            let answer = answer.replace("(1)", "");
                            page.click_builder("#ueditor_0").click().await?;
                            page.keyboard.input_text(answer.trim()).await?;
                        }
                    } else if typ.contains("论述题") {
                        let ans = tiku::fuzzy_find(&tiku, &timu, "论述题").1;
                        println!("{:#?}", ans);
                        if let Some(tiku::题目类型::填空题 { content: _, answer }) = ans {
                            let answer = answer.replace("(1)", "");
                            page.click_builder("#ueditor_0").click().await?;
                            page.keyboard.input_text(answer.trim()).await?;
                        }
                    }
                }

                time::sleep(Duration::from_millis(100)).await;
                
                match page.query_selector("a:has-text(\"下一题\")").await? {
                    Some(ele) => {
                        ele.click_builder().click().await?;
                    }
                    None => {
                        break;
                    }
                }
                time::sleep(Duration::from_secs(1)).await;
            } else {
                break;
            }
        }

        warn!("填写结束, 按回车继续 ↲");
        io::stdin().read_line(&mut String::new())?;
        self.save_cookies().await.unwrap();
        Ok(())
    }

    pub async fn login(&self) -> Result<(), playwright::Error> {
        let page = self.context.new_page().await.unwrap();

        page.goto_builder(
            "https://passport2.chaoxing.com/login?newversion=true&refer=http://i.mooc.chaoxing.com",
        )
        .goto()
        .await?;

        warn!("登录后请按回车继续 ↲");
        io::stdin().read_line(&mut String::new())?;

        self.save_cookies().await.unwrap();
        page.close(None).await?;
        Ok(())
    }

    pub async fn save_cookies(&self) -> Result<(), playwright::Error> {
        let cookies = self.context.cookies(&[]).await?;
        let mut f = File::create("./cookies").await?;
        f.write_all(serde_json::to_string(&cookies).unwrap().as_bytes())
            .await?;
        Ok(())
    }

    pub async fn load_cookies(&self, path: impl AsRef<Path>) -> Result<(), playwright::Error> {
        let cookies: Vec<Cookie> = serde_json::from_str(&fs::read_to_string(path).await?)?;
        self.context.add_cookies(cookies.as_ref()).await?;
        Ok(())
    }

    pub fn load_tiku(&mut self, path: impl AsRef<Path>) {
        self.tiku = Some(tiku::load(path))
    }
}
