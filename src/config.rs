use crate::parser::parse_cfg;
use crate::Section;
use crate::SectionOwned;

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap;

#[cfg(not(feature = "ordered"))]
use std::collections::hash_map::Values;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as HashMap;

#[cfg(feature = "ordered")]
use indexmap::map::Values;

/// Cofig structure
#[derive(Debug, PartialEq, Clone)]
pub struct Config<'b> {
    sections: HashMap<&'b str, Section<'b>>,
}

impl<'b> Default for Config<'b> {
    fn default() -> Config<'b> {
        Self {
            sections: HashMap::new(),
        }
    }
}

impl<'b> Config<'b> {
    /// Create a new instance of Config
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a section into the config
    pub fn insert(&mut self, name: &'b str, section: Section<'b>) -> bool {
        self.sections.insert(name, section).is_none()
    }
    /// Create an instance of ConfigOwned from self. ConfigOwned, as it sounds,
    /// owns its data, whereas Config is a view onto data
    /// that has been passed into it.
    pub fn to_owned(&self) -> ConfigOwned {
        let mut config = ConfigOwned::new();
        for (key, section) in self.sections.iter() {
            config.insert(*key, section);
        }
        config
    }

    /// Retrieve an iterator over sections in the config
    pub fn sections(&self) -> Values<&str, Section> {
        self.sections.values()
    }

    /// Given a &str representing a cfg, parse it into a Config instance.
    ///
    /// # Example
    ///
    /// ```
    /// use cfgparser::Config;
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// let contents = r#"
    /// [playa]
    /// name = Playa Vista
    /// short_name = ddpv
    /// prefix = dd
    ///
    /// [portland]
    /// name = Portland
    /// short_name = ddpd
    /// prefix = pd
    /// "#;
    ///
    /// let config = Config::parse_cfg(contents)?;
    /// for section in config.sections() {
    ///     println!("{:#?}", section);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse_cfg(input: &'b str) -> Result<Self, String> {
        match parse_cfg(input) {
            Ok((_, sections)) => {
                let mut cfg = Self::new();
                for section in sections.into_iter() {
                    cfg.insert(section.name(), section);
                }
                Ok(cfg)
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    /// Retrieve a section given its name
    ///
    /// # Example
    ///
    pub fn get(&self, section: &str) -> Option<&Section> {
        self.sections.get(section)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigOwned {
    sections: HashMap<String, SectionOwned>,
}

impl Default for ConfigOwned {
    fn default() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }
}

impl ConfigOwned {
    /// Create a new instance of ConfigOwned
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a section into the config
    pub fn insert(&mut self, section_name: &str, section: &Section) -> bool {
        let (name, section) = section.to_owned();
        // todo - add custom error
        assert_eq!(name, section_name);
        self.sections.insert(name.to_string(), section).is_none()
    }

    /// Retrieve a section
    pub fn get<I>(&self, section: I) -> Option<&SectionOwned>
    where
        I: AsRef<str>,
    {
        self.sections.get(section.as_ref())
    }
}

#[cfg(test)]
#[path = "./unit_tests/config.rs"]
mod unit_tests;
