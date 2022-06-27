use std::io::Cursor;
use std::path::{Path};

use crate::error::{ApiError, Result};
use crate::session::{CookieJar, Session};
use qrcode::QrCode;
use qrcode::render::unicode;
use lazy_static::lazy_static;
use reqwest::multipart;
use reqwest::Body;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt};
use std::io;
use paris::*;
use tokio_util::codec::{BytesCodec, FramedRead};

// const MOBILE_USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 10; MI 8 Build/QKQ1.190828.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/83.0.4103.101 Mobile Safari/537.36";
const DESKTOP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36 Edg/97.0.1072.69";
const CHAOXIN_USER_AGENT: &str = "Dalvik/2.1.0 (Linux; U; Android 10; MI 8 MIUI/20.2.27) com.chaoxing.mobile/ChaoXingStudy_3_4.3.6_android_phone_496_27 (@Kalimdor)_994222229cb94d688c462b7257600006";
const HOME_URL: &str = "http://i.mooc.chaoxing.com/space/";
const MOBILE_ORIGIN: &str = "https://mobilelearn.chaoxing.com";
const PASSPORT_ORIGIN: &str = "http://passport2.chaoxing.com";
const LOGIN_PATH: &str = "cloudscanlogin?mobiletip=%e7%94%b5%e8%84%91%e7%ab%af%e7%99%bb%e5%bd%95%e7%a1%ae%e8%ae%a4&pcrefer=http://i.chaoxing.com";

lazy_static! {
    static ref LOGIN_HEADERS: Vec<(&'static str, &'static str)> = vec![
        ("User-Agent", DESKTOP_USER_AGENT),
        ("Upgrade-Insecure-Requests", "1"),
        ("Host", "passport2.chaoxing.com"),
        (
            "Referer",
            "https://passport2.chaoxing.com/login?fid=2182&refer=http://i.mooc.chaoxing.com",
        ),
        ("Origin", PASSPORT_ORIGIN),
    ];
    static ref MOBILE_HEADER: Vec<(&'static str, &'static str)> = vec![
        ("User-Agent", CHAOXIN_USER_AGENT),
        ("Host", "pan-yz.chaoxing.com")
    ];
}

/// 用正则匹配内容，然后取组1的第一个结果
#[macro_export]
macro_rules! regex_find {
    ($text: expr, $pattern: expr) => {{
        let re = regex::Regex::new($pattern).unwrap();
        re.captures($text).map(|r| r[1].to_string())
    }};
}


#[macro_export]
macro_rules! document_find_text {
    ($document: expr, $pattern: expr) => {{
        $document
            .select(&Selector::parse($pattern).unwrap())
            .next()
            .map(|e| e.text().collect())
    }};
}

#[macro_export]
macro_rules! document_find_attr {
    ($document: expr, $pattern: expr, $attr: expr) => {{
        $document
            .select(&Selector::parse($pattern).unwrap())
            .next()
            .map(|e| e.value().attr($attr).unwrap_or_default())
    }};
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CourseInfo {
    pub url: String,
    pub name: String,
}

#[derive(Clone)]
pub struct CheckIn {
    pub session: Session,
    name: Option<String>,
    course_list: Vec<CourseInfo>,
}

impl CheckIn {
    pub async fn load(course_path: &str) -> Result<Self> {
        let s = Self {
            session: Session::new(true)?,
            name: None,
            course_list: serde_json::from_str(&fs::read_to_string(course_path).await?)?,
        };
        Ok(s)
    }
    pub fn courses(&self) -> Vec<CourseInfo> {
        self.course_list.clone()
    }
    pub async fn load_cookies(&mut self, path: &str) -> Result<()> {
        self.session.cookie_jar = CookieJar::from_str(&fs::read_to_string(path).await?);
        Ok(())
    }
    pub async fn login(&mut self) -> Result<()> {
        if !self.check_is_login().await? {
            info!("开始登录");
            let res = self
                .session
                .get(
                    &format!("{}/{}", PASSPORT_ORIGIN, LOGIN_PATH),
                    Some(&LOGIN_HEADERS),
                )
                .await?
                .text()
                .await?;
            let document = Html::parse_document(&res);
            let uuid = regex_find!(
                &res,
                r#"<input type = "hidden" value="(.*?)" id = "uuid"/>"#
            )
            .unwrap();
            let enc = regex_find!(&res, r#"<input type = "hidden" value="(.*?)" id = "enc"/>"#).unwrap();
            let image_url = format!(
                "{}{}",
                PASSPORT_ORIGIN,
                document_find_attr!(&document, r#"img[id="ewm"]"#, "src").unwrap()
            );
            // let image_url = format!("{}/createqr?uuid={}&xxtrefer=&type=1&mobiletip=%E7%94%B5%E8%84%91%E7%AB%AF%E7%99%BB%E5%BD%95%E7%A1%AE%E8%AE%A4", PASSPORT_ORIGIN, uuid);

            success!("二维码地址获取成功..\n {}", &image_url);
            let res = self.session.get(&image_url, Some(&LOGIN_HEADERS)).await?;
            let img = image::load(Cursor::new(res.bytes().await?), image::ImageFormat::Png)?.to_luma8();
            let mut img = rqrr::PreparedImage::prepare(img);
            let grids = img.detect_grids();
            assert_eq!(grids.len(), 1);
            // Decode the grid
            let (_, content) = grids[0].decode()?;
            let image  = QrCode::new(content)?
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            log!("{}", image);
            warn!("登录后请按回车继续 ↲");
            io::stdin().read_line(&mut String::new())?;
            self.check_login_status(&uuid, &enc).await?;
            let mut f = File::create("./cookies").await?;
            f.write_all(self.session.cookie_jar.to_string().unwrap().as_bytes())
                .await?;
        }
        self.name = Some(self.get_person_name().await?);
        Ok(())
    }

    async fn check_login_status(&mut self, uuid: &str, enc: &str) -> Result<()> {
        let res = self
            .session
            .post(
                &format!("{}/getauthstatus", PASSPORT_ORIGIN),
                Some(&[("uuid", uuid), ("enc", enc)]),
                Some(&LOGIN_HEADERS),
            )
            .await?;
        let j = res.json::<serde_json::Value>().await?;
        if let Some(status) = j["status"].as_bool() {
            if status {
                Ok(())
            } else {
                Err(ApiError::custom(1, j["mes"].as_str().unwrap()))
            }
        } else {
            Err(ApiError::custom(1, "登陆失败"))
        }
    }
    async fn get_person_name(&mut self) -> Result<String> {
        let res = self.session.get(HOME_URL, None).await?.text().await?;
        let document = Html::parse_document(&res);
        let title: String = document_find_text!(&document, r#"p[class="personalName"]"#).unwrap();
        Ok(title)
    }

    async fn check_is_login(&mut self) -> Result<bool> {
        let res = self.session.get(HOME_URL, None).await?.text().await?;
        let document = Html::parse_document(&res);
        let title: String = document_find_text!(&document, "title").unwrap_or_default();
        Ok(title != "用户登录")
    }

    pub async fn check_in_course(&mut self, course: &CourseInfo) -> Result<()> {
        let res = self
            .session
            .get(&course.url, None)
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&res);
        let title: String = document_find_text!(&document, "title").unwrap();
        if title == "提示信息" {
            return Err(ApiError::custom(1, "检查课程失败"))
        }
        let uid = document_find_attr!(&document, r#"input[id="puid"]"#, "value").unwrap();
        let course_id = document_find_attr!(&document, r#"input[id="courseId"]"#, "value").unwrap();
        let class_id = document_find_attr!(&document, r#"input[id="classId"]"#, "value").unwrap();
        let fid = document_find_attr!(&document, r#"input[id="fid"]"#, "value").unwrap();

        let start_list = document
            .select(&Selector::parse(r#"div[id="startList"]"#).unwrap())
            .next()
            .unwrap();

        for e in start_list.select(&Selector::parse(r#"div[class="Mct"]"#).unwrap()) {
            let v = e.value().attr("onclick").unwrap();
            let active_type = regex_find!(v, r#"activeDetail\(.*?,(\d+),.*?\)"#).unwrap();
            if active_type == "2" {
                let active_id = regex_find!(v, r#"activeDetail\((\d+),.*?,.*?\)"#).unwrap();
                match self.check(&active_id, course_id, class_id, fid, uid).await {
                    Ok(r) => {
                        if r { log!("<green> ({}) 签到成功", active_id) }
                    },
                    Err(_) => {
                        log!("<red> ({}) 签到失败", active_id);
                    },
                }
            }
        }
        Ok(())
    }

    async fn check(
        &mut self,
        active_id: &str,
        course_id: &str,
        class_id: &str,
        fid: &str,
        uid: &str,
    ) -> Result<bool> {
        let res = self
            .session
            .get(
                &format!(
                    "{}/widget/sign/pcStuSignController/preSign?activeId={}&classId={}&fid={}&courseId={}",
                    MOBILE_ORIGIN,
                    active_id,
                    class_id,
                    fid,
                    course_id
                ),
                None,
            )
            .await?
            .text()
            .await?;

        if let Some(_) = res.find("签到成功") {
            Ok(true)
        } else if let Some(_) = res.find("手势图案") {
            self.hand_check(active_id, course_id, class_id).await
        } else if let Some(_) = res.find("手机扫码") {
            self.qrcode_check(active_id, uid, fid).await
        } else if let Some(_) = res.find("位置信息") {
            self.addr_check("扬州", "-1", "-1", active_id, uid, fid).await
        } else if let Some(_) = res.find("手机拍照") {
            self.photo_check("up_img.jpg", active_id, uid, fid).await
        } else {
            Ok(false)
        }
    }

    async fn hand_check(
        &mut self,
        active_id: &str,
        course_id: &str,
        class_id: &str,
    ) -> Result<bool> {
        let res = self
            .session
            .get(
                &format!(
                    "{}/widget/sign/pcStuSignController/signIn?activeId={}&classId={}&courseId={}",
                    MOBILE_ORIGIN, active_id, class_id, course_id
                ),
                None,
            )
            .await?
            .text()
            .await?;

        match res.find("签到成功") {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn qrcode_check(&mut self, active_id: &str, uid: &str, fid: &str) -> Result<bool> {
        let res = self
            .session
            .get(
                &format!(
                    "{}/pptSign/stuSignajax?name={}&activeId={}&uid={}&fid={}&clientip=&useragent=&latitude=-1&appType=15",
                    MOBILE_ORIGIN,
                    self.name.clone().unwrap(),
                    active_id,
                    uid,
                    fid
                ),
                None,
            )
            .await?
            .text()
            .await?;

        match res.find("签到成功") {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn addr_check(
        &mut self,
        addr: &str,
        latitude: &str,
        longitude: &str,
        active_id: &str,
        uid: &str,
        fid: &str,
    ) -> Result<bool> {
        // let addr = "扬州";
        // let latitude = "-1"; // 纬度
        // let longitude = "-1"; // 经度

        let res = self
            .session
            .get(
                &format!(
                    "{}/pptSign/stuSignajax?name={}&address={}&latitude={}&longitude={}&activeId={}&uid={}&fid={}&appType=15&ifTiJiao=1&clientip=",
                    MOBILE_ORIGIN,
                    self.name.clone().unwrap(),
                    addr,
                    latitude,
                    longitude,
                    active_id,
                    uid,
                    fid,
                ),
                None,
            )
            .await?
            .text()
            .await?;

        match res.find("签到成功") {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn photo_check(&mut self, image_path: impl AsRef<Path>, active_id: &str, uid: &str, fid: &str) -> Result<bool> {
        let object_id = self.upload_image(image_path).await?;
        let res = self
            .session
            .get(
                &format!(
                    "{}/pptSign/stuSignajax?name={}&activeId={}&uid={}&fid={}&objectId={}&clientip=&useragent=&latitude=-1&appType=15",
                    MOBILE_ORIGIN,
                    self.name.clone().unwrap(),
                    active_id,
                    uid,
                    fid,
                    object_id
                ),
                None,
            )
            .await?
            .text()
            .await?;

        match res.find("签到成功") {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn upload_image(&mut self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref().to_owned();
        let file_name = path
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned());
        let file = File::open(path).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        let form = multipart::Form::new().text("puid", "80421235").part(
            "file",
            multipart::Part::stream(body).file_name(file_name.unwrap()),
        );
        let res = self
            .session
            .multipart(
                "https://pan-yz.chaoxing.com/upload?_token=5d2e8d0aaa92e3701398035f530c4155",
                form,
                Some(&MOBILE_HEADER),
            )
            .await?
            .json::<Value>()
            .await?;

        Ok(res["objectId"].to_string())
    }
}
