/*!
实现了在文件filename中搜索query的功能,既可以精确搜索,也可以忽略大小写搜索
+ do_search使用了迭代器的in的方式
+ do_search_iterly使用了迭代器适配器和消费适配器的综合用法
*/

use std::fs;
use std::io;

use super::config::Config;

fn do_search<'a>(q: &str, cnt: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut out = Vec::new();

    let lq = q.to_ascii_lowercase();

    for line in cnt.lines() {
        if !case_sensitive {
            let ll = line.to_ascii_lowercase();
            let q = lq.as_str();
            if ll.contains(q) {
                out.push(line);
            }
        } else {
            if line.contains(q) {
                out.push(line);
            }
        }
    }

    out
}

/**
综合使用了迭代器的高级用法,迭代器适配器和消费适配器

# Examples:
```rust
fn test_exactly_search() {
    let cnt = "Hello, Good Morning!
    Good morning!
    How are you today?
    I'm fine, thanks!";
    let query = "morning";

    assert_eq!(minigrep::search::do_search_iterly(query, cnt, true), vec!["Good morning!"]);
}
```
# Panics
# Errors
# Safety
*/
pub fn do_search_iterly<'a>(q: &str, cnt: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let lq = q.to_ascii_lowercase();

    let out = cnt
        .lines()
        .filter(|line| {
            if case_sensitive {
                line.contains(q)
            } else {
                line.to_ascii_lowercase().contains(lq.as_str())
            }
        })
        .collect();

    out
}

pub fn search_iterly(cfg: &Config) -> Result<Vec<String>, io::Error> {
    let cnt = fs::read_to_string(cfg.filename.as_str())?;

    let res = do_search_iterly(cfg.query.as_str(), cnt.as_str(), cfg.case_sensitive);
    let mut out = Vec::new();
    for l in res {
        out.push(String::from(l));
    }

    Ok(out)
}

pub fn start_search(cfg: &Config) -> Result<Vec<String>, io::Error> {
    let cnt = fs::read_to_string(cfg.filename.as_str())?;

    let res = do_search(cfg.query.as_str(), cnt.as_str(), cfg.case_sensitive);
    let mut out = Vec::new();
    for l in res {
        out.push(String::from(l));
    }

    Ok(out)
}

#[cfg(test)]
pub mod test {
    #[test]
    pub fn search_test() {
        let cnt = "Hello, Ketty,
        Good morning, class!
        Good morning, teacher!
        My name is Li Lei, what's your name?
        My name is Han Meimei.
        How are you?
        Fine, thank you , and you?
        I'm fine too!";
        let query = "fine";

        let res = super::do_search(query, cnt, true);
        assert_eq!(
            res.len(),
            1,
            "Should get only 1 line if case_sensitive is true"
        );

        let res = super::do_search(query, cnt, false);
        assert_eq!(
            res.len(),
            2,
            "Should get 2 lines if case_sensitive is false"
        );
    }
}
