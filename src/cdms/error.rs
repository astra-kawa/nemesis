use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum RawCdmError {
    #[error("XML file read failure")]
    XmlRead,
    #[error("XML deserialization failure")]
    XmlDeserialize,
}
