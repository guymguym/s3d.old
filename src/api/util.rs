use crate::api::*;
use hyper::{http::request::Parts, Body, Request, Response};
use std::collections::HashMap;

pub type HttpRequest = Request<Body>;
pub type HttpResponse = Response<Body>;
pub type HttpResult = Result<HttpResponse, hyper::Error>;
pub type SyncError = Box<dyn std::error::Error + Send + Sync>;

pub trait ReqParser {
    fn parse(req: HttpRequest, bucket: &str, key: &str) -> Self;
}

pub trait ResWriter {
    fn write(self) -> HttpResponse;
}

pub trait RetWriter {
    fn write(self) -> HttpResult;
}

impl<T: ResWriter> RetWriter for Result<T, S3Error> {
    fn write(self) -> HttpResult {
        match self {
            Ok(res) => Ok(res.write()),
            Err(err) => Ok(err.write()),
        }
    }
}

pub struct BodyWriter {
    buf: Vec<u8>,
}

impl BodyWriter {
    pub fn new() -> Self {
        BodyWriter { buf: Vec::new() }
    }
    pub fn new_xml() -> Self {
        let mut w = Self::new();
        w.append("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        w
    }
    pub fn append(&mut self, s: &str) -> &mut Self {
        self.buf.extend_from_slice(s.as_bytes());
        self
    }
    pub fn append_xml(&mut self, tag: &str, content: &str) -> &mut Self {
        self.append(format!("<{0}>{1}</{0}>", tag, content).as_str())
    }
    pub fn _str(self) -> String {
        String::from_utf8(self.buf).unwrap()
    }
    pub fn body(self) -> Body {
        Body::from(self.buf)
    }
}

#[derive(Debug, Clone)]
pub struct QueryStr {
    map: HashMap<String, String>,
}

impl QueryStr {
    pub fn new(query: String) -> Self {
        let mut qs = QueryStr {
            map: HashMap::new(),
        };
        for q in query.split('&') {
            let kv: Vec<&str> = q.split('=').collect();
            qs.map.insert(
                kv.get(0).unwrap_or(&"").to_string(),
                kv.get(1).unwrap_or(&"").to_string(),
            );
        }
        qs
    }
    pub fn from_parts(parts: &Parts) -> Self {
        Self::new(parts.uri.query().unwrap_or("").to_string())
    }
    pub fn get(&self, key: &str) -> String {
        self.map.get(key).unwrap_or(&"".to_string()).to_string()
    }
    pub fn get_i32(&self, key: &str) -> i32 {
        self.get(key).parse::<i32>().unwrap_or(0)
    }
}
