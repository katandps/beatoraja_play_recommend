pub fn main() {
    println! {"{}", controller::Controller::new().run().to_string()}
}
