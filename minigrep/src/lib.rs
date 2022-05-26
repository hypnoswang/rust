pub mod config;
pub mod search;

use std::error::Error;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::new(args)?;
    let out = search::start_search(&cfg)?;

    for o in out {
        println!("{}", o);
    }

    Ok(())
}
