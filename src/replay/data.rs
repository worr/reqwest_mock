use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::time::Duration;

use super::RedirectPolicy;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct Headers {
    headers: ::reqwest::header::Headers,
}

impl Serialize for Headers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let tuples_iter = self.headers.iter().map(|hv| (hv.name().to_string(), hv.value_string()));
        HashMap::<String, String>::from_iter(tuples_iter).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Headers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let map = HashMap::<String, String>::deserialize(deserializer)?;

        let mut headers = ::reqwest::header::Headers::new();
        for (name, value) in map.iter() {
            headers.append_raw(name.clone(), value.as_bytes().to_vec())
        }

        Ok(Headers {
            headers: headers
        })
    }
}

impl Deref for Headers {
    type Target = ::reqwest::header::Headers;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.headers
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    method: ::reqwest::Method
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.method.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        Ok(Method {
            method: ::reqwest::Method::from_str(s.as_ref()).map_err(|e| D::Error::custom(e))?
        })
    }
}

#[derive(Clone, Debug)]
pub struct Url {
    url: ::reqwest::Url
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.url.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        Ok(Url {
            url: ::reqwest::Url::parse(s.as_ref()).map_err(|e| D::Error::custom(e))?
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTarget {
    url: Url,
    method: Method,
}

impl RequestTarget {
    /// Accessor to mutate the wrapped `reqwest::Method`.
    pub fn method(&mut self) -> &mut ::reqwest::Method {
        &mut self.method.method
    }

    /// Accessor to mutate the wrapped `reqwest::Url`.
    pub fn url(&mut self) -> &mut ::reqwest::Url {
        &mut self.url.url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub target: Option<RequestTarget>,

    pub gzip: bool,
    pub redirect: RedirectPolicy,
    pub timeout: Option<Duration>,
    pub basic_auth: Option<BasicAuth>,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Default for RequestData {
    fn default() -> Self {
        RequestData {
            target: None,
            gzip: true,
            redirect: RedirectPolicy::default(),
            timeout: None,
            basic_auth: None,
            headers: Headers { headers: ::reqwest::header::Headers::new() },
            body: None,
        }
    }
}

impl RequestData {
    pub fn new_for_client(cd: &ClientData) -> Self {
        let mut data = Self::default();
        data.gzip = cd.gzip;
        data.redirect = cd.redirect.clone();
        data.timeout = cd.timeout.clone();
        data
    }
}

/// This struct is held by the Client and stores the current config at the beginnig of a request.
/// Generally this is mostly a conveniency type and we will always store all the relevant data
/// in each `RequestData` instance anyway.
#[derive(Debug)]
pub struct ClientData {
    pub gzip: bool,
    pub redirect: RedirectPolicy,
    pub timeout: Option<Duration>,
}

impl Default for ClientData {
    fn default() -> Self {
        ClientData {
            gzip: true,
            redirect: RedirectPolicy::default(),
            timeout: None
        }
    }
}

