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

fn display_vec_with_sections<T>(f: &mut fmt::Formatter, vec: &Vec<T>) -> fmt::Result
where
    T: Display,
{
    let mut first = true;
    for location in vec {
        if !first {
            writeln!(f, ", {}", location)?;
        } else {
            writeln!(f, "---")?;
            writeln!(f, "{}", location)?;
        }
        first = false;
    }
    Ok(())
}

impl fmt::Display for MPD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "## MPD ")?;

        if self.minimum_update_period.is_some() {
            writeln!(f, "Refresh period: {}", self.minimum_update_period.unwrap())?;
        }

        if self.min_buffer_time.is_some() {
            writeln!(f, "Minimum Buffer Time: {}", self.min_buffer_time.unwrap())?;
        }

        // Customize so only `x` and `y` are denoted.
        if !self.locations.is_empty() {
            writeln!(f, "locations: ")?;
            display_vec_with_commat(f, &self.locations)?;
        }

        if !self.base_url.is_empty() {
            writeln!(f, "base urls: ")?;
            display_vec_with_commat(f, &self.base_url)?;
        }

        if self.publish_time.is_some() {
            writeln!(f, "base publish_time: {}", self.publish_time.unwrap())?;
        }

        match (&self.availability_start_time, &self.availability_end_time) {
            (Some(start), Some(end)) => writeln!(f, "availability: {} - {}", start, end)?,
            (None, Some(end)) => writeln!(f, "availability: Unknown - {}", end)?,
            (Some(start), None) => writeln!(f, "availability: {} - Unknown", start)?,
            (None, None) => writeln!(f, "availability: Unknown")?,
        }

        writeln!(f, "### Periods ")?;
        display_vec_with_sections(f, &self.periods)?;

        Ok(())
    }
}
