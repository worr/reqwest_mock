use super::*;
use reqwest::Url;
use reqwest::header::{ContentType, Headers};
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;

mod data;
use self::data::{BasicAuth, ClientData, ResponseData, RequestData};

mod storage;
use self::storage::{ReplayData, ReplayFile};

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

struct InnerClient {
    mode: ClientMode,
    file: ReplayFile,
}

pub struct ReplayClient {
    inner: Rc<RefCell<InnerClient>>,
    data: ClientData,
}

impl Client for ReplayClient {
    type ReqBuilder = ReplayRequestBuilder;

    fn gzip(&mut self, enable: bool) {
        self.data.gzip = enable;
    }

    fn redirect(&mut self, policy: RedirectPolicy) {
        self.data.redirect = policy;
    }

    fn timeout(&mut self, timeout: Duration) {
        self.data.timeout = Some(timeout);
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::ReqBuilder {
        ReplayRequestBuilder {
            data: RequestData::new_for_client(&self.data),
            inner: self.inner.clone(),
            method: method,
            url: url.into_url().unwrap(),
        }
    }
}

pub struct ReplayRequestBuilder {
    inner: Rc<RefCell<InnerClient>>,
    method: Method,
    url: Url,
    data: RequestData,
}

impl RequestBuilder for ReplayRequestBuilder {
    fn header<H: Header + HeaderFormat>(mut self, header: H) -> Self {
        self.data.headers.set(header);
        self
    }

    fn headers(mut self, headers: Headers) -> Self {
        self.data.headers.extend(headers.iter());
        self
    }

    fn basic_auth(mut self, username: String, password: Option<String>) -> Self {
        self.data.basic_auth = Some(BasicAuth {
                                        username: username,
                                        password: password,
                                    });
        self
    }

    fn body<T: Into<Body>>(mut self, body: T) -> Self {
        self.data.body = Some(body.into().data);
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
        self.inner.borrow_mut().send_request(self.data)
    }
}

impl InnerClient {
    fn send_request(&mut self, request_data: RequestData) -> Result<Response, reqwest::Error> {
        match self.mode {
            ClientMode::Record => {
                // Perform the request.
                let mut client = reqwest::Client::new()?;
                client.gzip(request_data.gzip);
                client.redirect(request_data.redirect.to_reqwest_policy());
                if let Some(timeout) = request_data.timeout {
                    client.timeout(timeout);
                }

                let mut target = request_data.target.clone().unwrap();
                let mut req = client
                    .request(target.method().clone(), target.url().clone())
                    .headers(request_data.headers.to_reqwest_headers());
                if let Some(auth) = request_data.basic_auth.clone() {
                    req = req.basic_auth(auth.username, auth.password);
                }
                if let Some(body) = request_data.body.clone() {
                    req = req.body(body);
                }

                let mut response = req.send()?;

                // Generate the replay_data to be stored in the file.
                let mut response_body = Vec::<u8>::new();
                // TODO: handle error
                response.read_to_end(&mut response_body);
                let response_data = ResponseData::new(response.url(),
                                                      response.status(),
                                                      response.headers(),
                                                      response_body);
                let replay_data = ReplayData {
                    request: request_data,
                    response: response_data,
                };

                // Write to the file.
                // TODO: handle error
                self.file.write(replay_data);

                // Return the response.
                Ok(response)
            }
            ClientMode::Replay => {
                // Check if we have recorded the equivalent RequestData before.
                // If yes, we will load the serialized Response.
                // If not, we will actually perform the request, store the Response, and then
                //   return it.


                // TODO
                unimplemented!()
            }
        }
    }
}
