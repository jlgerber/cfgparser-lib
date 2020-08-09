//! This file contains unused functions. It should NOT be exposed via
//! lib.rs. It is just a holding place incase i change my mind about implementation

/// Parse a single underscore followed by an alphaword (str starting with a letter followed by
/// zero or more letters or numbers)
///
/// # Example
///
/// ```
/// use cfgparser::underscore_alphaword;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_alphaword)("_foobar");
/// assert_eq!(result, Ok(("","_foobar")));
/// ```
// NOT USED
pub fn underscore_alphaword(input: &str) -> IResult<&str, &str> {
    recognize(pair(tag("_"), alphaword))(input)
}

/// Given a str starting with an alphaword, and followed by zero or more underscore_alpha_alphamums,
/// parse it.
///
/// # Examples
///
/// ```
/// use cfgparser::alphaword_many0_underscore_alphaword;
/// use nom::combinator::complete;
///
/// let result = complete(alphaword_many0_underscore_alphaword)("fred1_bla_foobar");
/// assert_eq!(result, Ok(("","fred1_bla_foobar")));
/// ```
pub fn alphaword_many0_underscore_alphaword(input: &str) -> IResult<&str, &str> {
    recognize(pair(alphaword, many0(underscore_alphaword)))(input)
}

/// Parse an underscore followed by a str starting with a letter
/// followed by zero or more letters or numbers. Discard the
/// underscore in the returned value
///
/// # Examples
///
/// ```
/// use cfgparser::underscore_alphaword_drop_underscore;
/// use nom::combinator::complete;
///
/// let result = complete(underscore_alphaword_drop_underscore)("_foobar");
/// assert_eq!(result, Ok(("","foobar")));
/// ```
/// We can use this function as the basis for parsing something like
/// role[_subrole[_subsubrole]].
///
/// ```
/// use cfgparser::underscore_alphaword_drop_underscore;
/// use cfgparser::alphaword;
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
///          alphaword,
///          fold_many0(
///            underscore_alphaword_drop_underscore,
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
// NOT USED
pub fn underscore_alphaword_drop_underscore(input: &str) -> IResult<&str, &str> {
    preceded(tag("_"), alphaword)(input)
}

#[test]
fn underscore_alphaword_given_fofo_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_alphaword)(input)
    }
    let result = parser("_fofo");
    assert_eq!(result, Ok(("", "_fofo")));
}
// Here we are testing that
#[test]
fn alphaword_many0_underscore_alphaword_given_a_word_followed_by_multiple_underscore_words_can_parse(
) {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alphaword_many0_underscore_alphaword)(input)
    }
    let result = parser("a13b_fofo_bla");
    assert_eq!(result, Ok(("", "a13b_fofo_bla")));
}
