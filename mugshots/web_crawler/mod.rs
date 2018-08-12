//Copyright (c) 2018 Jason Graalum
// Using https://github.com/utkarshkukreti/select.rs <- turned out to be less than stable
// Using html5ever and reqwest crates

use web_map::WebMap;

//use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;

use reqwest::{self,StatusCode};
use url::Url;


use std::collections::HashSet;

//
// crawl function takes a Url and a reference to a HashSet of String
// The function parses the response from tha GET to the URL,
// finds any href's and saves the target in the list_of_urls hash
// This should be combined with get_tag_srcs
//
pub fn crawl(start_url: Url, &mut web_map: WebMap) {

    let sink = TokenParse {
        webmap
    };

    println!("Crawling: {:?}", start_url);
    let url_path = start_url.path();

    if url_path.len() != 0 {
        let (file,path) = url_path.rsplitn(2, url_path);
        crawl(start_url: Url::parse(path),web_map);

        // Get response fro GET request
        let mut resp  = reqwest::get(start_url).unwrap();

        match resp.status() {
            StatusCode::Ok => {
                match resp.text() {
                    Ok(text) => {
                        let mut chunk = ByteTendril::new();
                        chunk.try_push_bytes(texgt.as_bytes()).unwrap();

                        let mut input = BufferQueue::new();
                        input.push_back(chunk.try_reinterpret().unwrap());

                        let mut tok = Tokenizer::new(sink, TokenizerOpts {
                            profile: true,
                            ..Default::default()
                        });

                        let _ = tok.feed(&mut input);
                    },
                    Err(e) => return Err("Error on text parse"),
                };
            },
            StatusCode::MovedPermanently => "",
            StatusCode::Forbidden => "",
            _ => {},
        }



    return Ok(tok.sink.webmap);
}


#[derive(Clone)]
struct TokenParse {
    webmap: WebMap,
}


// Use the TokenSink Trait from the html5ever crate
// Code template taken from html5ever example/tokenizer.rs
impl TokenSink for TokenParse {
    type Handle = ();
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            TagToken(tag) => {
                for attr in tag.attrs.iter() {
                    let tag_name : String = tag.name.get(0..).unwrap().to_string();
                    let attr_name : String = attr.name.local.get(0..).unwrap().to_string();
                    let attr_val : String = attr.value.get(0..).unwrap().to_string();

                    if self.webmap.ref_tag_attr_pairs.contains(&(tag_name, attr_name)) {
                        let new_url = Url::parse(&attr_val).unwrap();

                        self.webmap.insert_page(new_url);
                    }
                }
            }
            _ => {},
        }

        TokenSinkResult::Continue
    }
}
