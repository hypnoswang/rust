use std::env::{self, Args};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 && args.len() != 4 {
            return Err("Invalid arguments numbers");
        }

        if args[1].is_empty() || args[2].is_empty() || (args.len() == 4 && args[3].is_empty()) {
            return Err("Empty arguments");
        }

        let mut arg4 = "";
        if args.len() == 4 {
            arg4 = args[3].as_str();
        }

        let case_sensitive = get_case_sensitive(arg4)?;

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive,
        })
    }

    // 这里使用mut是因为调用next()方法
    pub fn new_with_closure(mut args: Args) -> Result<Config, &'static str> {
        // 跳过第1个参数
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("No queury specified"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("No file name specified"),
        };

        let case = match args.next() {
            Some(v) => v,
            None => String::new(),
        };

        let case_sensitive = Self::get_case_sensitive(case.as_str())?;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    fn get_case_sensitive(arg: &str) -> Result<bool, &'static str> {
        let mut case_sensitive = true;

        let mut env_val = String::new();
        if let Ok(v) = env::var("MINIGREP_CASE_SENSITIVE") {
            env_val = v;
        }

        let mut val = arg;
        if val.is_empty() {
            val = env_val.as_str();
        }

        if !val.is_empty() {
            if val.to_ascii_lowercase().eq_ignore_ascii_case("false") {
                case_sensitive = false;
            } else if val.to_ascii_lowercase().eq_ignore_ascii_case("true") {
                case_sensitive = true;
            } else {
                return Err("Invalid argument for case_sensitive, should be true or false(case insensitive)");
            }
        }

        Ok(case_sensitive)
    }
}

#[cfg(test)]
pub mod test {

    #[test]
    fn create_config_with_invalid_args() {
        let args = vec!["Hello".to_string()];
        let res = super::Config::new(&args);
        assert!(res.is_err(), "Should fail when args.len() is not 3 or 4");

        let args = vec![
            "cmd".to_string(),
            "Hello".to_string(),
            "".to_string(),
            "".to_string(),
        ];
        let res = super::Config::new(&args);
        assert!(res.is_err(), "Should fail when some args are empty");

        let args = vec![
            "cmd".to_string(),
            "Hello".to_string(),
            "".to_string(),
            "Haha".to_string(),
        ];
        let res = super::Config::new(&args);
        assert!(
            res.is_err(),
            "Should fail when the 3rd arg is not true or false (case-insensitive)"
        );
    }

    fn get_env_case_sensitive() -> Option<Result<bool, &'static str>> {
        if let Ok(v) = super::env::var("MINIGREP_CASE_SENSITIVE") {
            if !v.is_empty() {
                if v.to_ascii_lowercase().eq_ignore_ascii_case("false") {
                    return Some(Ok(false));
                } else if v.to_ascii_lowercase().eq_ignore_ascii_case("true") {
                    return Some(Ok(true));
                } else {
                    return Some(Err("Invalid argument for case_sensitive, should be true or false(case insensitive)"));
                }
            } else {
                return Some(Err("Invalid argument for case_sensitive, should be true or false(case insensitive)"));
            }
        }

        None
    }

    #[test]
    fn create_config_with_3args() {
        let env_val = get_env_case_sensitive();

        let args = vec![
            "cmd".to_string(),
            "Hello".to_string(),
            "Simpsons.txt".to_string(),
        ];

        if env_val.is_none() {
            let res = super::Config::new(&args)
                .expect("Should create succeeded if we have 3 correct args and no env_var is set");
            assert_eq!(
                res,
                super::Config {
                    query: "Hello".to_string(),
                    filename: "Simpsons.txt".to_string(),
                    case_sensitive: true,
                },
                "Shoud create a correct config object if we have 3 correct args and no env_var is set"
            );
        } else {
            let r = env_val.unwrap();
            if r.is_err() {
                assert!(
                    super::Config::new(&args).is_err(),
                    "Should fail when the env_var is invalid"
                );
            } else {
                let ev = r.unwrap();
                let res = super::Config::new(&args).expect(
                    "Should create succeeded if we have 3 correct args and env_var is set correctly",
                );
                assert_eq!(
                    res,
                    super::Config {
                        query: "Hello".to_string(),
                        filename: "Simpsons.txt".to_string(),
                        case_sensitive: ev,
                    },
                    "Shoud create a correct config object if we have 3 correct args and with the correct case_sensitive value from env_var"
                );
            }
        }
    }

    #[test]
    fn create_config_with_4args() {
        let args = vec![
            "cmd".to_string(),
            "Hello".to_string(),
            "Simpsons.txt".to_string(),
            "fAlSe".to_string(),
        ];

        let res = super::Config::new(&args).expect(
            "Should create succeeded if we have 4 correct args whatever the env_var is set or not",
        );
        assert_eq!(
            res,
            super::Config {
                query: "Hello".to_string(),
                filename: "Simpsons.txt".to_string(),
                case_sensitive: false,
            },
            "Shoud create a correct config object if we have 4 correct whatever the env_var is set or not"
        );
    }
}
