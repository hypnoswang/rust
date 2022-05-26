use std::env;

use minigrep;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    minigrep::run(&arguments);
}
