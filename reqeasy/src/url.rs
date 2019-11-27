//!
//! https://tools.ietf.org/html/rfc3986
//!

use simple_regex::Re;

use crate::error::{Error, Result};

//FIXME - lazy_static

pub struct Url {
}

impl Url {
    pub fn parse(s: &str) -> Result<Self> {
        Re::compile("^(http)://([^/]*)([^?]*)$").map_err(|_| Error{})?.matches(s);
        Err(Error {})
    }

    pub fn scheme(&self) -> &str {
        unimplemented!();
    }

    pub fn host(&self) -> &str {
        unimplemented!();
    }

    pub fn path(&self) -> &str {
        unimplemented!();
    }
}
