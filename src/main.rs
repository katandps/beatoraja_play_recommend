extern crate model;
extern crate ui;

fn main() {
    //ui::main();
    println! {"{}", model::Controller::new().run().to_string()}
}
