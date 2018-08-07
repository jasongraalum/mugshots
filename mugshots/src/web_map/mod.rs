// Copyright (c) 2018 Jason Graalum
//
//
// web_map library
//
// Defines a structure to reflect the hierarchical traits of a web site
//
// Starts at a root node
// Includes References - <a hrefs ...> for now
// Includes Data Sources - <src img ...> for now

use url::Url;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone,Eq,PartialEq)]
pub struct WebMap {
    hosts: Vec<(String,WebMapNode)>,
    ref_tag_attr_pairs: Vec<(String, String)>,
    src_tag_attr_pairs: Vec<(String, String)>,
}

#[derive(Clone,Eq,PartialEq)]
struct WebMapNode {
    path_segment : String,
    node : WebResource,
    valid: bool,
    hash: i32,
    children: Vec<WebMapNode>,
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


impl WebMap {
    // Create new web_map
    pub fn new() -> Self
    {
        WebMap { hosts: Vec::new(), ref_tag_attr_pairs: Vec::new(), src_tag_attr_pairs: Vec::new() }
    }

    // Insert a source name into the web_map
    pub fn insert_page(&self, page: Url) -> bool
    {

        // Check for existence
        // Add src into the hashmap
        // Add src into the treemap
        false
    }
}


#[test]
fn create_new_map()
{
    let new_url = Url::parse("https://www.pdx.edu");
    match new_url {
        Ok(u) => WebMap::new(),
        Err(_) => {}
    };
}


