//Copyright (c) 2018 Jason Graalum
// Using https://github.com/utkarshkukreti/select.rs

use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken, StartTag, EndTag};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;

use reqwest::{self,Url};

//pub fn crawl<A, F>(start_url : url::Url, mut f : F ) -> Result<A,&str>
pub fn crawl(start_url: Url) -> Vec<Url> {
    let mut all_urls = Vec::new();
    let resp = reqwest::get(start_url);
    match resp {
        Ok(r) => all_urls.push(r.url().clone()),
        Err(_) => {}
    }

    return all_urls;
}

pub fn get_tag_srcs(url: Url, tag: &str) -> Option<Vec<String>> {

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
    };

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
    None
}


#[derive(Copy, Clone)]
struct TokenPrinter {
    in_char_run: bool,
}

impl TokenPrinter {
    fn is_char(&mut self, is_char: bool) {
        match (self.in_char_run, is_char) {
            (false, true ) => print!("CHAR : \""),
            (true,  false) => println!("\""),
            _ => (),
        }
        self.in_char_run = is_char;
    }

    fn do_char(&mut self, c: char) {
        self.is_char(true);
        print!("{}", c.escape_default().collect::<String>());
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
                match tag.kind {
                    StartTag => print!("TAG  : <\x1b[32m{}\x1b[0m", tag.name),
                    EndTag   => print!("TAG  : <\x1b[31m/{}\x1b[0m", tag.name),
                }
                for attr in tag.attrs.iter() {
                    print!(" \x1b[36m{}\x1b[0m='\x1b[34m{}\x1b[0m'",
                           attr.name.local, attr.value);
                }
                if tag.self_closing {
                    print!(" \x1b[31m/\x1b[0m");
                }
                println!(">");
            }
            ParseError(err) => {
                self.is_char(false);
                println!("ERROR: {}", err);
            }
            _ => {
                self.is_char(false);
                println!("OTHER: {:?}", token);
            }
        }
        TokenSinkResult::Continue
    }
}

