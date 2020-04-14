extern crate model;
extern crate ui;

fn main() {
    //ui::main();
    println! {"{}", controller::Controller::new().run().to_string()}
}
