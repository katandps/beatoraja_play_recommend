pub fn take() -> String {
    controller::Controller::local().run().to_string()
}
