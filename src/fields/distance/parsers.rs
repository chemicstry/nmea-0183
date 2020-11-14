use super::structs::*;
use crate::{
    fields::cardinality::{
        parse_east_west_indicator, parse_north_south_indicator, EastWest, NorthSouth,
    },
    parser_utils::parse_float,
};
use nom::IResult;

pub fn parse_raw_degree(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_degree = if let Some(float) = maybe_float {
        Some(Degree(float))
    } else {
        None
    };
    Ok((remaining, maybe_degree))
}

pub fn parse_degree(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_degree = if let Some(float) = maybe_float {
        Some(Degree(float / 100.)) // 4717.11399 is actually 47.1711399
    } else {
        None
    };
    Ok((remaining, maybe_degree))
}

pub fn parse_position_degree(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_degree = if let Some(mut float) = maybe_float {
        // NMEA latitude/longitude of 4717.11399 is actually 47 degrees and 17.11399 minutes
        float /= 100.0;
        let degrees = float as u8;
        let minutes = (float - degrees as f64) * 100.0;
        let degrees = degrees as f64 + (minutes / 60.0);
        Some(Degree(degrees))
    } else {
        None
    };
    Ok((remaining, maybe_degree))
}

pub fn parse_latitude(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }

    let (remaining, maybe_degree) = parse_position_degree(input)?;
    let (remaining, ns) = parse_north_south_indicator(remaining)?;

    let maybe_degree = match ns {
        NorthSouth::North => maybe_degree,
        NorthSouth::South => maybe_degree.map(|d| Degree(-d.0)),
    };

    Ok((remaining, maybe_degree))
}

pub fn parse_longitude(input: &str) -> IResult<&str, Option<Degree>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }

    let (remaining, maybe_degree) = parse_position_degree(input)?;
    let (remaining, ns) = parse_east_west_indicator(remaining)?;

    let maybe_degree = match ns {
        EastWest::East => maybe_degree,
        EastWest::West => maybe_degree.map(|d| Degree(-d.0)),
    };

    Ok((remaining, maybe_degree))
}

pub fn parse_minute(input: &str) -> IResult<&str, Option<Minute>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_minute = if let Some(float) = maybe_float {
        Some(Minute(float))
    } else {
        None
    };
    Ok((remaining, maybe_minute))
}

pub fn parse_second(input: &str) -> IResult<&str, Option<Second>> {
    let (remaining, sec) = parse_float(input)?;
    if let Some(s) = sec {
        Ok((remaining, Some(Second(s))))
    } else {
        Ok((remaining, None))
    }
}

pub fn parse_meter(input: &str) -> IResult<&str, Option<Meter>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_meter = if let Some(float) = maybe_float {
        Some(Meter(float))
    } else {
        None
    };
    Ok((remaining, maybe_meter))
}

pub fn parse_last_meter(input: &str) -> IResult<&str, Option<Meter>> {
    if input.len() < 1 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (remaining, maybe_float) = parse_float(input)?;

    let maybe_meter = if let Some(float) = maybe_float {
        Some(Meter(float))
    } else {
        None
    };
    Ok((remaining, maybe_meter))
}

pub fn ensure_meter(input: &str) -> IResult<&str, ()> {
    if input.len() < 2 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    if input.chars().nth(1) != Some(',') {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    } else {
        match input.chars().nth(0) {
            // Index subscription is safe since input has at least 2 items
            Some('M') => Ok((&input[2..], ())),
            _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
        }
    }
}

pub fn parse_residuals(input: &str) -> IResult<&str, [Option<Meter>; 12]> {
    let mut remaining = input;
    let mut residuals = [None; 12];
    for i in 0..12 {
        let parsed = parse_meter(remaining)?;
        remaining = parsed.0;
        residuals[i] = parsed.1;
    }

    Ok((remaining, residuals))
}
