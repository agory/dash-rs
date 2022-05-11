use super::{representations::AdaptationSet, segments::*, utils::*};
use crate::serde_custom::duration_iso_8601;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Period {
    // attribut
    #[serde(rename = "xlink:href", default)]
    href: String,
    #[serde(rename = "xlink:actuate", default = "HrefActuate::default")]
    href_actuate: HrefActuate,
    #[serde(rename = "id", default)]
    id: String,
    #[serde(rename = "start", with = "duration_iso_8601", default)]
    start: Option<Duration>,
    #[serde(rename = "duration", with = "duration_iso_8601", default)]
    duration: Option<Duration>,
    #[serde(rename = "bitstreamSwitching", default)]
    bitstream_switching: bool,
    // child
    #[serde(rename = "BaseURL", default)]
    base_urls: Vec<BaseURL>,
    #[serde(rename = "SegmentBase", default)]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList", default)]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate", default)]
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

impl fmt::Display for Period {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Period ID : {}", &self.id)?;

        if self.start.is_some() {
            write!(f, "Start : {}", self.start.unwrap())?;
        }

        if self.duration.is_some() {
            write!(f, "Duration : {}", self.duration.unwrap())?;
        }

        Ok(())
    }
}
