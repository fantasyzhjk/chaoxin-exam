

use chaoxin_checkin::{CheckIn, Exam, tiku, Result};
use chrono::prelude::*;
use paris::*;
use tokio::time::{self, Duration};
use std::env;


#[tokio::main]
async fn main() -> Result<()> {
    // let mut c = CheckIn::load("./courses.json").await?;
    // c.load_cookies("./cookies").await.unwrap_or_default();
    // c.login().await?;
    // success!(
    //     "[{}] 已登录",
    //     Local::now().format("%Y-%m-%d %H:%M:%S")
    // );

    let mut c = Exam::init().await?;
    if let Err(_) = c.load_cookies("./cookies").await {
        c.login().await?;
    }

    if let Some(path) = env::args().skip(1).next() {
        c.load_tiku(path);
    }
    loop {
        c.main().await?;
    }
}
