fn main() {
    let s1 = String::from("Hello");
    println!("The 1st word of \"{}\" is: {}", s1, nth_word(1, &s1));
    println!("The 2nd word of \"{}\" is: {}", s1, nth_word(2, &s1));

    let s2 = String::from("Hello, welcome to rust world!!");
    println!("The 1st word of \"{}\" is: {}", s2, nth_word(1, &s2));
    println!("The 2nd word of \"{}\" is: {}", s2, nth_word(2, &s2));
    println!("The 5th word of \"{}\" is: {}", s2, nth_word(5, &s2));
    println!("The 7th word of \"{}\" is: {}", s2, nth_word(7, &s2));
}

fn nth_word(seq: usize, s: &String) -> &str {
    if seq <= 0 {
        return "";
    }

    let bytes = s.as_bytes();

    let mut cnt = 0;
    let mut last_idx: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            cnt += 1;
            if cnt == seq {
                if last_idx == 0 {
                    return &s[last_idx..i];
                } else {
                    return &s[(last_idx + 1)..i];
                }
            } else {
                last_idx = i;
            }
        }
    }

    //最后一个单词没有空格
    cnt += 1;
    if cnt == seq {
        if last_idx == 0 {
            return &s[last_idx..];
        } else {
            return &s[(last_idx + 1)..];
        }
    } else {
        //只可能 cnt < seq
        return "";
    }
}
