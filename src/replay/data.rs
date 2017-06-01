use reqwest::header::Headers;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use super::RedirectPolicy;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>,
}

fn serialize_headers(headers: &Headers) -> HashMap<String, String> {
    let tuples_iter = headers.iter().map(|hv| (hv.name().to_string(), hv.value_string()));
    HashMap::from_iter(tuples_iter)
}

fn deserialize_headers(source: &HashMap<String, String>) -> Headers {
    let mut headers = Headers::new();
    for (name, value) in source.iter() {
        headers.append_raw(name.clone(), value.as_bytes().to_vec())
    }
    headers
}

#[derive(Debug)]
pub struct HeadersData {
    headers: Headers,
}

impl Serialize for HeadersData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serialize_headers(&self.headers).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HeadersData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let map = HashMap::deserialize(deserializer)?;
        Ok(HeadersData {
            headers: deserialize_headers(&map)
        })
    }
}

impl Deref for HeadersData {
    type Target = Headers;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}

impl DerefMut for HeadersData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.headers
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub gzip: bool,
    pub redirect: RedirectPolicy,
    pub timeout: Option<Duration>,
    pub basic_auth: Option<BasicAuth>,
    pub headers: HeadersData,
    pub body: Option<Vec<u8>>,
}

impl Default for RequestData {
    fn default() -> Self {
        RequestData {
            gzip: true,
            redirect: RedirectPolicy::default(),
            timeout: None,
            basic_auth: None,
            headers: HeadersData { headers: Headers::new() },
            body: None,
        }
    }
}
