//Copyright (c) 2018 Jason Graalum
// Using https://github.com/utkarshkukreti/select.rs

use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken, StartTag, EndTag};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;

use reqwest::{self,Url};

use std::collections::HashSet;

pub fn crawl(start_url: Url, list_of_urls: &mut HashSet<String>, depth: usize){

    println!("Crawling: {:?}", start_url);
    match get_tag_srcs(start_url,"href".to_string()) {
        Some(vec_of_href) => {
            println!("Vec = {:?}", vec_of_href);
            for href in vec_of_href {
                println!("HREF : {}", href);
                let new_url = Url::parse(href.as_str());
                match new_url {
                        Ok(u) => {
                            if list_of_urls.insert(href) && depth > 0 {
                                crawl(u, list_of_urls, depth - 1);
                            }
                        },
                    Err(_) => {},
                }
            }
        },
        None => {},
    }
}

pub fn get_tag_srcs(url: Url, tag: String) -> Option<Vec<String>> {

    let mut tag_src: Vec<String> = Vec::new();
    let resp  = reqwest::get(url);
    let mut resp_text: String;
    match resp {
        Ok(mut r) => {
            resp_text = match r.text() {
                Ok(t) => t,
                Err(_) => "Error in text".to_string(),
            };
        }
        Err(_) => resp_text = "Error in response".to_string(),
    }

    let mut sink = TokenPrinter {
        in_char_run: false,
        tag_name : tag,
        tok_src_vec: Vec::new(),
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

    //println!();
    //println!();
    //println!("IMG: {:?}",tok.sink.tok_src_vec);
    //assert!(input.is_empty());

    //tok.end();
    Some(tok.sink.tok_src_vec)
}


#[derive(Clone)]
struct TokenPrinter {
    in_char_run: bool,
    tok_src_vec: Vec<String>,
    tag_name: String,
}

impl TokenPrinter {
    fn is_char(&mut self, is_char: bool) {
        //match (self.in_char_run, is_char) {
        //    (false, true ) => print!("CHAR : \""),
        //    (true,  false) => println!("\""),
        //    _ => (),
        //}
        self.in_char_run = is_char;
    }

    fn do_char(&mut self, c: char) {
        self.is_char(true);
        //print!("{}", c.escape_default().collect::<String>());
    }
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(b) => {
                for c in b.chars() {
                    self.do_char(c);
                }
            }
            NullCharacterToken => self.do_char('\0'),
            TagToken(tag) => {
                self.is_char(false);
                for attr in tag.attrs.iter() {
                    if self.tag_name == attr.name.local.to_string() {
                        self.tok_src_vec.push(format!("{}",
                                                      attr.value));
                    }
                }
            }

            ParseError(err) => {
                self.is_char(false);
            }
            _ => {
                self.is_char(false);
            }
        }
        TokenSinkResult::Continue
    }
}
