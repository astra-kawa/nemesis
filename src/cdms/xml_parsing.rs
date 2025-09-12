use crate::cdms::RawConjunctionDataMessage;
use dioxus::prelude::*;
use quick_xml::de::from_str;

#[derive(Debug, serde::Deserialize)]
struct MessageWrapper {
    pub cdm: RawConjunctionDataMessage,
}

/// Parse a CDM XML string into a `ConjunctionDataMessage`.
pub fn parse_cdm_xml_str(xml: &str) -> Result<RawConjunctionDataMessage, quick_xml::de::DeError> {
    let wrapper: MessageWrapper = from_str(xml)?;
    Ok(wrapper.cdm)
}

/// Parse a CDM XML file into a `ConjunctionDataMessage`.
#[server]
pub async fn parse_cdm_xml_file(
    path: std::path::PathBuf,
) -> Result<RawConjunctionDataMessage, ServerFnError> {
    let xml = match std::fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => return Err(ServerFnError::new("Unable to read XML file".to_string())),
    };

    match parse_cdm_xml_str(&xml) {
        Ok(cdm) => Ok(cdm),
        Err(_) => Err(ServerFnError::new("Unable to parse CDM file".to_string())),
    }
}
