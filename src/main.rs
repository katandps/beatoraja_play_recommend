pub fn main() {
    println! {"{}", controller::Controller::local().run().to_string()}
}
