pub fn take() -> String {
    controller::Controller::new().run().to_string()
}
