use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_till;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

/// Parse a str that starts with a letter, followed by zero or more
/// letters and/or numbers
///
/// # Example
///
/// ```
/// use cfgparser::parser::atoms::alphaword;
/// use nom::combinator::complete;
///
/// let result = complete(alphaword)("a123a5");
/// assert_eq!(result, Ok(("","a123a5")));
/// ```
pub fn alphaword(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, alphanumeric0))(input)
}

/// Parse a single underscore followed by an alphanum
///
/// # Example
///
/// ```
/// use cfgparser::parser::atoms::underscore_word;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_word)("_1foo1");
/// assert_eq!(result, Ok(("","_1foo1")));
/// ```
pub fn underscore_word(input: &str) -> IResult<&str, &str> {
    recognize(pair(tag("_"), alphanumeric1))(input)
}

/// Given a str starting with an alphaword, and followed by zero or more _words,
/// parse it.
///
/// # Examples
///
/// ```
/// use cfgparser::parser::atoms::alphaword_many0_underscore_word;
/// use nom::combinator::complete;
///
/// let result = complete(alphaword_many0_underscore_word)("fred1_1bla_foobar");
/// assert_eq!(result, Ok(("","fred1_1bla_foobar")));
/// ```
pub fn alphaword_many0_underscore_word(input: &str) -> IResult<&str, &str> {
    recognize(pair(alphaword, many0(underscore_word)))(input)
}

/// This parser recognizes 3 conditions:
///
/// - a '#' followed by anything, up to and including a \n
/// - a '#' followed by anything
/// - a zero or more spaces followed by an optional \n
///
/// # Examples
///
/// ## Comment
/// ```
/// use cfgparser::parser::atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("# this is an example");
/// assert_eq!(result, Ok(("", "# this is an example")));
/// ```
/// ## spaces
/// ```
/// use cfgparser::parser::atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("    ").unwrap();
/// assert_eq!(result, (("","    ")) );
/// ```
///
/// ## comment with newline
/// ```
/// use cfgparser::parser::atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("# this is an example\n");
/// assert_eq!(result, Ok(("", "# this is an example\n")));
/// ```
/// ## spaces with newline
/// ```
/// use cfgparser::parser::atoms::space0_eol;
/// use nom::combinator::complete;
///
/// let result = complete(space0_eol)("    \n").unwrap();
/// assert_eq!(result, (("","    \n")) );
/// ```
pub fn space0_eol(input: &str) -> IResult<&str, &str> {
    alt((
        // this one ends in a \n
        recognize(pair(
            tag("#"),
            recognize(pair(take_till(|c| c == '\n'), take(1usize))),
        )),
        // this one doesnt (like if it is the last line of the file)
        recognize(pair(tag("#"), take_till(|c| c == '\n'))),
        // this is just zero or more spaces and optionally a \n
        multispace0,
    ))(input)
}

#[cfg(test)]
#[path = "../unit_tests/parser_atoms.rs"]
mod unit_tests;
