use std::fs::{self, File};
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let r = File::open("./hello.txt");

    let mut f = match r {
        Ok(f) => {
            println!("Open file succeeded");
            f
        }
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let s = read_file1().expect("read_file1 failed");
    println!("read_file1 succeeded, s={}", s);

    let s = read_file2().expect("read_file2 failed");
    println!("read_file2 succeeded, s={}", s);

    let s = read_file3().expect("read_file3 failed");
    println!("read_file3 succeeded, s={}", s);

    let s = read_file4().expect("read_file4 failed");
    println!("read_file4 succeeded, s={}", s);
}
