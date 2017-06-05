extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

use reqwest::{Response, IntoUrl, Method};
use reqwest::header::{Header, Headers, HeaderFormat};
use serde::ser::Serialize;
use std::time::Duration;
use std::fs::File;
use std::io::{Cursor, Read};

/// A client providing the same interface as the reqwest::Client struct.
pub trait Client: Sized {
    type ReqBuilder: RequestBuilder;

    fn gzip(&mut self, enable: bool);

    fn redirect(&mut self, policy: RedirectPolicy);
    
    fn timeout(&mut self, timeout: Duration);
    
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::ReqBuilder;

    fn get<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Get, url)
    }
    fn post<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Post, url)
    }
    fn put<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Put, url)
    }
    fn patch<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Patch, url)
    }
    fn delete<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Delete, url)
    }
    fn head<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.request(Method::Head, url)
    }
}

pub trait RequestBuilder {
    fn header<H: Header + HeaderFormat>(self, header: H) -> Self;
    fn headers(self, headers: Headers) -> Self;
    fn basic_auth(self, username: String, password: Option<String>) -> Self;
    fn body<T: Into<Body>>(self, body: T) -> Self;
    fn form<T: Serialize>(self, form: &T) -> Self;
    fn json<T: Serialize>(self, json: &T) -> Self;
    fn send(self) -> Result<Response, reqwest::Error>;
}

/*
TODO: It's not possible to implement methods with the same name as a method which already exists
    on the struct.
    This is a problem we might have to solve using a new type, which is a bit ugly.

impl Client for reqwest::Client {
    type ReqBuilder = reqwest::RequestBuilder;

    /*
    fn new() -> Result<Self> {
        reqwest::Client::new()
    }
    */

    fn gzip(&mut self, enable: bool) {
        self.gzip(enable)
    }
    fn redirect(&mut self, policy: RedirectPolicy) {
        self.redirect(policy.to_reqwest_policy())
    }
    fn timeout(&mut self, timeout: Duration) {
        self.timeout(timeout)
    }
    fn get<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.get(url)
    }
    fn post<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.post(url)
    }
    fn put<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.put(url)
    }
    fn patch<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.patch(url)
    }
    fn delete<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.delete(url)
    }
    fn head<U: IntoUrl>(&self, url: U) -> Self::ReqBuilder {
        self.head(url)
    }
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::ReqBuilder {
        self.request(method, url)
    }
}

impl RequestBuilder for reqwest::RequestBuilder {
    fn header<H: Header + HeaderFormat>(self, header: H) -> Self {
        self.header(header)
    }
    fn headers(self, headers: Headers) -> Self {
        self.headers(headers)
    }
    fn basic_auth(self, username: String, password: Option<String>) -> Self {
        self.basic_auth(username, password)
    }
    fn body<T: Into<Body>>(self, body: T) -> Self {
        self.body(reqwest::Body::from(body.into()))
    }
    fn form<T: Serialize>(self, form: &T) -> Self {
        self.form(form)
    }
    fn json<T: Serialize>(self, json: &T) -> Self {
        self.json(json)
    }
    fn send(self) -> Result<Response, reqwest::Error> {
        self.send()
    }
}
*/

/// Specifies how to handle redirects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RedirectPolicy {
    Limit(usize),
    None
}

impl RedirectPolicy {
    /// Allow up to n redirects.
    pub fn limited(n: usize) -> Self {
        RedirectPolicy::Limit(n)
    }

    /// Allow no redirects.
    pub fn none() -> Self {
        RedirectPolicy::None
    }

    /// Convert to `reqwest::RedirectPolicy`.
    fn to_reqwest_policy(&self) -> reqwest::RedirectPolicy {
        use RedirectPolicy::*;
        match *self {
            Limit(n) => reqwest::RedirectPolicy::limited(n),
            None => reqwest::RedirectPolicy::none(),
        }
    }
}

impl From<RedirectPolicy> for reqwest::RedirectPolicy {
    fn from(r: RedirectPolicy) -> Self {
        r.to_reqwest_policy()
    }
}

impl Default for RedirectPolicy {
    fn default() -> Self {
        Self::limited(10)
    }
}

#[derive(Clone, Debug)]
pub struct Body {
    data: Vec<u8>,
}

impl From<Vec<u8>> for Body {
    fn from(f: Vec<u8>) -> Self {
        Body {
            data: f
        }
    }
}

impl From<String> for Body {
    fn from(f: String) -> Self {
        Body {
            data: f.bytes().collect()
        }
    }
}

impl<'a> From<&'a [u8]> for Body {
    fn from(f: &'a [u8]) -> Self {
        Body {
            data: f.to_vec()
        }
    }
}

impl<'a> From<&'a str> for Body {
    fn from(f: &'a str) -> Self {
        Body {
            data: f.bytes().collect()
        }
    }
}

impl From<Body> for ::reqwest::Body {
    fn from(b: Body) -> Self {
        reqwest::Body::new(Cursor::new(b.data.clone()))
    }
}

/* TODO
impl From<File> for Body {
    fn from(f: File) -> Self {
        Body {
            data: f.bytes().collect()
        }
    }
}
*/

pub mod replay;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
