use crate::serde_custom::{duration_iso_8601, tags};
use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

use super::{period::Period, segments::BaseURL, utils::*};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MPDType {
    Dynamic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MPD {
    // Attrib ut
    r#type: MPDType,
    #[serde(with = "tags")]
    profiles: Vec<String>,
    availability_start_time: Option<DateTime<Local>>,
    availability_end_time: Option<DateTime<Local>>,
    publish_time: Option<DateTime<Local>>,
    #[serde(with = "duration_iso_8601", default)]
    media_presentation_duration: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    minimum_update_period: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    min_buffer_time: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    time_shift_buffer_depth: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    suggested_presentation_delay: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
    max_segment_duration: Option<Duration>,
    #[serde(with = "duration_iso_8601", default)]
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
fn display_vec_with_commat<T>(f: &mut fmt::Formatter, vec: &Vec<T>) -> fmt::Result
where
    T: Display,
{
    let mut first = true;
    for location in vec {
        if !first {
            write!(f, ", {}", location)?;
        } else {
            write!(f, "{}", location)?;
        }
        first = false;
    }
    Ok(())
}

impl fmt::Display for MPD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        if !self.locations.is_empty() {
            write!(f, "locations: ")?;
            display_vec_with_commat(f, &self.locations)?;
        }

        if !self.base_url.is_empty() {
            write!(f, "base urls: ")?;
            display_vec_with_commat(f, &self.base_url)?;
        }

        Ok(())
    }
}
