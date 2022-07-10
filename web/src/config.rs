use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub addr: String,
    pub workers: usize,

    routes: Vec<Route>,
}

#[derive(Deserialize, Debug, Clone)]
struct Route {
    method: String,
    uri: String,
    rsp_file: String,
}

impl Config {
    pub fn read_cfg(file: &str) -> Result<Config, &'static str> {
        let res = fs::read_to_string(file);
        match res {
            Ok(content) => Self::read_cfg_from_str(&content),
            Err(e) => {
                eprintln!("Read file {} failed: {}", file, e.to_string());
                Err("Read config file failed")
            }
        }
    }

    fn read_cfg_from_str(cnt: &str) -> Result<Config, &'static str> {
        let res = toml::from_str(cnt);
        match res {
            Ok(cfg) => Ok(cfg),
            Err(e) => {
                eprintln!("Parse config failed: {}", e.to_string());
                Err("Parse config file failed")
            }
        }
    }

    pub fn route(&self, m: &str, u: &str) -> &str {
        for r in &self.routes {
            if m == r.method && u == r.uri {
                return &r.rsp_file;
            }
        }

        ""
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_toml() {
        let cnt = r#"
        name = "rust_web_server"
        addr = "0.0.0.0:8080"

        [[routes]]
        method = "GET"
        uri = "/"
        rsp_file = "hello.html"

        [[routes]]
        method = "GET"
        uri = "/homer"
        rsp_file = "homer.html"
            "#;

        let res = Config::read_cfg_from_str(cnt);
        assert_eq!(res.is_ok(), true);

        println!("The config is: {:#?}", res.unwrap());
    }
}
