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
    pub comment: Option<Vec<String>>,
    pub creation_date: Option<String>,
    pub originator: Option<String>,
    pub message_for: Option<String>,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Body {
    #[serde(rename = "relativeMetadataData")]
    pub relative_metadata_data: RelativeMetadataData,
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
    pub comment: Option<Vec<String>>,
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
    pub comment: Option<Vec<String>>,
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
    pub state_vector: StateVector,
    #[serde(rename = "odParameters")]
    pub od_parameters: Option<OdParameters>,
    #[serde(rename = "additionalParameters")]
    pub additional_parameters: Option<AdditionalParameters>,
    #[serde(rename = "covarianceMatrix")]
    pub covariance_matrix: Option<CovarianceMatrix>,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OdParameters {
    pub comment: Option<Vec<String>>,
    pub time_lastob_start: Option<String>,
    pub time_lastob_end: Option<String>,
    pub recommended_od_span: Option<f64>,
    pub actual_od_span: Option<f64>,
    pub obs_available: Option<u32>,
    pub obs_used: Option<u32>,
    pub residuals_accepted: Option<f64>,
    pub weighted_rms: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AdditionalParameters {
    pub comment: Option<Vec<String>>,
    pub area_pc: Option<f64>,
    pub cd_area_over_mass: Option<f64>,
    pub cr_area_over_mass: Option<f64>,
    pub thrust_acceleration: Option<f64>,
    pub sedr: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CovarianceMatrix {
    pub comment: Option<Vec<String>>,
    pub cr_r: Option<f64>,
    pub ct_r: Option<f64>,
    pub ct_t: Option<f64>,
    pub cn_r: Option<f64>,
    pub cn_t: Option<f64>,
    pub cn_n: Option<f64>,
    pub crdot_r: Option<f64>,
    pub crdot_t: Option<f64>,
    pub crdot_n: Option<f64>,
    pub crdot_rdot: Option<f64>,
    pub ctdot_r: Option<f64>,
    pub ctdot_t: Option<f64>,
    pub ctdot_n: Option<f64>,
    pub ctdot_rdot: Option<f64>,
    pub ctdot_tdot: Option<f64>,
    pub cndot_r: Option<f64>,
    pub cndot_t: Option<f64>,
    pub cndot_n: Option<f64>,
    pub cndot_rdot: Option<f64>,
    pub cndot_tdot: Option<f64>,
    pub cndot_ndot: Option<f64>,
    pub cdrg_r: Option<f64>,
    pub cdrg_t: Option<f64>,
    pub cdrg_n: Option<f64>,
    pub cdrg_rdot: Option<f64>,
    pub cdrg_tdot: Option<f64>,
    pub cdrg_ndot: Option<f64>,
    pub cdrg_drg: Option<f64>,
    pub csrp_r: Option<f64>,
    pub csrp_t: Option<f64>,
    pub csrp_n: Option<f64>,
    pub csrp_rdot: Option<f64>,
    pub csrp_tdot: Option<f64>,
    pub csrp_ndot: Option<f64>,
    pub csrp_drg: Option<f64>,
    pub csrp_srp: Option<f64>,
}
