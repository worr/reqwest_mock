use super::*;
use reqwest::Url;
use reqwest::header::ContentType;
use std::cell::RefCell;
use std::rc::Rc;

enum ClientMode {
    Record,
    Replay,
}

struct RequestData {
    /// apparently this is only about automatic decompression
    gzip: bool,
    redirect: RedirectPolicy,
    timeout: Option<Duration>,
    basic_auth: Option<BasicAuth>,
    headers: Headers,
    body: Option<Body>,
}

impl RequestData {
    fn default() -> Self {
        RequestData {
            gzip: true,
            redirect: RedirectPolicy::default(),
            timeout: None,
            basic_auth: None,
            headers: Headers::new(),
            body: None,
        }
    }
}

struct BasicAuth {
    username: String,
    password: Option<String>,
}

pub struct ReplayClient {
    mode: ClientMode,

    /// Here we record all request data specified by the API user regardless if we are recording or
    /// not, so in the case of a replay with changed request data we are capable of taking adequate
    /// measures.
    request_data: Rc<RefCell<RequestData>>,
}

impl Client for ReplayClient {
    type ReqBuilder = ReplayRequestBuilder;

    fn gzip(&mut self, enable: bool) {
        self.request_data.borrow_mut().gzip = enable;
    }

    fn redirect(&mut self, policy: RedirectPolicy) {
        self.request_data.borrow_mut().redirect = policy;
    }

    fn timeout(&mut self, timeout: Duration) {
        self.request_data.borrow_mut().timeout = Some(timeout);
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::ReqBuilder {
        ReplayRequestBuilder {
            data: Rc::new(RefCell::new(RequestData::default())),
            method: method,
            url: url.into_url().unwrap(),
        }
    }
}

pub struct ReplayRequestBuilder {
    data: Rc<RefCell<RequestData>>,
    method: Method,
    url: Url,
}

impl RequestBuilder for ReplayRequestBuilder {
    fn header<H: Header + HeaderFormat>(self, header: H) -> Self {
        self.data.borrow_mut().headers.set(header);
        self
    }
    fn headers(self, headers: Headers) -> Self {
        self.data.borrow_mut().headers.extend(headers.iter());
        self
    }
    fn basic_auth(self, username: String, password: Option<String>) -> Self {
        self.data.borrow_mut().basic_auth = Some(BasicAuth {
                                                     username: username,
                                                     password: password,
                                                 });
        self
    }

    fn body<T: Into<Body>>(self, body: T) -> Self {
        self.data.borrow_mut().body = Some(body.into());
        self
    }
    fn form<T: Serialize>(self, form: &T) -> Self {
        let body = serde_urlencoded::to_string(form).expect("serde urlencoded cannot fail");
        self.header(ContentType::form_url_encoded()).body(body)
    }
    fn json<T: Serialize>(self, json: &T) -> Self {
        let body = serde_json::to_vec(json).expect("serde to_vec cannot fail");
        self.header(ContentType::json()).body(body)
    }
    fn send(self) -> Result<Response> {
        unimplemented!()
    }
}
