use crate::application::use_cases;


pub async fn send_message() {
    use_cases::send_message::execute().await;
}