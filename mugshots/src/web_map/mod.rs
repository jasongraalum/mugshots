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
        WebMap { hosts: Vec::new(), resources : HashMap::new(), ref_tag_attr_pairs: Vec::new(), src_tag_attr_pairs: Vec::new() }
    }

    pub fn add_host(&mut self, hostname : &str) -> bool
    {
        match Url::parse(hostname) {
            Err(_) => false,
            Ok(url) => {
                let mut hostname_string = String::new();
                hostname_string.push_str(hostname);
                self.hosts.push((hostname_string, WebMapNode::new(url)));
                true
            },
        }
    }

    pub fn list_hosts(&self) -> Vec<String>
    {
        let mut host_list : Vec<String> = Vec::new();
        for &(ref h, ref n) in &self.hosts  {
            host_list.push(h.clone());
        }

        return host_list;
    }

    /*
    // Insert a source name into the web_map
    pub fn insert_page(&mut self, page_url: Url) -> bool
    {
        let mut hostname  = match page_url.host() {
            None => return false,
            Some(h) => match h {
                Host::Domain(host) => host,
                _ => return false,
            },
        };

        for &mut (host, node) in &mut self.hosts {
            if host == hostname {
                found_node = Some(&mut node);
            }
        }

        match found_node {
            None => {
                let mut hostname_string : String = "".to_string();
                hostname_string.push_str(hostname);
                hostname_string.push('/');
                match Url::parse(hostname_string.as_str()) {
                    Ok(root_url) => {
                        self.hosts.push((hostname_string, WebMapNode::new(root_url)));
                        return true;
                    },
                    Err(_err) => return false,
                }
            },
            Some(_) => self.add_node_path(&page_url),
        }

    }

    // Add a new set of webmapnodes to the webmap.  Add nodes from empty on down
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

}



#[derive(Clone,Eq,PartialEq)]
pub struct WebMapNode {
    path: String,
    resources_hashes : Vec<i32>,
    status: Option<StatusCode>,
    hash: i32,
    references: Vec<WebMapNode>,
}

impl WebMapNode {
    pub fn new(url : Url) -> WebMapNode{
        // GET Response
        let (references, resources) = process_url(url);

        let root_node = WebMapNode {path : url.path().to_string(), resources_hashes : Vec::new(), status : None, hash : 0, references: Vec::new()}
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
