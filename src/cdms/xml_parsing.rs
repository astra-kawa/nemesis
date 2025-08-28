use crate::cdms::ConjunctionDataMessage;
use quick_xml::de::from_str;

#[derive(Debug, serde::Deserialize)]
struct MessageWrapper {
    pub cdm: ConjunctionDataMessage,
}

/// Parse a CDM XML string into a `ConjunctionDataMessage`.
pub fn parse_cdm_xml_str(xml: &str) -> Result<ConjunctionDataMessage, quick_xml::de::DeError> {
    let wrapper: MessageWrapper = from_str(xml)?;
    Ok(wrapper.cdm)
}

/// Parse a CDM XML file into a `ConjunctionDataMessage`.
pub fn parse_cdm_xml_file<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<ConjunctionDataMessage, Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string(path)?;
    let cdm = parse_cdm_xml_str(&xml)?;
    Ok(cdm)
}
