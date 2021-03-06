use crate::fields::distance::*;
use crate::fields::time::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// GNSS Pseudo Range Error Statistics
pub struct GSTMessage {
    /// UTC time of associated position fix
    pub time: Option<NaiveTime>,
    /// RMS value of the standard deviation of the ranges
    pub range_rms: Option<Meter>,
    /// Standard deviation of semi-major axis
    pub std_major: Option<Meter>,
    /// Standard deviation of semi-minor axis
    pub std_minor: Option<Meter>,
    /// Orientation of semi-major axis
    pub orient: Option<Degree>,
    /// Standard deviation of latitude error
    pub std_lat: Option<Meter>,
    /// Standard deviation of longitude error
    pub std_lon: Option<Meter>,
    /// Standard deviation of altitude error
    pub std_alt: Option<Meter>,
}

pub fn parse_gst(input: &str) -> IResult<&str, GSTMessage> {
    let (remaining, (time, range_rms, std_major, std_minor, orient, std_lat, std_lon, std_alt)) =
        tuple((
            parse_time,
            parse_meter,
            parse_meter,
            parse_meter,
            parse_degree,
            parse_meter,
            parse_meter,
            parse_meter,
        ))(input)?;
    Ok((
        remaining,
        GSTMessage {
            time,
            range_rms,
            std_major,
            std_minor,
            orient,
            std_lat,
            std_lon,
            std_alt,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gst() {
        let input = "082356.00,1.8,,,,1.7,1.3,2.2";
        let expected = Ok((
            "",
            GSTMessage {
                time: Some(NaiveTime::from_hms(08, 23, 56)),
                range_rms: Some(Meter(1.8)),
                std_major: None,
                std_minor: None,
                orient: None,
                std_lat: Some(Meter(1.7)),
                std_lon: Some(Meter(1.3)),
                std_alt: Some(Meter(2.2)),
            },
        ));

        assert_eq!(expected, parse_gst(input));
    }
}
