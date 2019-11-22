//! FIXME
//!

pub mod client;
pub mod error;
pub mod http;

use client::{Response, RequestBuilder};
use error::Result;


/// FIXME
pub fn get(url: &str) -> Result<Response> {
    RequestBuilder::from_url(url)?.method("GET").send()
}


#[test]
fn test_get_google() {
    assert!(get("http://google.com").unwrap().status_code == 200);
}
