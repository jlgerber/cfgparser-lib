use super::*;
use nom::combinator::complete;
use nom::error::ErrorKind;
use nom::Err;

#[test]
fn alpha_alphanum_given_a13b_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alpha_alphanum)(input)
    }
    let result = parser("a13b");
    assert_eq!(result, Ok(("", "a13b")));
}

#[test]
fn alpha_alphanum_given_1abc_fails_to_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alpha_alphanum)(input)
    }
    let result = parser("1abc");
    assert_eq!(result, Err(Err::Error(("1abc", ErrorKind::Alpha))));
}

#[test]
fn sep_ann_given_fofo_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_alpha_alphanum)(input)
    }
    let result = parser("_fofo");
    assert_eq!(result, Ok(("", "_fofo")));
}

#[test]
fn underscore_alpha_alphanum_seq_given_a13b_fofo_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_alpha_alphanum_seq)(input)
    }
    let result = parser("a13b_fofo");
    assert_eq!(result, Ok(("", "a13b_fofo")));
}

#[test]
fn underscore_alpha_alphanum2_given_input_parses() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_alpha_alphanum2)(input)
    }
    let result = parser("_fofo");
    assert_eq!(result, Ok(("", "fofo")));
}

#[test]
fn header_given_nospaces_is_ok() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser("[key]");
    assert_eq!(result, Ok(("", "key")));
}
#[test]
fn header_given_spaces_is_ok() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser(" [ key  ]    ");
    assert_eq!(result, Ok(("", "key")));
}
#[test]
fn header_given_spaces_underscores_is_ok() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser(" [ kEy_VAlue  ]    ");
    assert_eq!(result, Ok(("", "kEy_VAlue")));
}

#[test]
fn until_illegal_char_given_valid_str() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(until_illegal_char)(input)
    }
    let result = parser("this.is-_one1234567890$");
    assert_eq!(result, Ok(("", "this.is-_one1234567890$")));
}

#[test]
fn until_illegal_char_given_partially_valid_str() {
    fn parser(input: &str) -> IResult<&str, &str> {
        until_illegal_char(input)
    }
    let result = parser("this.is- _one1234567890$");
    assert_eq!(result, Ok((" _one1234567890$", "this.is-")));
}

#[test]
fn key_value_pair_given_valid_input() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("this_key = val123-543_bla");
    assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
}
