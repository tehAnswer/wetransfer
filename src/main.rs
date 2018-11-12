extern crate wetransfer;
use std::env;

fn main() {
    let app_token = env::var("APP_TOKEN").expect("Set APP_TOKEN env var.");
    let client = wetransfer::sync::Client::new(app_token);
    println!("{:?}", client);
}
