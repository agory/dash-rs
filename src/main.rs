use chrono::{DateTime, Duration, Local};
use serde::{self};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::fs::File;
use std::io::prelude::*;

mod duration_iso_8601_serde;
mod tags_serde;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum MPDType {
    Dynamic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum HrefActuate {
    OnLoad,
    OnRequest,
    Other,
    None,
}

impl HrefActuate {
    fn default() -> Self {
        HrefActuate::OnRequest
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct BaseURL {
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Url {
    #[serde(rename = "$value")]
    url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Descriptor {
    scheme_id_uri: String,
    #[serde(default)]
    value: String,
    #[serde(default)]
    id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Metrics {
    metrics: String,
    reporting: Vec<Descriptor>,
    #[serde(default)]
    range: Vec<Range>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Range {
    #[serde(rename = "starttime", with = "duration_iso_8601_serde", default)]
    start_time: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    duration: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct ProgramInformation {
    #[serde(default)]
    lang: String,
    #[serde(default, rename = "moreInformationURL")]
    more_information_url: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    copyright: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SegmentUrl {
    #[serde(rename = "sourceURL", default)]
    source_url: String,
    #[serde(default)]
    range: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Segment {
    #[serde(rename = "t")]
    start: f64,
    #[serde(default)]
    n: f64,
    #[serde(rename = "d")]
    timescale: f64,
    #[serde(rename = "t", default)]
    repeat: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SegmentTimeline {
    #[serde(rename = "S")]
    segments: Vec<Segment>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SegmentListUrl {
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
#[serde(rename_all = "camelCase")]
struct SegmentBase {
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
struct MultipleSegmentBaseType {
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
struct SegmentList {
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
struct SegmentTemplate {
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
struct Event {
    #[serde(default)]
    presentation_time: u64,
    #[serde(default)]
    duration: f64,
    #[serde(default)]
    id: u64,
    #[serde(default)]
    message_data: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EventStream {
    // Attribut
    #[serde(rename = "xlink:href", default)]
    href: String,
    #[serde(rename = "xlink:actuate", default = "HrefActuate::default")]
    href_actuate: HrefActuate,
    #[serde(default)]
    message_data: String,
    scheme_id_uri: String,
    #[serde(default)]
    value: String,
    #[serde(default)]
    timescale: u64,

    // children
    #[serde(default, rename = "Event")]
    events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Subset {
    contains: Vec<u64>,
    #[serde(default)]
    id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AdaptationSet {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Period {
    // attribut
    #[serde(rename = "xlink:href", default)]
    href: String,
    #[serde(rename = "xlink:actuate", default = "HrefActuate::default")]
    href_actuate: HrefActuate,
    #[serde(rename = "id", default)]
    id: String,
    #[serde(rename = "start", with = "duration_iso_8601_serde", default)]
    start: Option<Duration>,
    #[serde(rename = "duration", with = "duration_iso_8601_serde", default)]
    duration: Option<Duration>,
    #[serde(rename = "bitstreamSwitching", default)]
    bitstream_switching: bool,
    // child
    #[serde(rename = "BaseURL", default)]
    base_url: Vec<BaseURL>,
    #[serde(default)]
    supplemental_property: Vec<Descriptor>,
    #[serde(default)]
    segment_base: Option<SegmentBase>,
    #[serde(default)]
    segment_list: Option<SegmentList>,
    #[serde(default)]
    segment_template: Option<SegmentTemplate>,
    #[serde(default)]
    asset_identifier: Option<Descriptor>,
    #[serde(default)]
    event_stream: Vec<EventStream>,
    #[serde(rename = "AdaptationSet", default)]
    adaptations: Vec<AdaptationSet>,
    #[serde(rename = "Subset", default)]
    subsets: Vec<Subset>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct MPD {
    // Attribut
    r#type: MPDType,
    #[serde(with = "tags_serde")]
    profiles: Vec<String>,
    availability_start_time: Option<DateTime<Local>>,
    availability_end_time: Option<DateTime<Local>>,
    publish_time: Option<DateTime<Local>>,
    #[serde(with = "duration_iso_8601_serde", default)]
    media_presentation_duration: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    minimum_update_period: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    min_buffer_time: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    time_shift_buffer_depth: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    suggested_presentation_delay: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    max_segment_duration: Option<Duration>,
    #[serde(with = "duration_iso_8601_serde", default)]
    max_subsegment_duration: Option<Duration>,

    // Children
    #[serde(rename = "Period")]
    periods: Vec<Period>,
    #[serde(rename = "ProgramInformation", default)]
    programme_information: Vec<ProgramInformation>,
    #[serde(rename = "BaseURL", default)]
    base_url: Vec<BaseURL>,
    #[serde(rename = "Location", default)]
    locations: Vec<Url>,
    #[serde(rename = "Metrics", default)]
    metrics: Vec<Metrics>,
    #[serde(rename = "EssentialProperty", default)]
    essential_property: Vec<Descriptor>,
    #[serde(rename = "SupplementalProperty", default)]
    supplemental_property: Vec<Descriptor>,
    #[serde(rename = "UTCTiming", default)]
    utc_timing: Vec<Descriptor>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("resources\\hdeindex-1.mpd")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mpd: MPD = from_str(&contents).unwrap();
    println!("{:#?}", mpd);
    Ok(())
}
