use super::*;

#[test]
fn from_cfg_given_str_returns_config() {
    let contents = r#"
[test]
this = is
the = way
    
[location]
name = France.d3.com
short_name = fr
"#;
    let result = Config::parse_cfg_from_str(contents);

    let mut config = Config::new();

    let mut section = Section::new("test");
    section.insert("this", "is");
    section.insert("the", "way");

    let mut section2 = Section::new("location");
    section2.insert("name", "France.d3.com");
    section2.insert("short_name", "fr");

    config.insert("location", section2);
    config.insert("test", section);

    let result = result.unwrap();

    assert_eq!(result, config);
}
#[test]
fn from_cfg_given_str_with_comments_returns_config() {
    let contents = r#"
[test]
this = is#this should word
the = way
# this is a comment
[location]
name = France.d3.com
short_name = fr
"#;
    let result = Config::parse_cfg_from_str(contents);

    let mut config = Config::new();

    let mut section = Section::new("test");
    section.insert("this", "is");
    section.insert("the", "way");

    let mut section2 = Section::new("location");
    section2.insert("name", "France.d3.com");
    section2.insert("short_name", "fr");

    config.insert("location", section2);
    config.insert("test", section);

    let result = result.unwrap();

    assert_eq!(result, config);
}
