extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

use reqwest::{Body, RedirectPolicy, Response, IntoUrl, Method};
use reqwest::header::{Header, Headers, HeaderFormat};
use serde::ser::Serialize;
use std::time::Duration;

// TODO
pub type Result<T> = ::std::result::Result<T, reqwest::Error>;

/// A client providing the same interface as the reqwest::Client struct.
// TODO: Consider where we want to put `new()` as this will limit how flexible
// users of the library will be...
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
    fn send(self) -> Result<Response>;
}

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
        self.redirect(policy)
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
        self.body(body)
    }
    fn form<T: Serialize>(self, form: &T) -> Self {
        self.form(form)
    }
    fn json<T: Serialize>(self, json: &T) -> Self {
        self.json(json)
    }
    fn send(self) -> Result<Response> {
        self.send()
    }
}

pub mod replay;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
