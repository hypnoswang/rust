pub mod config;
pub mod search;

use std::process;

pub fn run(args: &[String]) {
    let cfg = config::Config::new(args).unwrap_or_else(|e| {
        eprintln!("create config failed: {:#?}", e);
        process::exit(1);
    });

    let out = search::start_search(&cfg).unwrap_or_else(|e| {
        eprintln!("Search failed: {:#?}", e);
        process::exit(1);
    });

    for o in out {
        println!("{}", o);
    }
}
