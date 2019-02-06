//! # WeTransfer

//! [WeTransfer](https://wetransfer.com) is a file transfer service specialized 
//! in sending large files via email. This crate acts as an unofficial Rust client
//! of their [public api](https://developers.wetransfer.com/documentation),
//! featuring  their two offered _products_:
// !
//! - **Transfers**,  a file sharing service which can transport up to 2GB. 
//! The service generates a publicly accessible link, from which the download 
//! can be started. Those files expire after seven days.
//!
//! - **Boards**, a service to store creative ideas into boards,
//! where both files and links can be pinned. These boards will expire
//! after three months of inactivity.
//! 
//! ## Usage
//! 
//! ```rust
//! extern crate wetransfer;
//! 
//! use wetransfer::requests::*;
//! use std::env;
//! 
//! fn main() {
//!     let app_token = env::var("APP_TOKEN").expect("Set APP_TOKEN env var.");
//!     let client = wetransfer::sync::Client::new(app_token).unwrap();
//!     let file_paths = vec!["/Users/sergio/Desktop/file.jpg"];
//!     
//!     // Create a transfer.
//!     let result_transfer = client.transfers.create("La Chuka.", &file_paths);
//!     println!("{:?}", result_transfer);
//! 
//!     // // Or create a board
//!     let board = client.boards.create("Title", Some("Description")).unwrap();
//!     println!("{:?}", board);
//!     
//!     // Add links to the board
//!     let links = vec![
//!         AddLink { 
//!             url: "https://wetransfer.com".to_string(),
//!             title: "Homepage".to_string()
//!         },
//!         AddLink {
//!             url: "https://github.com/tehAnswer".to_string(),
//!             title: "Sergio".to_string()
//!         }
//!     ];
//!     let result_links = client.boards.add_links(board.id.as_str(), &links);
//!     println!("{:?}", result_links);
//! 
//!     // Or add files.
//!     let result_files = client.boards.add_files(board.id.as_str(), &file_paths);
//!     println!("{:?}", result_files); 
//! }
//! ```
//! 
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[cfg(test)]
extern crate mockito;

/// Namespace for synchronous operations.
pub mod sync;
/// Namespace that declares struct types to model the `requests` payloads.
pub mod requests;
/// Namespace that declares struct types to model the `responses` from the APIs.
pub mod responses;
