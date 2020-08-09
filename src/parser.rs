use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
//use nom::character::complete::none_of;
use nom::character::complete::space0;
use nom::combinator::recognize;
use nom::multi::many0;
//use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

/// Parse a str that starts with a letter, followed by zero or more
/// letters and/or numbers
///
/// # Example
///
/// ```
/// use cfgparser::alpha_alphanum;
/// use nom::combinator::complete;
///
/// let result = complete(alpha_alphanum)("a123a5");
/// assert_eq!(result, Ok(("","a123a5")));
/// ```
pub fn alpha_alphanum(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, alphanumeric0))(input)
}

/// Parse a single underscore followed by an alpha_alphanum (str starting with a letter followed by
/// zero or more letters or numbers)
///
/// # Example
///
/// ```
/// use cfgparser::underscore_alpha_alphanum;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_alpha_alphanum)("_foobar");
/// assert_eq!(result, Ok(("","_foobar")));
/// ```
pub fn underscore_alpha_alphanum(input: &str) -> IResult<&str, &str> {
    recognize(pair(tag("_"), alpha_alphanum))(input)
}

/// Parse an underscore followed by a str starting with a letter
/// followed by zero or more letters or numbers. Discard the
/// underscore
///
/// # Examples
///
/// ```
/// use cfgparser::underscore_alpha_alphanum2;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_alpha_alphanum2)("_foobar");
/// assert_eq!(result, Ok(("","foobar")));
/// ```
/// We can use this function as the basis for parsing something like
/// role[_subrole[_subsubrole]].
///
/// ```
/// use cfgparser::underscore_alpha_alphanum2;
/// use cfgparser::alpha_alphanum;
/// use nom::combinator::complete;
/// use nom::sequence::tuple;
/// use nom::multi::fold_many0;
/// use nom::sequence::pair;
/// use nom::IResult;
///
/// // this is an example of the power of nom's composition
/// fn parser(s: &str) -> IResult<&str,(&str, Vec<&str>)> {
///    complete(
///      pair(
///          alpha_alphanum,
///          fold_many0(
///            underscore_alpha_alphanum2,
///            Vec::new(),
///            |mut acc: Vec<_>, item| {
///               acc.push(item);
///               acc
///             }
///           ) // fold_many0 end
///        ) // pair end
///      )(s) // complete end
/// }
/// let result = parser("foo_bar_bla");
/// assert_eq!(result, Ok(("",("foo", vec!["bar", "bla"]))))
/// ```
pub fn underscore_alpha_alphanum2(input: &str) -> IResult<&str, &str> {
    preceded(tag("_"), alpha_alphanum)(input)
}

/// Given a str starting with an alpha_alphanum, and followed by zero or more underscore_alpha_alphamums,
/// parse it.
///
/// # Examples
///
/// ```
/// use cfgparser::underscore_alpha_alphanum_seq;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_alpha_alphanum_seq)("fred1_bla_foobar");
/// assert_eq!(result, Ok(("","fred1_bla_foobar")));
/// ```
pub fn underscore_alpha_alphanum_seq(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha_alphanum, many0(underscore_alpha_alphanum)))(input)
}

/// Match the header of the cfg
///
/// we accept the following
/// - '[name]'
/// - '[name_with_under]'
/// - '   [ name_with_various_spaces  ]  '
///
/// # Example
///
/// ```
/// use cfgparser::header;
/// use nom::combinator::complete;
///
/// let result = complete(header)("[the_first_thing]");
/// assert_eq!(result, Ok(("","the_first_thing")));
/// ```
pub fn header(input: &str) -> IResult<&str, &str> {
    let result = tuple((
        space0,
        tag("["),
        space0,
        underscore_alpha_alphanum_seq,
        space0,
        tag("]"),
        space0,
    ))(input)?;
    let (remaining, (_, _, _, key, _, _, _)) = result;
    Ok((remaining, key))
}
use nom::error::ParseError;
use nom::{AsChar, InputTakeAtPosition};

/// parse a string, consuming characters until encountering an "illegal" character
/// at which point parsing stops making progress
pub fn until_illegal_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
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

/// parse out a key value pair from a cfg given a line like
/// key = value
///
/// # Example
///
/// ```
///
pub fn key_value_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let result = tuple((
        underscore_alpha_alphanum_seq,
        space0,
        tag("="),
        space0,
        until_illegal_char,
        space0,
    ))(input)?;
    let (remaining, (key, _, _, _, value, _)) = result;
    Ok((remaining, (key, value)))
}

#[cfg(test)]
#[path = "./parser_tests.rs"]
mod tests;
