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
}

fn makeup(s: &mut String) -> &String {
    s.push_str(", World!");
    s
}
