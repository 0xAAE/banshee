//use std::path::PathBuf;
// config the only module that is init prior to logger, so the logger won't work here
//use log::info;

use crate::config::endpoint::Endpoint;

pub struct ConfigCore {
    peers: Vec<Endpoint>
}

impl ConfigCore {

    pub fn new(_pathname: &str) -> ConfigCore {
        let p = vec![ Endpoint::new("127.0.0.1", 12000) ];
        ConfigCore {
            peers: p
        }
    }

    pub fn peers(&self) -> &Vec<Endpoint> {
        &self.peers
    }
}