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
        match s {
            "Unable to read XML file" | "XML file read failure" => Ok(RawCdmError::XmlRead),
            "Unable to parse CDM file" | "XML deserialization failure" => {
                Ok(RawCdmError::XmlDeserialize)
            }
            other => Err(format!("Unknown RawCdmError: {other}")),
        }
    }
}
