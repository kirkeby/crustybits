//! FIXME

use crate::error::Result;

/// FIXME
//FIXME
#[allow(unused_variables,dead_code)]
pub struct RequestBuilder {
    url: String,
    method: String,
}

impl RequestBuilder {
    pub fn from_url(url: &str) -> Result<Self> {
        Ok(RequestBuilder { url: url.into(), method: "GET".into() })
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.into(); self
    }

    pub fn send(self) -> Result<Response> {
        unimplemented!()
    }
}

/// FIXME
pub struct Response {
    pub status_code: u16 //FIXME
}
