#[derive(Debug, Clone, PartialEq)]
pub struct ConjunctionDataMessage {
    pub id: String,
    pub version: String,
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub comment: Option<Vec<String>>,
    pub creation_date: String,
    pub originator: String,
    pub message_for: Option<String>,
    pub message_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub relative_metadata_data: RelativeMetadataData,
    pub primary_segment: Segment,
    pub secondary_segment: Segment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelativeMetadataData {
    pub comment: Option<Vec<String>>,
    pub tca: String,
    pub miss_distance: f64,
    pub relative_speed: Option<f64>,
    pub relative_state_vector: Option<RelativeStateVector>,
    pub start_screen_period: Option<String>,
    pub stop_screen_period: Option<String>,
    pub screen_volume_frame: Option<String>,
    pub screen_volume_x: Option<f64>,
    pub screen_volume_y: Option<f64>,
    pub screen_volume_z: Option<f64>,
    pub screen_entry_time: Option<String>,
    pub screen_exit_time: Option<String>,
    pub collision_probability: Option<f64>,
    pub collision_probability_method: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelativeStateVector {
    pub relative_position_r: Option<f64>,
    pub relative_position_t: Option<f64>,
    pub relative_position_n: Option<f64>,
    pub relative_velocity_r: Option<f64>,
    pub relative_velocity_t: Option<f64>,
    pub relative_velocity_n: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub metadata: SegmentMetadata,
    pub data: SegmentData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentMetadata {
    pub comment: Option<Vec<String>>,
    pub object: String,
    pub object_designator: String,
    pub catalog_name: String,
    pub object_name: String,
    pub international_designator: String,
    pub object_type: Option<String>,
    pub operator_contact_position: Option<String>,
    pub operator_organization: Option<String>,
    pub operator_phone: Option<String>,
    pub operator_email: Option<String>,
    pub ephemeris_name: String,
    pub covariance_method: String,
    pub maneuverable: String,
    pub orbit_center: Option<String>,
    pub ref_frame: String,
    pub gravity_model: Option<String>,
    pub atmospheric_model: Option<String>,
    pub n_body_perturbations: Option<String>,
    pub solar_rad_pressure: Option<String>,
    pub earth_tides: Option<String>,
    pub intrack_thrust: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentData {
    pub comment: Option<Vec<String>>,
    pub state_vector: StateVector,
    pub od_parameters: Option<OdParameters>,
    pub additional_parameters: Option<AdditionalParameters>,
    pub covariance_matrix: CovarianceMatrix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateVector {
    pub comment: Option<Vec<String>>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dot: f64,
    pub y_dot: f64,
    pub z_dot: f64,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct AdditionalParameters {
    pub comment: Option<Vec<String>>,
    pub area_pc: Option<f64>,
    pub cd_area_over_mass: Option<f64>,
    pub cr_area_over_mass: Option<f64>,
    pub thrust_acceleration: Option<f64>,
    pub sedr: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CovarianceMatrix {
    pub comment: Option<Vec<String>>,
    pub cr_r: f64,
    pub ct_r: f64,
    pub ct_t: f64,
    pub cn_r: f64,
    pub cn_t: f64,
    pub cn_n: f64,
    pub crdot_r: f64,
    pub crdot_t: f64,
    pub crdot_n: f64,
    pub crdot_rdot: f64,
    pub ctdot_r: f64,
    pub ctdot_t: f64,
    pub ctdot_n: f64,
    pub ctdot_rdot: f64,
    pub ctdot_tdot: f64,
    pub cndot_r: f64,
    pub cndot_t: f64,
    pub cndot_n: f64,
    pub cndot_rdot: f64,
    pub cndot_tdot: f64,
    pub cndot_ndot: f64,
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
    pub cthr_r: Option<f64>,
    pub cthr_t: Option<f64>,
    pub cthr_n: Option<f64>,
    pub cthr_rdot: Option<f64>,
    pub cthr_tdot: Option<f64>,
    pub cthr_ndot: Option<f64>,
    pub cthr_drg: Option<f64>,
    pub cthr_srp: Option<f64>,
    pub cthr_thr: Option<f64>,
}
