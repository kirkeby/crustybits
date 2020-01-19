//! FIXME

use crate::error::Result;
use crate::url::Url;

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
        let url = Url::parse(url)?;
        // FIXME remove asserts; check other fields
        assert!(url.scheme() == "http");
        Ok(RequestBuilder {
            method: "GET".into(),
            protocol: Protocol::HTTP,
            host: url.host().into(),
            path: url.path().into(),
        })
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.into(); self
    }

    pub fn send(self) -> Result<Response> {
        let mut c = std::net::TcpStream::connect((self.host.as_ref(), 80u16))?;
        c.write_all(format!(
            "GET {} HTTP/1.0\r\nHost: {}\r\n\r\n",
            self.path, self.host,
        ).as_ref())?;
        c.shutdown(std::net::Shutdown::Write)?;
        Response::read_from(&mut std::io::BufReader::new(c))
    }
}

/// HTTP request or response headers.
// FIXME: need a multi-valued map here
pub type Headers = std::collections::HashMap<String, String>;

/// FIXME
pub type StatusCode = u16;

/// FIXME
pub struct Response {
    pub status_code: StatusCode,
    pub headers: Headers,
}

impl Response {
    fn read_from<B: BufRead>(c: &mut B) -> Result<Self> {
        let status_code = Response::read_status_line(c)?;
        let headers = Response::read_headers(c)?;
        Ok(Response {
            status_code,
            headers,
        })
    }

    fn read_status_line<B: BufRead>(c: &mut B) -> Result<StatusCode> {
        let mut line = String::new();
        c.read_line(&mut line)?;
        let pieces = line.splitn(3, ' ').collect::<Vec<_>>();
        // FIXME - no assert!
        assert!(pieces.len() == 3);
        assert!(pieces[0].starts_with("HTTP/1."));
        pieces[1].parse().map_err(|_| crate::error::Error {})
    }

    fn read_headers<B: BufRead>(c: &mut B) -> Result<Headers> {
        let mut headers = std::collections::HashMap::new();

        loop {
            let mut line = String::new();
            c.read_line(&mut line)?;
            if line.trim().is_empty() {
                break;
            }
            // FIXME continuation lines

            let pieces = line.splitn(2, ':').collect::<Vec<_>>();
            assert!(pieces.len() == 2);
            headers.insert(
                pieces[0].trim().into(),
                pieces[1].trim().into());
        }

        Ok(headers)
    }
}


/// FIXME
pub fn get(url: &str) -> Result<Response> {
    RequestBuilder::from_url(url)?.method("GET").send()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_google() -> Result<()> {
        let r = get("http://google.com").expect("get no www");
        assert_eq!(r.status_code, 301);
        assert_eq!(r.headers["Location"], "http://www.google.com/");

        let r = get("http://www.google.com/")?;
        assert_eq!(r.status_code, 200);
        assert_eq!(r.headers["Content-Type"], "text/html; charset=ISO-8859-1");

        Ok(())
    }
}
