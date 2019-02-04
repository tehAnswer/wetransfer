extern crate wetransfer;
use wetransfer::responses::Transfer;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let app_token = env::var("APP_TOKEN").expect("Set APP_TOKEN env var.");
    let client = wetransfer::sync::Client::new(app_token).unwrap();
    // let response = client.transfers.create("La Chuka, uploaded in Rust.", &vec!["/Users/sergio/Desktop/chuka.jpg"]);
    let response = client.boards.create("xd", None);
    println!("{:?}", response);
    let board = response.unwrap();
    let files: Vec<String> = vec![String::from("/Users/sergio/Desktop/an9zg4z_460s.jpg"), String::from("/Users/sergio/Desktop/ax1pm5p_460s.jpg"), String::from("/Users/sergio/Desktop/a3Q7pNv_460s.jpg")];
    let response2 = client.boards.add_files(board.id, &files);
    println!("{:?}", response2);
}
