fn show_match(v: Option<i32>) {
    println!("show_match");

    match v {
        Some(vv) => println!("Got value {}", vv),
        None => println!("Got none"),
    }
}

fn show_if_let(v1: Option<i32>, v2: i32, v3: Result<i32, ()>) {
    println!("show_if_let");

    if let Some(v1) = v1 {
        println!("v1 is {}", v1);
    } else if v2 > 100 {
        println!("v2 is {}", v2);
    } else if let Ok(v3) = v3 {
        println!("v3 is {}", v3);
    }
}

fn show_while_let() {
    println!("show_while_let");

    let mut v = vec![14, 15, 17];

    while let Some(vv) = v.pop() {
        println!("vv is {}", vv);
    }
}

fn show_for() {
    println!("show_for");

    let v = vec![14, 15, 17];

    for (idx, val) in v.iter().enumerate() {
        println!("idx={}, val={}", idx, val);
    }
}

fn show_let() {
    println!("show_let");

    let (x, y, z) = (1, 3, 5);
    let (r, s, _) = ('a', 'b', 'c');
    let (p, ..) = ("Homer", "Lisa", "Bart", "Maggie");

    println!("x={}, y={}, z={}, r={}, s={}, p={}", x, y, z, r, s, p);
}

fn show_fn((x, y): (i32, i32)) {
    println!("show_fn");

    println!("Got value: {}, {}", x, y);
}

fn show_closure<F>(f: F)
where
    F: FnOnce((u32, u32)) -> u32,
{
    println!("show_closure");

    println!("Got result={}, with arguments (99, 100)", f((99, 100)));
}

fn main() {
    show_match(Some(7));
    show_match(None);

    show_if_let(Some(17), 32, Ok(117));
    show_if_let(None, 119, Ok(117));
    show_if_let(None, 19, Ok(117));

    show_while_let();
    show_for();
    show_let();

    show_fn((73, 84));
    show_closure(|(v1, v2)| v1 + v2);
}
