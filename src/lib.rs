mod error;
mod session;
mod chaoxin;
mod exam;
pub use chaoxin::CheckIn;
pub use error::Result;
pub use exam::Exam;

#[cfg(test)]
mod tests {
    use crate::chaoxin::CheckIn;

    #[tokio::test]
    async fn upload() {
        let mut chaoxin = CheckIn::load("./courses.json").await.unwrap();
        chaoxin.load_cookies("./cookies").await.unwrap_or_default();
        chaoxin.upload_image("up_img.jpg").await.unwrap();
    }
}