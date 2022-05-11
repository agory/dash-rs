use crate::serde_custom::duration_iso_8601;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HrefActuate {
    OnLoad,
    OnRequest,
    Other,
    None,
}

impl HrefActuate {
    pub fn default() -> Self {
        HrefActuate::OnRequest
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    #[serde(rename = "$value")]
    url: String,
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    scheme_id_uri: String,
    #[serde(default)]
    value: String,
    #[serde(default)]
    id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Metrics {
    metrics: String,
    reporting: Vec<Descriptor>,
    #[serde(default)]
    range: Vec<Range>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "starttime", with = "duration_iso_8601", default)]
    start_time: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    duration: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ProgramInformation {
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
#[serde(rename_all = "camelCase")]
pub struct Event {
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
pub struct EventStream {
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
pub struct Subset {
    contains: Vec<u64>,
    #[serde(default)]
    id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentComponent {
    #[serde(default)]
    id: String,
    #[serde(default)]
    lang: String,
    #[serde(default)]
    content_type: String,
    #[serde(default)]
    par: String,

    #[serde(rename = "Accessibility", default)]
    accessibilities: Vec<Descriptor>,
    #[serde(rename = "Role", default)]
    roles: Vec<Descriptor>,
    #[serde(rename = "Rating", default)]
    ratings: Vec<Descriptor>,
    #[serde(rename = "Viewpoint", default)]
    viewpoints: Vec<Descriptor>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoScan {
    Progressive,
    Interlaced,
    Unknown,
}

impl VideoScan {
    pub fn default() -> VideoScan {
        VideoScan::Progressive
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SwitchingType {
    Media,
    Bitstream,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Switching {
    interval: u64,
    r#type: SwitchingType,
}
