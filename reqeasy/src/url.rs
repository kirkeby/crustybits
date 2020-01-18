//!
//! https://tools.ietf.org/html/rfc3986
//!

use crate::error::{Error, Result};
use simple_regex::Re;
use std::mem::replace;

#[derive(Debug)]
pub struct Url {
    scheme: String,
    host: String,
    path: String,
}

impl Url {
    pub fn parse(s: &str) -> Result<Self> {
        // FIXME - lazy_static?
        match Re::compile("^(http)://([^/]*)([^?]*)$").map(|re| re.search(s)) {
            Ok(Some(mut m)) => {
                let scheme = replace(&mut m.captured[0], String::new());
                let host = replace(&mut m.captured[1], String::new());
                let mut path = replace(&mut m.captured[2], String::new());
                if path.len() == 0 { path.push('/') }
                Ok(Url { scheme, host, path })
            }
            _ => Err(Error {}), // invalid URL
        }
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
