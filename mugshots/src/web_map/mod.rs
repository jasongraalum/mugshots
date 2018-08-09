// Copyright (c) 2018 Jason Graalum
// // // web_map library
//
// Defines a structure to reflect the hierarchical traits of a web site
//
// Starts at a root node
// Includes References - <a hrefs ...> for now
// Includes Data Sources - <src img ...> for now

pub mod tokenizer;

use std::io;
use std::default::Default;

use html5ever::tokenizer::{TokenSink, Tokenizer, Token, TokenizerOpts, ParseError, TokenSinkResult};
use html5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken, StartTag, EndTag};
use html5ever::tokenizer::BufferQueue;
use html5ever::tendril::*;

use url::{self, Url,Host};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use reqwest::{self,StatusCode};

use web_map::tokenizer::UrlTokenParser;

#[derive(Clone,Eq,PartialEq)]
pub struct WebMap {
    hosts: Vec<(String,u64)>,
    resources : HashMap<u64, WebResource>,
    references : HashMap<u64, WebMapNode>,
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

    pub fn add_node(&mut self, hostname: &str, node_url: &Url) -> u64 {
        let (status, resources, references) = self.process_url(&hostname, &node_url);

        match self.process_url(&hostname, &node_url) {
            (StatusCode::Ok, Some(res), Some(refs)) => {
                let new_node: WebMapNode = WebMapNode {
                    url : node_url.clone(),
                    status: Some(status),
                    resources :res,
                    references : refs,
                    children: Vec::new() };
                let mut hasher = DefaultHasher::new();
                let new_hash = new_node.hash(&mut hasher);
                let hash_val = hasher.finish();
                self.references.insert(hash_val, new_node);
                3
            },
            (StatusCode::Ok, None, None) => 1,
            _ => 2,
        }
    }

    pub fn process_url(&mut self, hostname : &str, url: &Url) -> (StatusCode, Option<Vec<u64>>, Option<Vec<u64>>)
    {
        let mut resp = reqwest::get(url.clone()).unwrap();

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
        //assert!(input.is_empty());

        //tok.end();
        //println!("References");
        //println!("{:?}",tok.sink.references);
        //println!("Resources");
        //println!("{:?}",tok.sink.resources);


        //(reshp.status(), Some(tok.sink.references), Some(tok.sink.resources))
        (resp.status(), None, None)
    }
}

#[derive(Clone,Eq,PartialEq)]
pub struct WebMapNode {
    url: Url,
    status: Option<StatusCode>,
    references: Vec<u64>,
    resources : Vec<u64>,
    children: Vec<u64>,
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
