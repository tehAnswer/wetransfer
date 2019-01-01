extern crate wetransfer;
use std::env;

fn main() {
    let app_token = env::var("APP_TOKEN").expect("Set APP_TOKEN env var.");
    let client = wetransfer::sync::Client::new(app_token).unwrap();
    // let response = client.transfers.create("Puta bida", ["/Users/sergio/Desktop/screen.png"])
    let response1 = client.transfers.create_transfer_request("Puta bida", vec!["/Users/sergio/Desktop/screen.png"]).unwrap();
    println!("{:?}", response1);
    let response2 = client.transfers.upload_url_for(response1.id, response1.files[0].id.to_string(), 1);
    println!("{:?}", response2);
}
