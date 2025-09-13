use crate::cdms::RawCdmError;
use crate::cdms::RawConjunctionDataMessage;
use dioxus::prelude::*;
#[cfg(any(feature = "server", test))]
use quick_xml::de::from_str;

#[cfg(any(feature = "server", test))]
#[derive(serde::Deserialize)]
struct MessageWrapper {
    cdm: RawConjunctionDataMessage,
}

/// Parse a CDM XML string into a `ConjunctionDataMessage`.
#[cfg(any(feature = "server", test))]
pub fn parse_cdm_xml_str(xml: &str) -> Result<RawConjunctionDataMessage, quick_xml::de::DeError> {
    let wrapper: MessageWrapper = from_str(xml)?;
    Ok(wrapper.cdm)
}

/// Parse a CDM XML file into a `ConjunctionDataMessage`.
#[server]
pub async fn parse_cdm_xml_file(
    path: std::path::PathBuf,
) -> Result<RawConjunctionDataMessage, ServerFnError<RawCdmError>> {
    let xml = match std::fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => return Err(ServerFnError::WrappedServerError(RawCdmError::XmlRead)),
    };

    match parse_cdm_xml_str(&xml) {
        Ok(cdm) => Ok(cdm),
        Err(_) => Err(ServerFnError::WrappedServerError(
            RawCdmError::XmlDeserialize,
        )),
    }
}
