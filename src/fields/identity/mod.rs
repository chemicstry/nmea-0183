mod parsers;
mod structs;

pub use parsers::*;
pub use structs::*;

#[cfg(test)]
mod talker_tests {
    use super::*;

    #[test]
    fn test_parse_talker() {
        let input = "HNtest";
        let expected_output = Ok(("test", Talker::HeadingNonNorthSeekingGyro));
        assert_eq!(expected_output, parse_talker(input));
    }

    #[test]
    fn test_parse_unknown_talker() {
        let input = "PAtest";
        let expected_output = Ok(("test", Talker::Unknown(TalkerId::from("PA").unwrap())));
        assert_eq!(expected_output, parse_talker(input));
    }

    #[test]
    fn test_parse_talker_empty_input() {
        let input = "";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_talker(input));
    }
}
