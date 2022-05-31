pub mod config;
pub mod search;

use std::env::Args;
use std::error::Error;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::new(args)?;
    let out = search::start_search(&cfg)?;

    for o in out {
        println!("{}", o);
    }

    Ok(())
}

pub fn run_closure(args: Args) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::new_with_closure(args)?;
    let out = search::search_iterly(&cfg)?;

    for o in out {
        println!("{}", o);
    }

    Ok(())
}
