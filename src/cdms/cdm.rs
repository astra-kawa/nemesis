use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConjunctionDataMessage {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Header {
    pub comment: Option<String>,
    pub creation_date: Option<String>,
    pub originator: Option<String>,
    pub message_for: Option<String>,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Body {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: Option<RelativeMetadataData>,
    #[serde(default, rename = "segment")]
    pub segment: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeMetadataData {
    pub tca: Option<String>,
    pub miss_distance: Option<f64>,
    pub relative_speed: Option<f64>,
    #[serde(rename = "relativeStateVector")]
    pub relative_state_vector: Option<RelativeStateVector>,
    pub collision_probability: Option<f64>,
    pub collision_probability_method: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RelativeStateVector {
    pub relative_position_r: Option<f64>,
    pub relative_position_t: Option<f64>,
    pub relative_position_n: Option<f64>,
    pub relative_velocity_r: Option<f64>,
    pub relative_velocity_t: Option<f64>,
    pub relative_velocity_n: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Segment {
    pub metadata: SegmentMetadata,
    pub data: SegmentData,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SegmentMetadata {
    pub comment: Option<String>,
    pub object: Option<String>,
    pub object_designator: Option<String>,
    pub catalog_name: Option<String>,
    pub object_name: Option<String>,
    pub international_designator: Option<String>,
    pub object_type: Option<String>,
    pub operator_contact_position: Option<String>,
    pub operator_organization: Option<String>,
    pub operator_phone: Option<String>,
    pub operator_email: Option<String>,
    pub ephemeris_name: Option<String>,
    pub covariance_method: Option<String>,
    pub maneuverable: Option<String>,
    pub ref_frame: Option<String>,
    pub gravity_model: Option<String>,
    pub atmospheric_model: Option<String>,
    pub n_body_perturbations: Option<String>,
    pub solar_rad_pressure: Option<String>,
    pub earth_tides: Option<String>,
    pub intrack_thrust: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SegmentData {
    #[serde(rename = "stateVector")]
    pub state_vector: Option<StateVector>,
    // Additional fields exist in CDM but are optional and ignored here.
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateVector {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub x_dot: Option<f64>,
    pub y_dot: Option<f64>,
    pub z_dot: Option<f64>,
}
