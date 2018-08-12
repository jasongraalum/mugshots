// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This is a heavily modified version of the html5ever/examples/tokenizer.rs file.

extern crate html5ever;

use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken, StartTag, EndTag};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;

use reqwest;

#[derive(Clone)]
pub struct UrlTokenParser {
    pub in_char_run: bool,
    pub resources : Vec<String>,
    pub references : Vec<String>,
}

impl TokenSink for UrlTokenParser {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            TagToken(tag) => {
                for attr in tag.attrs.iter() {
                    if attr.name.local == "href".get(0..).unwrap().to_string(){
                        self.references.push(attr.value.get(0..).unwrap().to_string());
                    }
                    if attr.name.local.get(0..).unwrap().to_string() == "src" {
                        self.resources.push(attr.value.get(0..).unwrap().to_string());
                    }
                }
            }
            _ => {
                //println!("OTHER: {:?}", token);
            }
        }
        TokenSinkResult::Continue
    }
}

#[test]
fn test_tokenizer() {
    let mut sink = UrlTokenParser {
        in_char_run: false,
        resources : Vec::new(),
        references : Vec::new(),
    };
    let mut resp_text = reqwest::get("https://www.pdx.edu").unwrap().text().unwrap();

    let mut chunk = ByteTendril::new();
    chunk.try_push_bytes(resp_text.as_bytes()).unwrap();

    let mut input = BufferQueue::new();
    input.push_back(chunk.try_reinterpret().unwrap());

    let mut tok = Tokenizer::new(sink, TokenizerOpts {
        profile: true,
        .. Default::default()
    });

    let _ = tok.feed(&mut input);
    assert!(input.is_empty());

    tok.end();
    println!("References");
    println!("{:?}",tok.sink.references);
    println!("Resources");
    println!("{:?}",tok.sink.resources);
}

