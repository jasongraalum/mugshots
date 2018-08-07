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
pub fn crawl(start_url: Url) ->  Result<WebMap, &'static str> {

    let webmap = WebMap::new();

    println!("Crawling: {:?}", start_url);

    let mut resp  = reqwest::get(start_url).unwrap();

    let mut resp_text : String = " ".to_string();
    match resp.status() {
        StatusCode::Ok => {
            resp_text = match resp.text() {
                Ok(t) => t,
                Err(e) => return Err("Error on text parse"),
            };
        },
        StatusCode::MovedPermanently => {},
        StatusCode::Forbidden => {},
        _ => {},
    }

    let sink = TokenParse {
        webmap: webmap,
        tag_name_attr_pairs: Vec::new(),
    };

    let mut chunk = ByteTendril::new();
    chunk.try_push_bytes(resp_text.as_bytes()).unwrap();

    let mut input = BufferQueue::new();
    input.push_back(chunk.try_reinterpret().unwrap());

    let mut tok = Tokenizer::new(sink, TokenizerOpts {
        profile: true,
        .. Default::default()
    });

    let _ =  tok.feed(&mut input);

    return Ok(tok.sink.webmap);
}

pub fn get_tag_srcs(url: Url, tag: String) -> Option<Vec<String>> {
    //let tag_src: Vec<String> = Vec::new();
    println!("Getting {} tags from {:?}", tag, url);

    /*
    */

    //Some(tok.sink.tok_src_vec)
    None
}


#[derive(Clone)]
struct TokenParse {
    webmap: WebMap,
    tag_name_attr_pairs: Vec<(String, String)>,
}


// Use the TokenSink Trait from the html5ever crate
impl TokenSink for TokenParse {
    type Handle = ();
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            TagToken(tag) => {
                for attr in tag.attrs.iter() {
                    let tag_name : String = tag.name.get(0..).unwrap().to_string();
                    let attr_name : String = attr.name.local.get(0..).unwrap().to_string();
                    let attr_val : String = attr.value.get(0..).unwrap().to_string();

                    if self.tag_name_attr_pairs.contains(&(tag_name, attr_name)) {
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
