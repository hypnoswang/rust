//! # Crate root for the project
//! We have two modeules:
//! - config: used to get config from command line arguments and environment variables
//! - search: the implementaotion for the search features

pub mod config;
pub mod search;

use std::env::Args;
use std::error::Error;

/// 使用slice引用的run版本
/// # Examples
/// # Panics
/// This function will not panic in any scense.
/// # Errors
/// + The file to be searched not exists or could not be opened
/// + Not enough arguments or invalid value for environment variable MINIGREP_CASE_SENSITIVE or
/// the thired argument
/// # Safety
pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::new(args)?;
    let out = search::start_search(&cfg)?;

    for o in out {
        println!("{}", o);
    }

    Ok(())
}

/// 使用Iterator引用的run版本
/// # Examples
/// # Panics
/// This function will not panic in any scense.
/// # Errors
/// + The file to be searched not exists or could not be opened
/// + Not enough arguments or invalid value for environment variable MINIGREP_CASE_SENSITIVE or
/// the thired argument
pub fn run_closure(args: Args) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::new_with_closure(args)?;
    let out = search::search_iterly(&cfg)?;

    for o in out {
        println!("{}", o);
    }

    Ok(())
}
