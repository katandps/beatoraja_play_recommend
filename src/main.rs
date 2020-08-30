#[tokio::main]
pub async fn main() {
    println! {"{}", controller::Controller::local().await.run_async().await.to_string()};
}
