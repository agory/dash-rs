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
struct Period {}

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
