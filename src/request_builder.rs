use body::IntoBody;
use client::Client;
use reqwest::{IntoUrl, Url, Method};
use request::Request;
use response::Response;
use reqwest::header::{Headers, Header};
use error::{Error, ResultExt};

pub struct RequestBuilder<'cl, Cl: Client + 'cl> {
    client: &'cl Cl,

    url: Result<Url, Error>,
    method: Method,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl<'cl, Cl: Client + 'cl> RequestBuilder<'cl, Cl> {
    #[doc(hidden)]
    pub fn new<U: IntoUrl>(client: &'cl Cl, url: U, method: Method) -> Self {
        RequestBuilder {
            client: client,
            url: url.into_url().chain_err(|| "invalid url"),
            method: method,
            headers: Headers::new(),
            body: None,
        }
    }

    /// Add a header to the request.
    pub fn header<H: Header>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }

    /// Add multiple headers to the request.
    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers.extend(headers.iter());
        self
    }

    /// Set the body of the request.
    pub fn body<B: IntoBody>(mut self, body: B) -> Self {
        self.body = Some(body.into_body());
        self
    }

    /// Send the request.
    pub fn send(self) -> Result<Response, Error> {
        let request = Request {
            url: self.url?,
            method: self.method,
            headers: self.headers,
            body: self.body,
        };

        self.client.execute(None, request)
    }
}
