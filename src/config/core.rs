use std::path::PathBuf;

use crate::config::endpoint::Endpoint;

pub struct ConfigCore {
    peers: Vec<Endpoint>
}

impl ConfigCore {

    pub fn new(pathname: &str) -> ConfigCore {
        let p = vec![ Endpoint::new("127.0.0.1", 12000) ];
        ConfigCore {
            peers: p
        }
    }

    pub fn peers(&self) -> &Vec<Endpoint> {
        &self.peers
    }
}