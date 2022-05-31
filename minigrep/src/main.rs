use std::env;
use std::process;

use minigrep;

fn main() {
    //let arguments: Vec<String> = env::args().collect();

    //if let Err(e) = minigrep::run(&arguments) {
    //    eprintln!("Failed: {:?}", e);
    //    process::exit(1);
    //}

    minigrep::run_closure(env::args()).unwrap_or_else(|e| {
        eprintln!("Failed: {:?}", e);
        process::exit(1);
    });
}
