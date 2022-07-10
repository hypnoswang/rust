use clap::{arg, Command};
use web;

fn main() {
    let matches = Command::new("web")
        .version("1.0")
        .author("WangQiang <hypnos.wang@gmail.com>")
        .about("An exercise of rust")
        .arg(arg!(--config <VALUE>))
        .get_matches();

    let cfg = matches
        .get_one::<String>("config")
        .expect("cfg argument is required");

    println!("config: {:?}", cfg);

    web::run_server(cfg);
}
