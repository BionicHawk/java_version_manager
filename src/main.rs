use features::Context;

mod models;
mod features;

fn main() {
    let _ = Context::load();
    println!("Hello, world!");
}
