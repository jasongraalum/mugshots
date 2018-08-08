// Copyright (c) 2018 Jason Graalum
// //
// web_map library
//
// Defines a structure to reflect the hierarchical traits of a web site
//
// Starts at a root node
// Includes References - <a hrefs ...> for now
// Includes Data Sources - <src img ...> for now

use url::{self, Url,Host};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use reqwest::{self,StatusCode};

//use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;



#[derive(Clone,Eq,PartialEq)]
pub struct WebMap {
    hosts: Vec<(String,i32)>,
    resources : HashMap<i32, WebResource>,
    references : HashMap<i32, WebMapNode>,
    ref_tag_attr_pairs: Vec<(String, String)>,
    src_tag_attr_pairs: Vec<(String, String)>,
}

impl WebMap {
    // Create new web_map
    pub fn new() -> WebMap
    {
        WebMap { hosts: Vec::new(), references: HashMap::new(), resources: HashMap::new(), ref_tag_attr_pairs: Vec::new(), src_tag_attr_pairs: Vec::new() }
    }

    pub fn add_host(&mut self, hostname: &str) -> bool
    {
        match Url::parse(hostname) {
            Err(_) => false,
            Ok(url) => {
                let mut hostname_string = String::new();
                hostname_string.push_str(hostname);
                let node_hash = self.add_node(&hostname, &url);
                self.hosts.push((hostname_string, node_hash));
                true
            },
        }
    }

    pub fn list_hosts(&self) -> Vec<String>
    {
        let mut host_list: Vec<String> = Vec::new();
        for &(ref h, ref n) in &self.hosts {
            host_list.push(h.clone());
        }

        return host_list;
    }

    pub fn add_node(&mut self, hostname: &str, node_url: &Url) -> i32 {
        let (status, resources, references) = self.process_url(&mut self, &hostname, &node_url);

        let new_node: WebMapNode = WebMapNode {
            url : node_url.clone(),
            status,
            references,
            resources,
            children: Vec::new()
        };
        0
    }

    pub fn process_url(&mut self, hostname : &str, url: &Url) -> (StatusCode, Option<Vec<i32>>, Option<Vec<i32>>)
    {
        let token_output : (Vec<i32>, Vec<i32>);
        let sink = TokenParse {
            token_struct: token_output
        };

        let mut resp = reqwest::get(url).unwrap();

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
            }
        }
        (resp.status(), None, None)
    }
}

/*
    fn add_node_path(&mut self, page_url: &Url ) -> bool
    {
        // Break the page_url into different levels
        // Url: /a/b/c => /a, /a/b, /a/b/c
        let mut node_level : String = "/".to_string();
        let mut levels = Vec::new();
        let segments = page_url.path_segments();
        match segments {
            Some(seg_set) => {
                for s in seg_set {
                    node_level.push('/');
                    node_level.push_str(s);
                    levels.push(node_level);
                }
            }
            None => return false,
        }
        // Remove levels which already exist in the WebMap
        false
    }
    */


#[derive(Clone)]
struct TokenParse {
    token_struct : (Vec<i32>,Vec<i32>)
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


#[derive(Clone,Eq,PartialEq)]
pub struct WebMapNode {
    url: Url,
    status: Option<StatusCode>,
    references: Vec<i32>,
    resources : Vec<i32>,
    children: Vec<i32>,
}

impl Hash for WebMapNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let host = self.url.host_str();
        let path = self.url.path();
        host.hash(state);
        path.hash(state);
        self.status.hash(state);
    }
}

impl WebMapNode {
    pub fn new(url : Url) -> WebMapNode {
        // GET Response
        let root_node = WebMapNode {url : url,
            references : Vec::new(),
            resources: Vec::new(),
            status : None,
            children: Vec::new()};

        root_node
    }
}


#[derive(Clone,PartialEq, Eq)]
struct WebResource {
    url: Url,
    resource_type: String,
}

impl Hash for WebResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let host = self.url.host_str();
        let path = self.url.path();
        host.hash(state);
        path.hash(state);
        self.resource_type.hash(state);
    }
}


#[test]
fn webmap_add_new_host()
{
    let mut map = WebMap::new();
    if map.add_host("https://www.pdx.edu") == true  &&
        map.add_host("https://www.google.com") == true  {
        for h in map.list_hosts() {
            println!("Host : {:?}", h);
        }
        assert!(true);
    }
        else {
            assert!(false);
        }
}
