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

#[derive(Clone, Debug)]
pub struct Headers {
    headers: ::reqwest::header::Headers,
}

impl Headers {
    pub fn to_reqwest_headers(&self) -> ::reqwest::header::Headers {
        self.headers.clone()
    }
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


macro_rules! wrap_as_serde_str_type {
    (
        $name:ident,
        $wrapped:ty
    ) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub value: $wrapped
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                self.value.as_ref().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                use serde::de::Error;
                let s = String::deserialize(deserializer)?;
                Ok($name {
                    value: s.parse().map_err(|e| D::Error::custom(e))?
                })
            }
        }
    }
}

wrap_as_serde_str_type!(Url, ::reqwest::Url);
wrap_as_serde_str_type!(Method, ::reqwest::Method);
// TODO when available
//wrap_as_serde_str_type!(HttpVersion, ::reqwest::HttpVersion);

#[derive(Clone, Debug)]
pub struct StatusCode {
    pub value: ::reqwest::StatusCode
}

impl Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.value.to_u16().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let val = u16::deserialize(deserializer)?;
        Ok(StatusCode {
            value: ::reqwest::StatusCode::from_u16(val)
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTarget {
    url: Url,
    method: Method,
}

impl RequestTarget {
    /// Accessor to mutate the wrapped `reqwest::Method`.
    pub fn method(&mut self) -> &mut ::reqwest::Method {
        &mut self.method.value
    }

    /// Accessor to mutate the wrapped `reqwest::Url`.
    pub fn url(&mut self) -> &mut ::reqwest::Url {
        &mut self.url.value
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseData {
    /// The final URL of this request.
    pub url: Url,

    pub status: StatusCode,
    pub headers: Headers,
    // TODO
    //    version: HttpVersion,
    pub body: Vec<u8>
}

impl ResponseData {
    pub fn new(url: &::reqwest::Url,
               status: &::reqwest::StatusCode,
               headers: &::reqwest::header::Headers,
               body: Vec<u8>) -> Self
    {
        ResponseData {
            url: Url { value: url.clone() },
            status: StatusCode { value: status.clone() },
            headers: Headers { headers: headers.clone() },
            body: body
        }
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

