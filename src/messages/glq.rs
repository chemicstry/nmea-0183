use crate::parser_utils::parse_string;
use nom::IResult;

#[derive(Debug, PartialEq)]
/// Poll a standard message if the current Talker ID is GL
pub struct GLQMessage<'a> {
    /// Message ID of the message to be polled
    pub msg_id: &'a str,
}

pub fn parse_glq(input: &str) -> IResult<&str, GLQMessage> {
    let (remaining, msg_id) = parse_string(input)?;
    Ok((remaining, GLQMessage { msg_id }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_glq() {
        let input = "RMC";
        let expected = Ok(("", GLQMessage { msg_id: "RMC" }));

        assert_eq!(expected, parse_glq(input));
    }
}
