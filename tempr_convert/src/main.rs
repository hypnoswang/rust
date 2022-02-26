fn main() {
    let c = f2c(32.0);
    println!("f 32.0 to c is: {}", c);

    let c = f2c(212.0);
    println!("f 212.0 to c is: {}", c);

    let c = f2c(100.0);
    println!("f 100.0 to c is: {}", c);

    let f = c2f(0.0);
    println!("c 0 to f is: {}", f);

    let f = c2f(100.0);
    println!("c 100 to f is: {}", f);

    let f = c2f(36.5);
    println!("c 36.5 to f is: {}", f);
}

fn f2c(f: f32) -> f32 {
    let c = (f - 32.0) * 5.0 / 9.0;
    c
}

fn c2f(c: f32) -> f32 {
    let f = c * 9.0 / 5.0 + 32.0;
    f
}
