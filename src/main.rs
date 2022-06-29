use chaoxin_checkin::{tiku, CheckIn, Exam, Result};
use chrono::prelude::*;
use paris::*;
use std::{env, io};
use tokio::time::{self, Duration};

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

    env::args().skip(1).next().map(|path| {
        c.load_tiku(&path);
        info!("已加载题库：{}", path);
    });

    loop {
        c.main().await?;
    }
}
