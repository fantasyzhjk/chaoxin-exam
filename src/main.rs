use std::path::Path;

use chaoxin_checkin::{CheckIn, Exam, Result};
use chrono::prelude::*;
use office::{DataType, Excel};
use paris::*;
use tokio::time::{self, Duration};

use rust_fuzzy_search::fuzzy_compare;

#[derive(Clone, Debug)]
enum 题目类型 {
    单选题 { content: String, answer: String },
    多选题 { content: String, answer: String },
    判断题 { content: String, answer: bool },
    填空题 { content: String, answer: String },
}

fn fuzzy_find(list: &Vec<题目类型>, timu: &str) -> (f32, Option<题目类型>) {
    let mut score = 0.0;
    let mut ti: Option<题目类型> = None;
    for item in list.iter() {
        let content = match item {
            题目类型::单选题 { content, answer: _ } => content,
            题目类型::多选题 { content, answer: _ } => content,
            题目类型::判断题 { content, answer: _ } => content,
            题目类型::填空题 { content, answer: _ } => content,
        };
        let s = fuzzy_compare(content, timu);
        if s > score {
            score = s;
            ti = Some(item.clone())
        }
    }
    (score, ti)
}

fn load_timu(path: impl AsRef<Path>) -> Vec<题目类型> {
    let mut excel = Excel::open(path).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    let mut list = vec![];
    for row in r.rows().skip(1) {
        let content = if let DataType::String(content) = row[0].clone() {
            content.trim().to_owned()
        } else {
            unimplemented!()
        };
        let answer = if let DataType::String(answer) = row[1].clone() {
            answer.trim().to_owned()
        } else {
            unimplemented!()
        };
        let typ = if let DataType::String(typ) = row[2].clone() {
            typ.trim().to_owned()
        } else {
            unimplemented!()
        };

        list.push(match typ.as_str() {
            "单选题" => 题目类型::单选题 { content, answer },
            "多选题" => 题目类型::多选题 { content, answer },
            "判断题" => 题目类型::判断题 {
                content,
                answer: if answer.contains("对") {
                    true
                } else if answer.contains("正确") {
                    true
                } else {
                    false
                },
            },
            "填空题" => 题目类型::填空题 { content, answer },
            "论述题" => 题目类型::填空题 { content, answer },
            _ => unimplemented!(),
        });
    }
    list
}

#[tokio::main]
async fn main() -> Result<()> {
    // let mut c = CheckIn::load("./courses.json").await?;
    // c.load_cookies("./cookies").await.unwrap_or_default();
    // c.login().await?;
    // success!(
    //     "[{}] 已登录",
    //     Local::now().format("%Y-%m-%d %H:%M:%S")
    // );

    let list = load_timu("JAVA程序设计.xlsx");
    
    // [\u4e00-\u9fa5]

    let timu = r#"

    switch (mc){
    
    case 1:
    
    "#;

    let r = regex::Regex::new(r#"(?m)[\u4e00-\u9fa5]"#).unwrap();

    let timu = r.captures_iter(timu).map(|c| c[0].to_owned()).collect::<String>();



    let (score, ti) = fuzzy_find(&list, &timu);
    println!("{:?}", (score, ti));

    // let c = Exam::init().await?;
    // if let Err(_) = c.load_cookies("./cookies").await {
    //     c.login().await?;
    // }
    // c.main().await?;
    Ok(())
}
