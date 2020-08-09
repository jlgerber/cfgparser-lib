#[cfg(feature = "ordered")]
use indexmap::IndexMap as HashMap;

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Section<'a> {
    name: &'a str,
    items: HashMap<&'a str, &'a str>,
}

impl<'a> Section<'a> {
    /// Create a new Section with the supplied name
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            items: HashMap::new(),
        }
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    /// Insert a key and value into the items map
    pub fn insert(&mut self, key: &'a str, value: &'a str) -> Option<&'a str> {
        self.items.insert(key, value)
    }

    /// Convert a Section into a SectionOwned
    pub fn to_owned(&self) -> (&str, SectionOwned) {
        let mut owned = SectionOwned::new();
        for (key, value) in self.items.iter() {
            owned.insert(key.to_string(), value.to_string());
        }
        (self.name, owned)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SectionOwned {
    items: HashMap<String, String>,
}

impl SectionOwned {
    /// Create a new Section with the supplied name
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Insert a key and value into the items map
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.items.insert(key.into(), value.into())
    }
}
