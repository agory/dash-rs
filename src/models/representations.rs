use crate::serde_custom::{conditional_uint, tags};
use either::Either;
use serde::{Deserialize, Serialize};

use super::segments::*;
use super::utils::*;

const EITHER_DEFAULT: Either<u64, bool> = Either::Right(false);
fn either_default() -> Either<u64, bool> {
    EITHER_DEFAULT
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ProtectionAttribut {
    #[serde(rename = "cenc:pssh")]
    CencPssh(String),
    #[serde(rename = "mspr:pro")]
    MsprPro(String),
    #[serde(rename = "dashif:authzurl")]
    DashifAuthzurl(String),
    #[serde(rename = "dashif:laurl")]
    DashifLaurl(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentProtection {
    scheme_id_uri: String,
    #[serde(default)]
    value: String,
    #[serde(default)]
    id: String,
    #[serde(rename = "cenc:default_KID", default)]
    default_kid: Option<String>,
    #[serde(rename = "$value", default)]
    protection_key: Vec<ProtectionAttribut>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdaptationSet {
    // attribut
    #[serde(with = "tags", default)]
    profiles: Vec<String>,
    #[serde(default)]
    width: u64,
    #[serde(default)]
    height: u64,
    #[serde(default)]
    sar: String,
    #[serde(default)]
    frame_rate: String,
    #[serde(default)]
    audio_sampling_rate: String,
    #[serde(default)]
    mime_type: String,
    #[serde(with = "tags", default)]
    segment_profiles: Vec<String>,
    #[serde(with = "tags", default)]
    codecs: Vec<String>,
    #[serde(rename = "maximumSAPPeriod", default)]
    maximum_sap_period: f64,
    #[serde(rename = "startWithSAP", default)]
    // value between 0 and 6
    start_with_sap: u8,
    #[serde(default)]
    max_playout_rate: f64,
    #[serde(default)]
    coding_dependency: bool,
    #[serde(default = "VideoScan::default")]
    scan_type: VideoScan,
    #[serde(rename = "xlink:href", default)]
    href: String,
    #[serde(rename = "xlink:actuate", default = "HrefActuate::default")]
    href_actuate: HrefActuate,
    #[serde(default)]
    id: String,
    #[serde(default)]
    group: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    content_type: String,
    #[serde(default)]
    par: String,
    #[serde(default)]
    max_bandwidth: u64,
    #[serde(default)]
    min_width: u64,
    #[serde(default)]
    max_width: u64,
    #[serde(default)]
    min_height: u64,
    #[serde(default)]
    max_height: u64,
    #[serde(default)]
    min_frame_rate: String,
    #[serde(default)]
    max_frame_rate: String,
    #[serde(with = "conditional_uint", default = "either_default")]
    segment_alignment: Either<u64, bool>,
    #[serde(with = "conditional_uint", default = "either_default")]
    subsegment_alignment: Either<u64, bool>,
    #[serde(default)]
    // value between 0 and 6
    subsegment_starts_with_sap: u8,
    #[serde(default)]
    bitstream_switching: bool,

    // children
    #[serde(rename = "Accessibility", default)]
    accessibilities: Vec<Descriptor>,
    #[serde(rename = "Role", default)]
    roles: Vec<Descriptor>,
    #[serde(rename = "Rating", default)]
    ratings: Vec<Descriptor>,
    #[serde(rename = "Viewpoint", default)]
    viewpoints: Vec<Descriptor>,
    #[serde(rename = "ContentComponent", default)]
    content_components: Vec<ContentComponent>,
    #[serde(rename = "Representation", default)]
    representations: Vec<Representation>,
    #[serde(rename = "BaseURL", default)]
    base_urls: Vec<BaseURL>,
    #[serde(rename = "SegmentBase", default)]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList", default)]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate", default)]
    segment_template: Option<SegmentTemplate>,
    #[serde(rename = "FramePacking", default)]
    frame_packing: Vec<Descriptor>,
    #[serde(rename = "AudioChannelConfiguration", default)]
    audio_channel_configuration: Vec<Descriptor>,
    #[serde(rename = "ContentProtection", default)]
    content_protection: Vec<ContentProtection>,
    #[serde(rename = "EssentialProperty", default)]
    essential_property: Vec<Descriptor>,
    #[serde(rename = "SupplementalProperty", default)]
    supplemental_property: Vec<Descriptor>,
    #[serde(rename = "InbandEventStream", default)]
    inband_event_stream: Vec<EventStream>,
    #[serde(rename = "Switching", default)]
    switching: Vec<Switching>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Representation {
    // attribut
    id: String,
    #[serde(with = "tags", default)]
    profiles: Vec<String>,
    #[serde(default)]
    width: u64,
    #[serde(default)]
    height: u64,
    #[serde(default)]
    sar: String,
    #[serde(default)]
    frame_rate: String,
    #[serde(default)]
    audio_sampling_rate: String,
    #[serde(default)]
    mime_type: String,
    #[serde(with = "tags", default)]
    segment_profiles: Vec<String>,
    #[serde(with = "tags", default)]
    codecs: Vec<String>,
    #[serde(rename = "maximumSAPPeriod", default)]
    maximum_sap_period: f64,
    #[serde(rename = "startWithSAP", default)]
    // value between 0 and 6
    start_with_sap: u8,
    #[serde(default)]
    max_playout_rate: f64,
    #[serde(default)]
    coding_dependency: bool,
    #[serde(default = "VideoScan::default")]
    scan_type: VideoScan,
    #[serde(default)]
    bandwidth: u64,
    #[serde(default)]
    quality_ranking: u64,
    #[serde(default)]
    dependency_id: Vec<String>,
    #[serde(default)]
    media_stream_structure_id: Vec<String>,

    // children
    #[serde(rename = "SubRepresentationType", default)]
    sub_representations: Vec<SubRepresentation>,
    #[serde(rename = "BaseURL", default)]
    base_urls: Vec<BaseURL>,
    #[serde(rename = "SegmentBase", default)]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList", default)]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate", default)]
    segment_template: Option<SegmentTemplate>,
    #[serde(rename = "FramePacking", default)]
    frame_packing: Vec<Descriptor>,
    #[serde(rename = "AudioChannelConfiguration", default)]
    audio_channel_configuration: Vec<Descriptor>,
    #[serde(rename = "ContentProtection", default)]
    content_protection: Vec<ContentProtection>,
    #[serde(rename = "EssentialProperty", default)]
    essential_property: Vec<Descriptor>,
    #[serde(rename = "SupplementalProperty", default)]
    supplemental_property: Vec<Descriptor>,
    #[serde(rename = "InbandEventStream", default)]
    inband_event_stream: Vec<EventStream>,
    #[serde(rename = "Switching", default)]
    switching: Vec<Switching>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]

struct SubRepresentation {
    // attribut
    #[serde(with = "tags", default)]
    profiles: Vec<String>,
    #[serde(default)]
    width: u64,
    #[serde(default)]
    height: u64,
    #[serde(default)]
    sar: String,
    #[serde(default)]
    frame_rate: String,
    #[serde(default)]
    audio_sampling_rate: String,
    #[serde(default)]
    mime_type: String,
    #[serde(with = "tags", default)]
    segment_profiles: Vec<String>,
    #[serde(with = "tags", default)]
    codecs: Vec<String>,
    #[serde(rename = "maximumSAPPeriod", default)]
    maximum_sap_period: f64,
    #[serde(rename = "startWithSAP", default)]
    // value between 0 and 6
    start_with_sap: u8,
    #[serde(default)]
    max_playout_rate: f64,
    #[serde(default)]
    coding_dependency: bool,
    #[serde(default = "VideoScan::default")]
    scan_type: VideoScan,
    #[serde(default)]
    level: u64,
    #[serde(default)]
    dependency_level: Vec<u64>,
    #[serde(default)]
    bandwidth: u64,
    #[serde(default)]
    content_component: Vec<String>,
    // chrildre
    #[serde(rename = "FramePacking", default)]
    frame_packing: Vec<Descriptor>,
    #[serde(rename = "AudioChannelConfiguration", default)]
    audio_channel_configuration: Vec<Descriptor>,
    #[serde(rename = "ContentProtection", default)]
    content_protection: Vec<ContentProtection>,
    #[serde(rename = "EssentialProperty", default)]
    essential_property: Vec<Descriptor>,
    #[serde(rename = "SupplementalProperty", default)]
    supplemental_property: Vec<Descriptor>,
    #[serde(rename = "InbandEventStream", default)]
    inband_event_stream: Vec<EventStream>,
    #[serde(rename = "Switching", default)]
    switching: Vec<Switching>,
}
