use cfgparser::Config;
use prettytable::*;
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: cfgparser <CFG>");
        std::process::exit(0);
    }

    let contents = std::fs::read_to_string(args[1].as_str())?;
    let config = Config::parse_cfg_from_str(contents.as_str())?;
    //println!("{:#?}", config);
    for section in config.sections() {
        let mut table = table!();
        table.set_titles(row!["HEADING", section.name()]);
        for item in section.iter() {
            table.add_row(row!(item.0, item.1));
        }
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
    Ok(())
}
