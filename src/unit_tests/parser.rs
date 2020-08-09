use super::*;
use nom::combinator::complete;
//use nom::error::ErrorKind;
//use nom::Err;

//
// header tests
//
mod header {
    use super::*;

    #[test]
    fn given_nospaces_can_parse() {
        fn parser(input: &str) -> IResult<&str, &str> {
            complete(header)(input)
        }
        let result = parser("[key]");
        assert_eq!(result, Ok(("", "key")));
    }

    #[test]
    fn given_spaces_can_parse() {
        fn parser(input: &str) -> IResult<&str, &str> {
            complete(header)(input)
        }
        let result = parser(" [ key  ]    ");
        assert_eq!(result, Ok(("", "key")));
    }

    #[test]
    fn given_spaces_underscores_can_parse() {
        fn parser(input: &str) -> IResult<&str, &str> {
            complete(header)(input)
        }
        let result = parser(" [ kEy_VAlue  ]    ");
        assert_eq!(result, Ok(("", "kEy_VAlue")));
    }
}
//
// until_illegar_char tests
//
mod until_illegal_char {
    use super::*;

    #[test]
    fn given_valid_str_can_parse() {
        fn parser(input: &str) -> IResult<&str, &str> {
            complete(until_illegal_char)(input)
        }
        let result = parser("this.is-_one1234567890$");
        assert_eq!(result, Ok(("", "this.is-_one1234567890$")));
    }

    #[test]
    fn given_partially_valid_str_can_parse() {
        fn parser(input: &str) -> IResult<&str, &str> {
            until_illegal_char(input)
        }
        let result = parser("this.is- _one1234567890$");
        assert_eq!(result, Ok((" _one1234567890$", "this.is-")));
    }
}
//
// key_value_pair tests
//
mod key_value_pair {
    use super::*;

    #[test]
    fn given_valid_input_can_parse() {
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            key_value_pair(input)
        }
        let result = parser("this_key = val123-543_bla");
        assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
    }

    #[test]
    fn given_valid_input_starting_with_space_can_parse() {
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            key_value_pair(input)
        }
        let result = parser("  this_key = val123-543_bla");
        assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
    }

    #[test]
    fn given_valid_input2_can_parse() {
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
    fn given_invalid_input2_can_partially_parse() {
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
    fn given_invalid_input_can_partialy_parse() {
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
}
//
// key_value_pair_line
//
mod key_value_pair_line {
    use super::*;
    #[test]
    fn given_valid_input_ending_in_newline_can_parse() {
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            key_value_pair_line(input)
        }
        let result = parser("this_key = val123-543_bla\n");
        assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
    }

    #[test]
    fn given_valid_input_ending_with_spaces_can_parse() {
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            key_value_pair_line(input)
        }
        let result = parser("  this_key = val123-543_bla   ");
        assert_eq!(result, Ok(("", ("this_key", "val123-543_bla"))));
    }
}
//
// parse_section tests
//
mod parse_section {
    use super::*;

    #[test]
    fn given_section_with_empty_lines_can_parse() {
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
    fn given_section_with_empty_lines_and_spaces_can_parse() {
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
}
//
// parse_sections tests
//
mod parse_sections {
    use super::*;

    #[test]
    fn given_section_with_empty_lines_can_parse() {
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
}
//
// parse_cfg_from_str tests
//
mod parse_cfg_from_str {
    use super::*;

    #[test]
    fn given_section_with_empty_lines_can_parse() {
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
    fn given_section_with_empty_lines_and_spaces_can_parse() {
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
    fn given_section_with_no_empty_last_line_can_parse() {
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
}
