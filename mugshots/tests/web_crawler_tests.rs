extern crate reqwest;
extern crate mugshots;

use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Url;

use mugshots::web_crawler;

use std::collections::HashSet;

#[test]
fn test1() {
    let client = Client::new();
    let resp = client
        .post("http://httpbin.org/post")
        .body("possibly too large")
        .send();
    match resp {
        Err(e) => {
            println!("Some Error: {:?}", e);
            return;
        }
        Ok(r) => match r.status() {
            StatusCode::Ok => println!("success!"),
            StatusCode::PayloadTooLarge => {
                println!("Request payload is too large!");
            }
            s => println!("Received response status: {:?}", s),
        },
    }
}

#[test]
fn crawl_test1() {
    let mut url_list = HashSet::new();
    let url = Url::parse("http://www.pdx.edu").unwrap;
    let map = web_crawler::crawl(u);

}

