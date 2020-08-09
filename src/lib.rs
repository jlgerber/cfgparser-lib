//! cfgparser
//!
//! Library for parsing a cfg file.
//!
//! Cfg files are text files generally with a cfg extension
//!
//! ```notrust
//! [header]
//! key = value
//! key2 = value2
//! keyN = ValueN
//!
//! [headerN]
//! ...
//! ```
//! The differ from Toml in that the values are implicitly strings.
//! Furthermore, they do not support nesting.
//!
//! The parser generates a structure that is a thin wrapper around
//! a map of maps. There are two main custom structs provided for this:
//!
//! - Confg
//! - Section
//!
//! The Config houses zero or more Section instances.
//! The Section contains zero or more key value pairs.
//! Getters are provided for each struct to aid usability.
//!
//! The entrypoint for generating a Config may be found at the root
//! of the crate, via a function called:
//! ```notrust
//! parse_from_path
//! ```  
pub mod parser;
pub use parser::*;

mod section;
pub use section::Section;
pub use section::SectionOwned;

mod config;
pub use config::Config;
pub use config::ConfigOwned;

use std::path::Path;

/// Create a config from a path
///
/// # Example
///
/// ```
/// use cfgparser::from_path;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut cfgpath = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// cfgpath.push("example_data");
/// cfgpath.push("operating_systems.cfg");
/// let config = from_path(cfgpath)?;
/// # Ok(())
/// # }
/// ```
pub fn from_path<P>(cfg_path: P) -> Result<ConfigOwned, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let config_str = std::fs::read_to_string(cfg_path.as_ref())?;
    let config = Config::parse_cfg_from_str(config_str.as_str())?;
    Ok(config.to_owned())
}
