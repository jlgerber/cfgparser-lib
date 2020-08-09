//! parser
//!
//! This module houses the nom parser implementation.
//! ```parser::*``` uses to the following naming convention:
//!
//! - *alphaword* - a word comprised of letters and numbers, starting with a leter
//! - *word* - a word comprised of letters and numbers
//!
use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::all_consuming;
use nom::combinator::complete;
use nom::error::ParseError;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::{AsChar, InputTakeAtPosition};

pub mod atoms;
use crate::Section;
use atoms::*;

// match a basic header. That is something that matches the following pattern:
// [key]
fn header(input: &str) -> IResult<&str, &str> {
    let result = tuple((
        space0,
        tag("["),
        space0,
        alphaword_many0_underscore_word,
        space0,
        tag("]"),
        space0,
    ))(input)?;
    let (remaining, (_, _, _, key, _, _, _)) = result;
    Ok((remaining, key))
}
// Take header with a newline at the end
fn header_newline(input: &str) -> IResult<&str, &str> {
    terminated(header, newline)(input)
}
// Match the header of the cfg
//
// we accept the following
// - '[name]'
// - '[name_with_under]'
// - '   [ name_with_various_spaces  ]  '
//
// # Example
//
// ```
// use cfgparser::header_line;
//
// let result = header_line("[the_first_1thing]");
// assert_eq!(result, Ok(("","the_first_1thing")));
// ```
fn header_line(input: &str) -> IResult<&str, &str> {
    alt((header_newline, complete(header)))(input)
}

// parse a string, consuming characters until encountering an "illegal" character
// at which point parsing stops making progress
fn until_illegal_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let itm = item.as_char();
        ['#', ' ', '[', ']', '!', '*', '\t', '\n', '\r', '"', '\'']
            .iter()
            .any(|x| x == &itm)
    })
}

// parse out a key value pair from a cfg given a line like
// key = value
fn key_value_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let result = tuple((
        space0,
        alphaword_many0_underscore_word,
        space0,
        tag("="),
        space0,
        until_illegal_char,
        space0,
    ))(input)?;
    let (remaining, (_, key, _, _, _, value, _)) = result;
    Ok((remaining, (key, value)))
}
/// parse a key value pair followed by a newline.
pub fn key_value_pair_newline(input: &str) -> IResult<&str, (&str, &str)> {
    let result = tuple((
        space0,
        alphaword_many0_underscore_word,
        space0,
        tag("="),
        space0,
        until_illegal_char,
        space0,
        newline,
    ))(input)?;
    let (remaining, (_, key, _, _, _, value, _, _)) = result;
    Ok((remaining, (key, value)))
}

// read a line defining a key value pair. either it ends in a carriage return,
// or it ends the file (ie it is complete)
fn key_value_pair_line(input: &str) -> IResult<&str, (&str, &str)> {
    alt((key_value_pair_newline, complete(key_value_pair)))(input)
}

/// parse a section
fn parse_section(input: &str) -> IResult<&str, Section> {
    let results = tuple((
        space0_eol,
        header_line,
        many1(key_value_pair_line),
        space0_eol,
    ))(input)?;

    let (rest, (_, key, kvpairs, _)) = results;
    let mut section = Section::new(key);
    for value in kvpairs {
        section.insert(value.0, value.1);
    }
    Ok((rest, section))
}

// Parse multiple sections, having at least one section.
fn parse_sections(input: &str) -> IResult<&str, Vec<Section>> {
    many1(parse_section)(input)
}

/// Given a config, return
pub fn parse_cfg_from_str(input: &str) -> IResult<&str, Vec<Section>> {
    all_consuming(parse_sections)(input)
}

#[cfg(test)]
#[path = "./unit_tests/parser.rs"]
mod unit_tests;
