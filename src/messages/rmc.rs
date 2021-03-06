use crate::fields::cardinality::*;
use crate::fields::distance::*;
use crate::fields::parameter::*;
use crate::fields::speed::*;
use crate::fields::time::*;
use chrono::naive::{NaiveDate, NaiveTime};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Recommended Minimum data
pub struct RMCMessage {
    /// UTC time
    pub time: Option<NaiveTime>,
    /// Data validity status
    pub status: Status,
    /// Latitude
    pub lat: Option<Degree>,
    /// North/South indicator
    pub ns: NorthSouth,
    /// Longitude
    pub lon: Option<Degree>,
    /// East/West indicator
    pub ew: EastWest,
    /// Speed over ground
    pub spd: Option<Knot>,
    /// Course over ground
    pub cog: Option<Degree>,
    pub date: Option<NaiveDate>,
    /// Magnetic variation value.
    pub mv: Option<Degree>,
    /// Magnetic variation E/W indicator.
    pub mv_ew: Option<EastWest>,
    /// Mode Indicator
    pub pos_mode: Fix,
    /// Navigational status indicator
    pub nav_status: NavigationalStatus,
}

pub fn parse_rmc(input: &str) -> IResult<&str, RMCMessage> {
    let (
        remaining,
        (time, status, lat, ns, lon, ew, spd, cog, date, mv, mv_ew, pos_mode, nav_status),
    ) = tuple((
        parse_time,
        parse_status,
        parse_degree,
        parse_north_south_indicator,
        parse_degree,
        parse_east_west_indicator,
        parse_knot,
        parse_raw_degree,
        parse_date,
        parse_degree,
        parse_maybe_east_west_indicator,
        parse_pos_mode,
        parse_navigational_status,
    ))(input)?;
    Ok((
        remaining,
        RMCMessage {
            time,
            status,
            lat,
            ns,
            lon,
            ew,
            spd,
            cog,
            date,
            mv,
            mv_ew,
            pos_mode,
            nav_status,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rmc() {
        let input = "083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V";
        let expected = Ok((
            "",
            RMCMessage {
                time: NaiveTime::from_hms_opt(8, 35, 59),
                status: Status::DataValid,
                lat: Some(Degree(47.1711437)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391522)),
                ew: EastWest::East,
                spd: Some(Knot(0.004)),
                cog: Some(Degree(77.52)),
                date: NaiveDate::from_ymd_opt(2002, 12, 09),
                mv: None,
                mv_ew: None,
                pos_mode: Fix::AutonomousGNSSFix,
                nav_status: NavigationalStatus::NotValid,
            },
        ));

        assert_eq!(expected, parse_rmc(input));
    }
}
