use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Endpoint {
    addr: String,
    port: u16
}

impl Endpoint {

    pub fn new(addr: &str, port: u16) -> Endpoint {
        Endpoint {
            addr: addr.to_string(),
            port: port
        }
    }

    fn addr(&self) -> &str {
        &self.addr
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl Display for Endpoint {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{}", self.addr(), self.port())
    }

}
