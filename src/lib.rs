pub async fn take() -> String {
    controller::Controller::local()
        .run_async()
        .await
        .to_string()
}
pub use model::*;
