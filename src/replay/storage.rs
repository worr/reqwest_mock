//! Defines an API to manage replay storage files.

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use super::data::{ResponseData, RequestData};

pub struct ReplayFile {
    path: PathBuf,
}

impl ReplayFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        ReplayFile { path: path.into() }
    }

    // TODO error type
    pub fn read(&self) -> Result<ReplayData, Box<Error>> {
        let file = File::open(&self.path)?;
        Ok(::serde_json::from_reader(file)?)
    }

    // TODO error type
    pub fn write(&self, data: ReplayData) -> Result<(), Box<Error>> {
        let mut file = File::create(&self.path)?;
        Ok(::serde_json::to_writer(file, &data)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplayData {
    pub request: RequestData,
    pub response: ResponseData,
}
