use cfgparser::Config;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = r"

[playa]
name = PlayaVista
short_name = ddpv
prefix = dd

[portland]
name = Portland
short_name = ddpd
prefix = pd

";

    let config = Config::parse_cfg(contents)?;
    println!("{:#?}", config);
    for section in config.sections() {
        println!("{:?}", section);
    }
    Ok(())
}
