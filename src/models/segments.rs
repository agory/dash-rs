use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use super::utils::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Segment {
    #[serde(rename = "t", default)]
    start: Option<f64>,
    #[serde(default)]
    n: f64,
    #[serde(rename = "d")]
    timescale: f64,
    #[serde(rename = "t", default)]
    repeat: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SegmentTimeline {
    #[serde(rename = "S")]
    segments: Vec<Segment>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SegmentListUrl {
    #[serde(default)]
    media: String,
    #[serde(default)]
    media_range: String,
    #[serde(default)]
    index: String,
    #[serde(default)]
    index_range: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SegmentUrl {
    #[serde(rename = "sourceURL", default)]
    source_url: String,
    #[serde(default)]
    range: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SegmentBase {
    // attribut
    #[serde(default)]
    timescale: u64,
    #[serde(default)]
    presentation_time_offset: f64,
    #[serde(default)]
    index_range: String,
    #[serde(default)]
    index_range_exact: bool,
    #[serde(default)]
    availability_time_offset: f64,
    #[serde(default)]
    availability_time_complete: bool,
    // children
    #[serde(rename = "Initialization", default)]
    initialization: Option<SegmentUrl>,
    #[serde(rename = "RepresentationIndex", default)]
    representation_index: Option<SegmentUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultipleSegmentBaseType {
    // attribut
    #[serde(default)]
    timescale: u64,
    #[serde(default)]
    start_number: u64,
    #[serde(default)]
    duration: u64,
    #[serde(default)]
    presentation_time_offset: f64,
    #[serde(default)]
    index_range: String,
    #[serde(default)]
    index_range_exact: bool,
    #[serde(default)]
    availability_time_offset: f64,
    #[serde(default)]
    availability_time_complete: bool,
    // children
    #[serde(rename = "Initialization", default)]
    initialization: Option<SegmentUrl>,
    #[serde(rename = "RepresentationIndex", default)]
    representation_index: Option<SegmentUrl>,
    #[serde(rename = "SegmentTimeline", default)]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching", default)]
    bitstream_switching: Option<SegmentUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SegmentList {
    // attribut
    #[serde(default)]
    timescale: u64,
    #[serde(default)]
    start_number: u64,
    #[serde(default)]
    duration: u64,
    #[serde(default)]
    presentation_time_offset: f64,
    #[serde(default)]
    index_range: String,
    #[serde(default)]
    index_range_exact: bool,
    #[serde(default)]
    availability_time_offset: f64,
    #[serde(default)]
    availability_time_complete: bool,
    #[serde(rename = "xlink:href", default)]
    href: String,
    #[serde(rename = "xlink:actuate", default = "HrefActuate::default")]
    href_actuate: HrefActuate,
    // children
    #[serde(rename = "Initialization", default)]
    initialization: Option<SegmentUrl>,
    #[serde(rename = "RepresentationIndex", default)]
    representation_index: Option<SegmentUrl>,
    #[serde(rename = "SegmentTimeline", default)]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching", default)]
    bitstream_switching: Option<SegmentUrl>,
    #[serde(rename = "SegmentURL", default)]
    segment_url: Vec<SegmentListUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SegmentTemplate {
    // attribut
    #[serde(default)]
    timescale: u64,
    #[serde(default)]
    start_number: u64,
    #[serde(default)]
    duration: u64,
    #[serde(default)]
    presentation_time_offset: f64,
    #[serde(default)]
    index_range: String,
    #[serde(default)]
    index_range_exact: bool,
    #[serde(default)]
    availability_time_offset: f64,
    #[serde(default)]
    availability_time_complete: bool,
    #[serde(default)]
    media: String,
    #[serde(default)]
    index: String,
    #[serde(default)]
    initialization_url: String,
    #[serde(rename = "bitstreamSwitching", default)]
    bitstream_switching_url: String,
    // children
    #[serde(rename = "Initialization", default)]
    initialization: Option<SegmentUrl>,
    #[serde(rename = "RepresentationIndex", default)]
    representation_index: Option<SegmentUrl>,
    #[serde(rename = "SegmentTimeline", default)]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching", default)]
    bitstream_switching: Option<SegmentUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BaseURL {
    #[serde(default)]
    service_location: String,
    #[serde(default)]
    byte_range: String,
    #[serde(default)]
    availability_time_offset: f64,
    #[serde(default)]
    availability_time_complete: bool,
    #[serde(rename = "$value")]
    url: String,
}

impl Display for BaseURL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", &self.url, &self.service_location)
    }
}
