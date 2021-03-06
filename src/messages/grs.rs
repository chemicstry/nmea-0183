use crate::fields::distance::*;
use crate::fields::identity::*;
use crate::fields::parameter::*;
use crate::fields::time::*;
use chrono::naive::NaiveTime;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// GNSS Range Residuals
pub struct GRSMessage {
    /// UTC time of associated position fix,
    pub time: Option<NaiveTime>,
    /// Computation method used
    pub mode: Option<ComputationMethod>,
    /// Range residuals for SVs used in navigation
    pub residuals: [Option<Meter>; 12],
    /// NMEA defined GNSS System ID,
    pub system_id: Option<u8>,
    /// NMEA defined GNSS Signal ID
    pub signal_id: Option<u8>,
}

pub fn parse_grs(input: &str) -> IResult<&str, GRSMessage> {
    let (remaining, (time, mode, residuals, system_id, signal_id)) = tuple((
        parse_time,
        parse_computation_method,
        parse_residuals,
        parse_system,
        parse_signal,
    ))(input)?;
    Ok((
        remaining,
        GRSMessage {
            time,
            mode,
            residuals,
            system_id,
            signal_id,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grs() {
        let input = "104148.00,1,2.6,2.2,-1.6,-1.1,-1.7,-1.5,5.8,1.7,,,,,1,1";
        let expected = Ok((
            "",
            GRSMessage {
                time: Some(NaiveTime::from_hms(10, 41, 48)),
                mode: Some(ComputationMethod::AfterGGA),
                residuals: [
                    Some(Meter(2.6)),
                    Some(Meter(2.2)),
                    Some(Meter(-1.6)),
                    Some(Meter(-1.1)),
                    Some(Meter(-1.7)),
                    Some(Meter(-1.5)),
                    Some(Meter(5.8)),
                    Some(Meter(1.7)),
                    None,
                    None,
                    None,
                    None,
                ],
                system_id: Some(1),
                signal_id: Some(1),
            },
        ));

        assert_eq!(expected, parse_grs(input));
    }
}
