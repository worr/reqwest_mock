use super::*;
use reqwest::Url;
use reqwest::header::{ContentType, Headers};
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;

mod data;

use self::data::{BasicAuth, RequestData};

#[derive(Debug)]
enum ClientMode {
    Record,
    Replay,
}

/// Specification of behavior in case of changed request data.
///
/// `hyper_replay` records both the incoming response data and the sent request data.
/// In the case you are not sending the same data anymore, as at the time of the original request,
/// this enum allows you to specify the wished behavior.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HandleChangedRequest {
    /// Ignores any changes in the request. If a replay file is present it will be used no matter
    /// what.
    Ignore,

    /// If changes in the request occur, a replay is promoted to a record and an actual network
    /// request will be sent and recorded.
    Record,

    /// If changes in the request occur, the replay will panic.
    Panic,
}

struct ReplayClientInner {
    mode: ClientMode,

    /// Here we record all request data specified by the API user regardless if we are recording or
    /// not, so in the case of a replay with changed request data we are capable of taking adequate
    /// measures.
    request_data: Rc<RefCell<RequestData>>,
}

pub struct ReplayClient {
    inner: Rc<RefCell<ReplayClientInner>>,
    request_data: Rc<RefCell<RequestData>>
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
            iclient: self.inner.clone(),
            data: Rc::new(RefCell::new(RequestData::default())),
            method: method,
            url: url.into_url().unwrap(),
        }
    }
}

pub struct ReplayRequestBuilder {
    iclient: Rc<RefCell<ReplayClientInner>>,
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
        self.data.borrow_mut().body = Some(body.into().data);
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

    fn send(self) -> Result<Response, reqwest::Error> {
        self.iclient.borrow_mut().send_request(&self)
    }
}

impl ReplayClientInner {
    fn send_request(&mut self, request: &ReplayRequestBuilder) -> Result<Response, reqwest::Error> {
        match self.mode {
            ClientMode::Record => {
                // TODO
                unimplemented!()
            },
            ClientMode::Replay => {
                // TODO
                unimplemented!()
            },
        }
    }
}

