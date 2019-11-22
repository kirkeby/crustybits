//! FIXME

use crate::error::Result;

use std::io::prelude::*;

/// FIXME
pub enum Protocol { HTTP, HTTPS }

/// FIXME
//FIXME
#[allow(unused_variables,dead_code)]
pub struct RequestBuilder {
    method: String,
    protocol: Protocol,
    host: String,
    path: String,
}

impl RequestBuilder {
    // FIXME remove unwraps
    pub fn from_url(url: &str) -> Result<Self> {
        let url = ::url::Url::parse(url)?;
        // FIXME remove asserts; check other fields
        assert!(url.scheme() == "http");
        assert!(url.query().is_none());
        Ok(RequestBuilder {
            method: "GET".into(),
            protocol: Protocol::HTTP,
            host: url.host_str().unwrap().into(),
            path: url.path().into(),
        })
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.into(); self
    }

    pub fn send(self) -> Result<Response> {
        let mut c = ::std::net::TcpStream::connect((self.host.as_ref(), 80u16))?;
        c.write(format!(
            "GET {} HTTP/1.0\r\nHost: {}\r\n\r\n",
            self.path, self.host,
        ).as_ref())?;
        c.shutdown(::std::net::Shutdown::Write)?;
        Response::read_from(&mut ::std::io::BufReader::new(c))
    }
}

/// FIXME
pub struct Response {
    pub status_code: u16 //FIXME
}

impl Response {
    fn read_from<B: ::std::io::BufRead>(c: &mut B) -> Result<Self> {
        let mut line = String::new();
        c.read_line(&mut line)?;
        dbg!(&line);
        let status = line.split(' ').nth(1).unwrap().parse().unwrap();
        Ok(Response { status_code: status })
    }
}
