use features::Context;
use handlers::github_handler::getReleases;

mod models;
mod features;
mod handlers;

fn main() {
    let _ = Context::load();
    let releases = getReleases("adoptium/jdk", None);

    println!("Hello, world! {}", releases.len());
}
