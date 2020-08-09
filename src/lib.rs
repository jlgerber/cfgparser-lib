mod parser;
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
    let config = Config::parse_cfg(config_str.as_str())?;
    Ok(config.to_owned())
}
