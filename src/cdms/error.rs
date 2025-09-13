use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum RawCdmError {
    #[error("XML file read failure")]
    XmlRead,
    #[error("XML deserialization failure")]
    XmlDeserialize,
}

impl FromStr for RawCdmError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == RawCdmError::XmlRead.to_string() {
            Ok(RawCdmError::XmlRead)
        } else if s == RawCdmError::XmlDeserialize.to_string() {
            Ok(RawCdmError::XmlDeserialize)
        } else {
            Err(format!("Unknown RawCdmError: {s}"))
        }
    }
}
