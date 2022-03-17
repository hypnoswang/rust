enum CharType {
    Vowel(char),
    Consonant(char),
    Invalid,
}

fn get_char_type(c: char) -> CharType {
    if !c.is_ascii_alphabetic() {
        return CharType::Invalid;
    }

    let vowels: [char; 5] = ['a', 'o', 'e', 'i', 'u'];
    let consonants: [char; 21] = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];

    let c1 = c.to_ascii_lowercase();

    if vowels.contains(&c1) {
        return CharType::Vowel(c);
    } else if consonants.contains(&c1) {
        return CharType::Consonant(c);
    }

    CharType::Invalid
}

fn pig_latin(input: &String) -> String {
    if input.is_empty() {
        return String::new();
    }

    let first_char = input.chars().next();
    let mut tail = String::new();

    let mut first = true;
    for c in input.chars() {
        if first {
            first = false;
            continue;
        }

        tail.push(c);
    }

    match first_char {
        Option::Some(c) => {
            let ct = get_char_type(c);
            match ct {
                CharType::Vowel(_) => {
                    //println!("vowel: {}", cc);
                    input.clone() + "-hay"
                }
                CharType::Consonant(cc) => {
                    //println!("consonant: {}", cc);
                    let mut post = String::from("-");
                    post.push(cc);
                    post.push('a');
                    post.push('y');
                    tail + &post
                }
                CharType::Invalid => {
                    //println!("invalid");
                    input.clone()
                }
            }
        }
        Option::None => input.clone(),
    }
}

fn main() {
    let s1 = String::from("Oppo");
    let s2 = String::from("Vivo");
    let s3 = String::from("啥都不是");
    let s4 = String::from("12adbc");
    let s5 = String::from(" haha");
    let s6 = String::from("cat");

    println!("The pig-latin of {} is {}", s1, pig_latin(&s1));
    println!("The pig-latin of {} is {}", s2, pig_latin(&s2));
    println!("The pig-latin of {} is {}", s3, pig_latin(&s3));
    println!("The pig-latin of {} is {}", s4, pig_latin(&s4));
    println!("The pig-latin of {} is {}", s5, pig_latin(&s5));
    println!("The pig-latin of {} is {}", s6, pig_latin(&s6));
}
