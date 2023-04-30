# Public WeTransfer API Deprecated since 2022-05-31

> As of May 31, 2022, WeTransfer’s Public API is no longer available. 
> 
> Official anouncement: https://help.wetransfer.com/hc/en-us/articles/360038551772-We-ve-retired-our-Public-API

# WeTransfer

[WeTransfer](https://wetransfer.com) is a file transfer service specialized 
in sending large files via email. This crate acts as an unofficial Rust client
of their [public api](https://developers.wetransfer.com/documentation),
featuring their two offered _products_:

- **Transfers**,  a file sharing service which can transport up to 2GB. 
The service generates a publicly accessible link, from which the download 
can be started. Those files expire after seven days.
- **Boards**, a service to store creative ideas into boards,
where both files and links can be pinned. These boards will expire
after three months of inactivity.

## Install

First, add to your crate root the following line:

```
[dependencies]
wetransfer = "0.1.1"
```

Then, import the crate into your application by adding the following line at your project's root.

```rust
extern crate wetransfer;
```


## Usage

In this section, all the features offered by the crate are showcased.

```rust
extern crate wetransfer;

use wetransfer::requests::*;
use std::env;

fn main() {
    let app_token = env::var("APP_TOKEN").expect("Set APP_TOKEN env var.");
    let client = wetransfer::sync::Client::new(app_token).unwrap();
    let file_paths = vec!["/Users/sergio/Desktop/file.jpg"];
    
    // Create a transfer.
    let result_transfer = client.transfers.create("La Chuka.", &file_paths);
    println!("{:?}", result_transfer);

    // // Or create a board
    let board = client.boards.create("Title", Some("Description")).unwrap();
    println!("{:?}", board);
    
    // Add links to the board
    let links = vec![
        AddLink { 
            url: "https://wetransfer.com".to_string(),
            title: "Homepage".to_string()
        },
        AddLink {
            url: "https://github.com/tehAnswer".to_string(),
            title: "Sergio".to_string()
        }
    ];
    let result_links = client.boards.add_links(&board.id, &links);
    println!("{:?}", result_links);

    // Or add files.
    let result_files = client.boards.add_files(&board.id, &file_paths);
    println!("{:?}", result_files); 
}
```
