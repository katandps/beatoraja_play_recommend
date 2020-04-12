extern crate model;
extern crate ui;

fn main() {
    //ui::main();
    println! {"{}", model::run(model::Controller::new())}
}
