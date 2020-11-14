use super::structs::*;
use crate::fields::identity::*;
use crate::fields::parameter::SentenceType;
use crate::messages::*;
use nom::bytes::complete::take_until;
use nom::character::complete::{crlf, char};
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_message_type(input: &str) -> IResult<&str, MessageType> {
    if input.len() < 4 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
    }
    let (message_type_str, remaining) = input.split_at(3);
    let (remaining, _) = char(',')(remaining)?;
    let message_type = MessageType::from(message_type_str).unwrap();
    Ok((remaining, message_type))
}

pub fn parse_sentence_type(input: &str) -> IResult<&str, SentenceType> {
    // Array slicing is safe here because nth(0) is Some(_)
    match input.chars().nth(0) {
        Some('$') => Ok((&input[1..], SentenceType::Parametric)),
        Some('!') => Ok((&input[1..], SentenceType::Encapsulation)),
        None => Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete))),
        _ => Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf))),
    }
}

pub fn parse_sentence(input: &str) -> IResult<&str, Sentence> {
    let (remaining, sentence_type) = parse_sentence_type(input)?;
    let (data_buffer, (talker, message_type)) = get_headers_if_sentence_valid(remaining)?;

    let (remaining_data, message) = match message_type.as_str() {
        "DTM" => {
            let (remaining, data) = parse_dtm(data_buffer)?;
            (remaining, Message::DTM(data))
        }
        "GBQ" => {
            let (remaining, data) = parse_gbq(data_buffer)?;
            (remaining, Message::GBQ(data))
        }
        "GGA" => {
            let (remaining, data) = parse_gga(data_buffer)?;
            (remaining, Message::GGA(data))
        }
        "GSA" => {
            let (remaining, data) = parse_gsa(data_buffer)?;
            (remaining, Message::GSA(data))
        }
        "GSV" => {
            let (remaining, data) = parse_gsv(data_buffer)?;
            (remaining, Message::GSV(data))
        }
        "GLL" => {
            let (remaining, data) = parse_gll(data_buffer)?;
            (remaining, Message::GLL(data))
        }
        "ZDA" => {
            let (remaining, data) = parse_zda(data_buffer)?;
            (remaining, Message::ZDA(data))
        }
        "RMC" => {
            let (remaining, data) = parse_rmc(data_buffer)?;
            (remaining, Message::RMC(data))
        }
        "GLQ" => {
            let (remaining, data) = parse_glq(data_buffer)?;
            (remaining, Message::GLQ(data))
        }
        "GNQ" => {
            let (remaining, data) = parse_gnq(data_buffer)?;
            (remaining, Message::GNQ(data))
        }
        "GBS" => {
            let (remaining, data) = parse_gbs(data_buffer)?;
            (remaining, Message::GBS(data))
        }
        "GNS" => {
            let (remaining, data) = parse_gns(data_buffer)?;
            (remaining, Message::GNS(data))
        }
        "GPQ" => {
            let (remaining, data) = parse_gpq(data_buffer)?;
            (remaining, Message::GPQ(data))
        }
        "GRS" => {
            let (remaining, data) = parse_grs(data_buffer)?;
            (remaining, Message::GRS(data))
        }
        "GST" => {
            let (remaining, data) = parse_gst(data_buffer)?;
            (remaining, Message::GST(data))
        }
        "TXT" => {
            let (remaining, data) = parse_txt(data_buffer)?;
            (remaining, Message::TXT(data))
        }
        "VLW" => {
            let (remaining, data) = parse_vlw(data_buffer)?;
            (remaining, Message::VLW(data))
        }
        "VTG" => {
            let (remaining, data) = parse_vtg(data_buffer)?;
            (remaining, Message::VTG(data))
        }
        _ => return Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)))
    };

    if remaining_data.len() == 0 {
        Ok((
            remaining_data,
            Sentence {
                sentence_type,
                talker,
                message,
            },
        ))
    } else {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::NonEmpty)));
    }
}

pub fn get_headers_if_sentence_valid(input: &str) -> IResult<&str, (Talker, MessageType)> {
    let (after_data, data) = take_until("*")(input)?;
    // Index subscription is safe because take_until does not consume the pattern
    let (after_checksum, checksum) = parse_checksum(&after_data[1..])?;
    if !sentence_is_valid(data, checksum) {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::Verify)));
    }
    if crlf(after_checksum)?.0.len() != 0 {
        return Err(nom::Err::Failure((input, nom::error::ErrorKind::NonEmpty)));
    }
    Ok(tuple((parse_talker, parse_message_type))(data)?)
}

pub fn parse_checksum(input: &str) -> IResult<&str, u8> {
    let (after_cs, maybe_cs) = take_until("\r")(input)?;
    if let Ok(cs) = decode_cs(maybe_cs) {
        Ok((after_cs, cs))
    } else {
        Err(nom::Err::Failure((input, nom::error::ErrorKind::Digit)))
    }
}

pub fn decode_cs(s: &str) -> Result<u8, nom::Err<(&str, nom::error::ErrorKind)>> {
    // The checksum is supposed to be 2 characters wide
    if s.chars().nth(1).is_none() {
        return Err(nom::Err::Failure((s, nom::error::ErrorKind::Complete)));
    } else {
        u8::from_str_radix(&s[0..2], 16)
            .map_err(|_| nom::Err::Failure((s, nom::error::ErrorKind::Digit)))
    }
}

pub fn sentence_is_valid(data: &str, checksum: u8) -> bool {
    let computed = data.chars().fold(0, |sum, c| sum ^ c as u8);
    computed == checksum
}
