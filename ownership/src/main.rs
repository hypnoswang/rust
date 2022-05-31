use std::collections::HashMap;

fn main() {
    let mut s = String::from("Hello");
    let s1 = &mut s;

    println!("s1 is {}", s1);

    let s2 = makeup(s1);
    // 这里不能同时打印s和s2，因为s1是mutable borrow，println中s是immutable borrow，冲突了
    //println!("s is {}, s2 is {}", s, s2);
    println!("s2 is {}", s2);

    // 至此为止，s1已经用完了，所以可以println可以immutable borrow了
    println!("s is {}", s);

    // 这里再打印s2是不行的，因为s2其实就是s1，是个mutable borrow
    // println!("s2 is {}", s2);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    let v1 = v[1];

    println!("the v1 is: {}", v1);

    let mut vs = Vec::new();
    vs.push(String::from("one"));
    vs.push(String::from("two"));
    vs.push(String::from("three"));

    let vs1 = &vs[1];
    println!("the vs1 is: {}", vs1);

    let s3 = "这个杀手不太冷".to_string();
    for i in s3.chars() {
        println!("--- {}", i);
    }
    for i in s3.bytes() {
        println!("--- {}", i);
    }

    let keys = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    let values = vec![1, 2, 3];

    let hm: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:#?}", hm);

    let one = hm.get(&String::from("one"));
    match one {
        Some(vx) => println!("#### {}", vx),
        None => println!("No such key"),
    }

    for (k, v) in hm {
        println!("key={}, val={}", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Homer"), 60);
    scores.insert(String::from("Bart"), 58);

    // 覆盖更新
    scores.insert(String::from("Homer"), 59);
    // 不存在则插入
    scores.entry(String::from("Lisa")).or_insert(99);
    println!("###-0 {:#?}", scores);

    // 不存在则插入，存在则减10分
    let hscore = scores.entry(String::from("Homer")).or_insert(59);
    *hscore -= 10;
    println!("###-1 {:#?}", scores);

    // 不存在则插入，存在则加10分
    let mscore = scores.entry(String::from("Maggie")).or_insert(90);
    *mscore += 10;
    println!("###-2 {:#?}", scores);

    let v = vec!["one".to_string(), "two".to_string()];
    get_ownership(v);
}

fn makeup(s: &mut String) -> &String {
    s.push_str(", World!");
    s
}

fn get_ownership(mut v: Vec<String>) {
    v.push(String::from("I'm the end"));
    println!("We got value {:#?}", v);
}
