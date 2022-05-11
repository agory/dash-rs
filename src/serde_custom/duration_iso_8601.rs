use chrono::Duration;
use serde::{self, de, Deserialize, Deserializer, Serializer};

const WEEK_AS_MS: i64 = 604800000;
const DAYS_AS_MS: i64 = 86400000;
const HOUR_AS_MS: i64 = 3600000;
const MINUTE_AS_MS: i64 = 60000;
const SECONDS_AS_MS: i64 = 1000;

#[derive(Debug)]
enum Iso8601Error {
    InvalidFormat(String, String),
    NotImplemented(String, String),
    InvalidNumberFormat(String, String),
    InvalidTokenPeriod(char, String),
    InvalidTokenTime(char, String),
    InvalidToken(char, String),
}

impl std::fmt::Display for Iso8601Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Iso8601Error::InvalidFormat(message, value) => {
                write!(f, "Invalid ISO_8601 Duration: {} > {}", message, value)
            }
            Iso8601Error::InvalidNumberFormat(message, value) => {
                write!(
                    f,
                    "Invalid ISO_8601 Duration: {} number invalid > {}",
                    message, value
                )
            }
            Iso8601Error::InvalidToken(message, value) => {
                write!(
                    f,
                    "Invalid ISO_8601 Duration: {} is not a token available > {}",
                    message, value
                )
            }
            Iso8601Error::InvalidTokenPeriod(message, value) => {
                write!(
                    f,
                    "Invalid ISO_8601 Duration: {} is not available in Period zone > {}",
                    message, value
                )
            }
            Iso8601Error::InvalidTokenTime(message, value) => {
                write!(
                    f,
                    "Invalid ISO_8601 Duration: {} is not available in Time zone > {}",
                    message, value
                )
            }
            Iso8601Error::NotImplemented(message, value) => {
                write!(
                    f,
                    "Not Implemented ISO_8601 Duration: {} > {}",
                    message, value
                )
            }
        };
    }
}

struct Iso8601 {}

impl Iso8601 {
    fn parse_str(str_sequence: &str) -> Result<Duration, Iso8601Error> {
        let mut milliseconds: f64 = 0.0;
        let mut temp = String::new();
        let mut time = false;
        if !str_sequence.starts_with('P') {
            return Err(Iso8601Error::InvalidFormat(
                "should start with Period indicator (P)".into(),
                str_sequence.to_string(),
            ));
        }
        for char in str_sequence.chars() {
            if char.is_digit(10) {
                temp.push(char);
                continue;
            }

            match (char, time) {
                ('.', _) => temp.push(char),
                ('S' | 'H', false) => {
                    if !time {
                        return Err(Iso8601Error::InvalidTokenPeriod(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                }
                ('Y' | 'W' | 'D', true) => {
                    if !time {
                        return Err(Iso8601Error::InvalidTokenTime(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                }
                ('Y', false) => {
                    if time {
                        return Err(Iso8601Error::InvalidTokenTime(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                    return Err(Iso8601Error::NotImplemented(
                        "Year (Y)".into(),
                        str_sequence.to_string(),
                    ));
                }
                ('W', false) => {
                    if time {
                        return Err(Iso8601Error::InvalidTokenTime(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                    milliseconds = temp.parse::<f64>().map_err(|_| {
                        Iso8601Error::InvalidNumberFormat("weeks".into(), str_sequence.to_string())
                    })? * WEEK_AS_MS as f64;
                    temp = String::new();
                }
                ('D', false) => {
                    if time {
                        return Err(Iso8601Error::InvalidTokenTime(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                    milliseconds += temp.parse::<f64>().map_err(|_| {
                        Iso8601Error::InvalidNumberFormat("days".into(), str_sequence.to_string())
                    })? * DAYS_AS_MS as f64;
                    temp = String::new();
                }
                ('H', true) => {
                    if !time {
                        return Err(Iso8601Error::InvalidTokenPeriod(
                            char,
                            str_sequence.to_string(),
                        ));
                    }
                    milliseconds += temp.parse::<f64>().map_err(|_| {
                        Iso8601Error::InvalidNumberFormat("hours".into(), str_sequence.to_string())
                    })? * HOUR_AS_MS as f64;
                    temp = String::new();
                }
                ('M', true) => {
                    if !time {
                        return Err(Iso8601Error::NotImplemented(
                            "Month (M)".into(),
                            str_sequence.to_string(),
                        ));
                    }
                    milliseconds += temp.parse::<f64>().map_err(|_| {
                        Iso8601Error::InvalidNumberFormat(
                            "minutes".into(),
                            str_sequence.to_string(),
                        )
                    })? * MINUTE_AS_MS as f64;
                    temp = String::new();
                }
                ('S', true) => {
                    milliseconds += temp.parse::<f64>().map_err(|_| {
                        Iso8601Error::InvalidNumberFormat(
                            "seconds".into(),
                            str_sequence.to_string(),
                        )
                    })? * SECONDS_AS_MS as f64;
                    temp = String::new();
                }
                ('T', false) => time = true,
                ('T', true) => {
                    return Err(Iso8601Error::InvalidFormat(
                        "should not have double Time indicator (T)".into(),
                        str_sequence.to_string(),
                    ));
                }
                ('P', _) => continue,
                _ => return Err(Iso8601Error::InvalidToken(char, str_sequence.to_string())),
            }
        }

        Ok(Duration::milliseconds(milliseconds as i64))
    }

    fn to_string(duration: Duration) -> String {
        let mut iso_duration = "P".to_string();
        let mut duration = duration.num_milliseconds();
        let week = duration / WEEK_AS_MS;
        duration -= week * WEEK_AS_MS;
        if week > 0 {
            iso_duration.push_str(&week.to_string());
            iso_duration.push('W');
        }
        let days = duration / DAYS_AS_MS;
        duration -= days * DAYS_AS_MS;
        if days > 0 {
            iso_duration.push_str(&days.to_string());
            iso_duration.push('D');
        }

        if duration <= 0 {
            return iso_duration;
        }

        iso_duration.push('T');
        let hours = duration / HOUR_AS_MS;
        duration -= hours * HOUR_AS_MS;
        if hours > 0 {
            iso_duration.push_str(&hours.to_string());
            iso_duration.push('H');
        }
        let minutes = duration / MINUTE_AS_MS;
        duration -= minutes * MINUTE_AS_MS;
        if minutes > 0 {
            iso_duration.push_str(&minutes.to_string());
            iso_duration.push('M');
        }
        let seconds = duration as f64 / SECONDS_AS_MS as f64;
        if seconds > 0.0 {
            let x: &[_] = &['0', '.'];
            iso_duration.push_str(format!("{:.3}", &seconds).trim_matches(x));
            iso_duration.push('S');
        }
        iso_duration
    }
}

pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value = duration.map(Iso8601::to_string);

    match value {
        Some(string) => serializer.serialize_str(&string),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = &String::deserialize(deserializer)?;

    if str_sequence.is_empty() {
        return Ok(None);
    }

    Ok(Some(
        Iso8601::parse_str(str_sequence).map_err(de::Error::custom)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_string_to_duration() {
        assert_eq!(
            Iso8601::parse_str("P20DT10H45M5.76S").unwrap(),
            Duration::milliseconds(1766705760)
        );
        assert_eq!(
            Iso8601::parse_str("PT10H45M5S").unwrap(),
            Duration::milliseconds(38705000)
        );
        assert_eq!(
            Iso8601::parse_str("P20W").unwrap(),
            Duration::milliseconds(12096000000)
        );
        assert_eq!(
            Iso8601::parse_str("PT43.56S").unwrap(),
            Duration::milliseconds(43560)
        );
        assert_eq!(
            Iso8601::parse_str("PT43.564S").unwrap(),
            Duration::milliseconds(43564)
        );
    }

    #[test]
    fn should_convert_duration_to_string() {
        assert_eq!(
            Iso8601::to_string(Duration::milliseconds(1766705760)),
            "P2W6DT10H45M5.76S"
        );
        assert_eq!(
            Iso8601::to_string(Duration::milliseconds(38705000)),
            "PT10H45M5S"
        );
        assert_eq!(
            Iso8601::to_string(Duration::milliseconds(12096000000)),
            "P20W"
        );
        assert_eq!(
            Iso8601::to_string(Duration::milliseconds(43560)),
            "PT43.56S"
        );
        assert_eq!(
            Iso8601::to_string(Duration::milliseconds(43564)),
            "PT43.564S"
        );
    }

    #[test]
    fn should_fail_to_parse_invalid_format() {
        assert_eq!(
            Iso8601::parse_str("20DT10H45M5.76S")
                .unwrap_err()
                .to_string(),
            "Invalid ISO_8601 Duration: should start with Period indicator (P) > 20DT10H45M5.76S"
        );

        assert_eq!(
            Iso8601::parse_str("P20DT10TH45M5.76S")
                .unwrap_err()
                .to_string(),
            "Invalid ISO_8601 Duration: should not have double Time indicator (T) > P20DT10TH45M5.76S"
        );
    }

    #[test]
    fn should_fail_to_parse_invalide_number_in_iso8601_string() {
        assert_eq!(
            Iso8601::parse_str("P34..D").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: days number invalid > P34..D"
        );
        assert_eq!(
            Iso8601::parse_str("P34..W").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: weeks number invalid > P34..W"
        );
        assert_eq!(
            Iso8601::parse_str("PT5..76H").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: hours number invalid > PT5..76H"
        );
        assert_eq!(
            Iso8601::parse_str("PT5..76M").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: minutes number invalid > PT5..76M"
        );
        assert_eq!(
            Iso8601::parse_str("PT5..76S").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: seconds number invalid > PT5..76S"
        );
    }

    #[test]
    fn should_fail_to_parse_valid_token_in_the_wrong_zone_in_iso8601_string() {
        assert_eq!(
            Iso8601::parse_str("P20S").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: S is not available in Period zone > P20S"
        );
        assert_eq!(
            Iso8601::parse_str("P20H").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: H is not available in Period zone > P20H"
        );
        assert_eq!(
            Iso8601::parse_str("PT20Y").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: Y is not available in Time zone > PT20Y"
        );
        assert_eq!(
            Iso8601::parse_str("PT20W").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: W is not available in Time zone > PT20W"
        );
        assert_eq!(
            Iso8601::parse_str("PT20D").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: D is not available in Time zone > PT20D"
        );
    }

    #[test]
    fn should_fail_to_parse_month_and_year_not_implemented() {
        assert_eq!(
            Iso8601::parse_str("P20M").unwrap_err().to_string(),
            "Not Implemented ISO_8601 Duration: Month (M) > P20M"
        );
        assert_eq!(
            Iso8601::parse_str("P20Y").unwrap_err().to_string(),
            "Not Implemented ISO_8601 Duration: Year (Y) > P20Y"
        );
    }

    #[test]
    fn should_fail_to_parse_invalid_token() {
        assert_eq!(
            Iso8601::parse_str("P20s").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: s is not a token available > P20s"
        );
        assert_eq!(
            Iso8601::parse_str("P20X").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: X is not a token available > P20X"
        );
        assert_eq!(
            Iso8601::parse_str("P20@").unwrap_err().to_string(),
            "Invalid ISO_8601 Duration: @ is not a token available > P20@"
        );
    }
}
