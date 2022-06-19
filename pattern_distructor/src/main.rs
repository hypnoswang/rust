#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

enum Distructor {
    WithNone,                      // 不绑定任何值
    WithStruct { x: f32, y: f32 }, // 绑定一个结构体
    WithValue(String),             // 绑定一个值
    WithTuple(i32, i32),           // 绑定一个元组
}

struct Mix {
    x: Point,
    y: Distructor,
}

fn dis_struct() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y), // 实际匹配成功这个分支
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn dis_enum() {
    let d = Distructor::WithStruct { x: 3.4, y: 8.2 };
    match d {
        Distructor::WithNone => println!("With none"),
        Distructor::WithStruct { x, y } => println!("Got struct, x={}, y={}", x, y), // 匹配成功
        Distructor::WithValue(v) => println!("Got value {}", v),
        Distructor::WithTuple(a, b) => println!("Got tuple {},{}", a, b),
    }
}

fn dis_mixed() {
    let m = Mix {
        x: Point { x: 3, y: 9 },
        y: Distructor::WithStruct { x: 13.1, y: 19.2 },
    };

    match m {
        Mix {
            x: Point { x: px, y: 9 },
            y: Distructor::WithStruct { x: dx, y: dy },
        } => println!("Got x.point.x={}, y.WithStruct.(x={},y={})", px, dx, dy),
        _ => println!("Some other value"),
    }
}

fn ignore_value(_: i32) {
    let _unused = 77;

    let (x, _, y, _, z) = (1, 2, 3, 4, 5);
    println!("Got values: {}, {}, {}", x, y, z);

    let s1 = Some("Hello".to_string());
    match s1 {
        Some(_v) => println!("Got a string"),
        _ => println!("Got nothing"),
    }
    //below code could not be compiled
    //println!("After match s2 is {:?}", s1);

    let s2 = Some("Hello".to_string());
    match s2 {
        Some(_) => println!("Got a string"),
        _ => println!("Got nothing"),
    }
    println!("After match s2 is {:?}", s2);
}

fn ignore_part() {
    let Point { x, .. } = Point { x: 321, y: 1101 };
    println!("Got x={}", x);

    let (head, .., tail) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("Got head={}, tail={}", head, tail);

    //below code could not be compiled
    //let (.., mid, ..) = (1, 2, 3, 4, 5);
    //println!("Got head={}, tail={}", head, tail);
}

fn match_guard() {
    let x = Some(7);
    match x {
        Some(v) if v <= 7 => println!("Got value less or equal to 7, {}", v),
        Some(v) => println!("Got value bigger than 7, {}", v),
        None => println!("Got nothing"),
    }

    let y = 10;
    match x {
        Some(n) if n == y => println!("Got x exactly equal to y: {}", n),
        _ => println!("Got something we don't care"),
    }

    enum Msg {
        Payload { cnt: u32 },
    }

    let m = Msg::Payload { cnt: 12 };
    match m {
        // 将值绑定到变量c上
        Msg::Payload { cnt: c @ 1..=12 } => println!("Got value less or equal to 12: {}", c),
        // 下边的分支不能被编译,因为{}内的只是模式匹配,没有值绑定
        //Msg::Payload { cnt: 13..=100 } => println!("Got value bigger than 12 but less or equal to
        //100: {}", cnt),
        Msg::Payload { cnt: 13..=100 } => {
            println!("Got value bigger than 12 but less or equal to 100")
        }
        // {}内是结构体解构的简写模式, 所以可以绑定值
        Msg::Payload { cnt } => println!("Got value we don't care: {}", cnt),
    }
}

fn main() {
    dis_struct();
    dis_enum();
    dis_mixed();
    ignore_value(32);
    match_guard();
}
