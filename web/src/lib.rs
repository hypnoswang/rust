use closure_thread_pool::Pool;
use config::Config;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::str;

pub mod config;

pub fn run_server(cf: &str) {
    let cfg = Config::read_cfg(cf).unwrap_or_else(|e| {
        eprintln!("Parse config failed: {}", e.to_string());
        process::exit(-1);
    });

    println!("the config is: {:#?}", cfg);

    let mut worker_cnt = 4;
    if cfg.workers > 0 && cfg.workers <= 10 {
        worker_cnt = cfg.workers;
    }

    let mut p = Pool::new(worker_cnt);

    let listener = TcpListener::bind(cfg.addr.as_str()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let c = cfg.clone(); // 这里clone cfg是没有必要和有性能损失的，后续改进
        p.dispatch(Box::new(move || {
            handle_connection(stream, &c);
        }))
        .unwrap();
    }
}

fn handle_connection(mut stream: TcpStream, cfg: &Config) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let buffer = str::from_utf8(&buffer[..]).unwrap();
    let lines: Vec<&str> = buffer.split("\r\n").collect();
    let reqline: Vec<&str> = lines[0].split(" ").collect();
    assert_eq!(3, reqline.len());

    let rspfile = cfg.route(reqline[0], reqline[1]);

    let (status_line, filename) = if rspfile.is_empty() {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    } else {
        ("HTTP/1.1 200 OK\r\n\r\n", rspfile)
    };

    let filename = format!("html/{}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
