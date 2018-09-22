extern crate reqwest;
extern crate web_map;

use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Url;

// Test taken from reqwest crate examples
// https://docs.rs/reqwest/0.8.5/reqwest/struct.Response.html
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
    let url = Url::parse("http://www.pdx.edu");
    println!("URL = {:?}", url);
    match url {
        Err(_e) => assert!(false),
        Ok(u) => println!("{:?}", web_map::crawl(u).pop()),
    }
}

#[test]
fn parse_test() {
    let url = Url::parse("http://www.pdx.edu");
    println!("URL = {:?}", url);
    match url {
        Err(_e) => assert!(false),
        Ok(u) => {
            web_crawler::get_tag_srcs(web_crawler::crawl(u).pop().unwrap(), "img");
        }
    };

    assert!(false);
}
