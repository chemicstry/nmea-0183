use crate::fields::cardinality::*;
use crate::fields::distance::*;
use crate::fields::parameter::*;
use crate::fields::time::*;
use crate::parser_utils::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// GNSS fix data
pub struct GNSMessage {
    /// UTC time
    pub time: Option<NaiveTime>,
    /// Latitude
    pub lat: Option<Degree>,
    /// North/South indicator
    pub ns: Option<NorthSouth>,
    /// Longitude
    pub lon: Option<Degree>,
    /// East/West indicator
    pub ew: Option<EastWest>,
    /// Positioning mode,
    pub pos_mode: Vec<Fix>,
    /// Number of satellites used
    pub num_sv: Option<u8>,
    /// Horizontal Dilution of Precision
    pub hdop: Option<f64>,
    /// Altitude above mean sea level
    pub alt: Option<Meter>,
    /// Geoid separation
    pub sep: Option<Meter>,
    /// Age of differential corrections
    pub diff_age: Option<Second>,
    /// ID of station providing differential corrections
    pub diff_station: Option<u8>,
    /// Navigational status indicator
    pub nav_status: Status,
}

pub fn parse_gns(input: &str) -> IResult<&str, GNSMessage> {
    let (
        remaining,
        (
            time,
            lat,
            ns,
            lon,
            ew,
            pos_mode,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
            nav_status,
        ),
    ) = tuple((
        parse_time,
        parse_degree,
        parse_maybe_north_south_indicator,
        parse_degree,
        parse_maybe_east_west_indicator,
        parse_pos_mode_vec,
        parse_u8,
        parse_float,
        parse_meter,
        parse_meter,
        parse_second,
        parse_u8,
        parse_status,
    ))(input)?;
    Ok((
        remaining,
        GNSMessage {
            time,
            lat,
            ns,
            lon,
            ew,
            pos_mode,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
            nav_status,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gns() {
        let input = "103600.01,5114.51176,N,00012.29380,W,ANNN,07,1.18,111.5,45.6,,,V";
        let expected = Ok((
            "",
            GNSMessage {
                time: Some(NaiveTime::from_hms_milli(10, 36, 00, 10)),
                lat: Some(Degree(51.145117600000006)), // floats ¯\_(ツ)_/¯
                ns: Some(NorthSouth::North),
                lon: Some(Degree(0.12293799999999999)), // floats ¯\_(ツ)_/¯
                ew: Some(EastWest::West),
                pos_mode: vec![Fix::AutonomousGNSSFix, Fix::NoFix, Fix::NoFix, Fix::NoFix],
                num_sv: Some(7),
                hdop: Some(1.18),
                alt: Some(Meter(111.5)),
                sep: Some(Meter(45.6)),
                diff_age: None,
                diff_station: None,
                nav_status: Status::DataInvalid,
            },
        ));

        assert_eq!(expected, parse_gns(input));
    }
}
