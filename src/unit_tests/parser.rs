use super::*;
use nom::combinator::complete;
use nom::error::ErrorKind;
use nom::Err;

#[test]
fn alphaword_given_word_starting_with_letter_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alphaword)(input)
    }
    let result = parser("a13b");
    assert_eq!(result, Ok(("", "a13b")));
}

#[test]
fn alphaword_given_word_starting_with_number_fails_to_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alphaword)(input)
    }
    let result = parser("1abc");
    assert_eq!(result, Err(Err::Error(("1abc", ErrorKind::Alpha))));
}

// Here we are testing that the parser for an underscore followed
// by a word (a-zA-Z0-9) can parse
#[test]
fn underscore_word_given_input_starting_with_number_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_word)(input)
    }
    let result = parser("_1fofo");
    assert_eq!(result, Ok(("", "_1fofo")));
}

#[test]
fn underscore_word_given_input_starting_with_letter_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(underscore_word)(input)
    }
    let result = parser("_fofo");
    assert_eq!(result, Ok(("", "_fofo")));
}

// test that the parser which takes a word starting with a letter followed by
// zero or more words separated by single underscores can parse. Note that other
// than the first word, we do not care if subsequent words start with a number or
// letter.
#[test]
fn alphaword_many0_underscore_word_given_input_starting_with_num_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(alphaword_many0_underscore_word)(input)
    }
    let result = parser("dude_123_1fofo");
    assert_eq!(result, Ok(("", "dude_123_1fofo")));
}

#[test]
fn header_given_nospaces_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser("[key]");
    assert_eq!(result, Ok(("", "key")));
}
#[test]
fn header_given_spaces_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser(" [ key  ]    ");
    assert_eq!(result, Ok(("", "key")));
}
#[test]
fn header_given_spaces_underscores_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(header)(input)
    }
    let result = parser(" [ kEy_VAlue  ]    ");
    assert_eq!(result, Ok(("", "kEy_VAlue")));
}

#[test]
fn until_illegal_char_given_valid_str_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        complete(until_illegal_char)(input)
    }
    let result = parser("this.is-_one1234567890$");
    assert_eq!(result, Ok(("", "this.is-_one1234567890$")));
}

#[test]
fn until_illegal_char_given_partially_valid_str_can_parse() {
    fn parser(input: &str) -> IResult<&str, &str> {
        until_illegal_char(input)
    }
    let result = parser("this.is- _one1234567890$");
    assert_eq!(result, Ok((" _one1234567890$", "this.is-")));
}

#[test]
fn key_value_pair_given_valid_input_can_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("this_key = val123-543_bla");
    assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
}

#[test]
fn key_value_pair_given_valid_input_starting_with_space_can_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("  this_key = val123-543_bla");
    assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
}

#[test]
fn key_value_pair_given_valid_input2_can_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("another_key = 6053@lichost3-64:27--@bb-fooby.d4.com");
    assert_eq!(
        result,
        Ok(("", ("another_key", "6053@lichost3-64:27--@bb-fooby.d4.com")))
    );
}

#[test]
fn key_value_pair_given_invalid_input2_can_partially_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("another_key = 6053 @lichost3-64:27--@bb-fooby.d4.com");
    assert_eq!(
        result,
        Ok(("@lichost3-64:27--@bb-fooby.d4.com", ("another_key", "6053")))
    );
}
#[test]
fn key_value_pair_given_invalid_input_can_partialy_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair(input)
    }
    let result = parser("another_key = 6053#@lichost3-64:27--@bb-fooby.d4.com");
    assert_eq!(
        result,
        Ok((
            "#@lichost3-64:27--@bb-fooby.d4.com",
            ("another_key", "6053")
        ))
    );
}

#[test]
fn key_value_pair_line_given_valid_input_ending_in_newline_can_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair_line(input)
    }
    let result = parser("this_key = val123-543_bla\n");
    assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
}

#[test]
fn key_value_pair_line_given_valid_input_ending_with_spaces_can_parse() {
    fn parser(input: &str) -> IResult<&str, (&str, &str)> {
        key_value_pair_line(input)
    }
    let result = parser("  this_key = val123-543_bla   ");
    assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
}

#[test]
fn parse_section_given_section_with_empty_lines_can_parse() {
    let section = r#"
    
[test]
this = is
the = way

"#;
    let result = parse_section(section);
    let mut expected = Section::new("test");
    expected.insert("this", "is");
    expected.insert("the", "way");
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn parse_section_given_section_with_empty_lines_and_spaces_can_parse() {
    let section = r#"
    
 [test]
  this = is
   the = way

"#;
    let result = parse_section(section);
    let mut expected = Section::new("test");
    expected.insert("this", "is");
    expected.insert("the", "way");
    assert_eq!(result, Ok(("", expected)));
}

#[test]
fn parse_sections_given_section_with_empty_lines_can_parse() {
    let sections = r#"
    
[test]
this = is
the = way

[test2]
foo = is
the = bar

"#;
    let result = parse_sections(sections);
    let mut section1 = Section::new("test");
    section1.insert("this", "is");
    section1.insert("the", "way");
    let mut section2 = Section::new("test2");
    section2.insert("foo", "is");
    section2.insert("the", "bar");
    assert_eq!(result, Ok(("", vec![section1, section2])));
}

#[test]
fn parse_cfg_from_str_given_section_with_empty_lines_can_parse() {
    let sections = r#"
    
[test]
this = is
the = way

[test2]
foo = is
the = bar

"#;
    let result = parse_cfg_from_str(sections);
    let mut section1 = Section::new("test");
    section1.insert("this", "is");
    section1.insert("the", "way");
    let mut section2 = Section::new("test2");
    section2.insert("foo", "is");
    section2.insert("the", "bar");
    assert_eq!(result, Ok(("", vec![section1, section2])));
}

#[test]
fn parse_cfg_from_str_given_section_with_empty_lines_and_spaces_can_parse() {
    let sections = r#"
    
  [ test ]
    this = is
the = way

 [test2   ]
 foo = is
    the = bar

"#;
    let result = parse_cfg_from_str(sections);
    let mut section1 = Section::new("test");
    section1.insert("this", "is");
    section1.insert("the", "way");
    let mut section2 = Section::new("test2");
    section2.insert("foo", "is");
    section2.insert("the", "bar");
    assert_eq!(result, Ok(("", vec![section1, section2])));
}

#[test]
fn parse_cfg_from_str_given_section_with_no_empty_last_line_can_parse() {
    let sections = r#"
    
[test]
this = is
the = way

[test2]
foo = is
the = bar"#;
    let result = parse_cfg_from_str(sections);
    let mut section1 = Section::new("test");
    section1.insert("this", "is");
    section1.insert("the", "way");
    let mut section2 = Section::new("test2");
    section2.insert("foo", "is");
    section2.insert("the", "bar");
    assert_eq!(result, Ok(("", vec![section1, section2])));
}
