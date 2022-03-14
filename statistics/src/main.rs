use std::collections::HashMap;

fn main() {
    let v = vec![1, 4, 89, 34, 4, 29, 98];

    println!("The mean of v is {}", mean(&v));
    println!("The median of v is {}", median(&v));
    println!("The mode of v is {}", mode(&v));
}

fn mean(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in nums {
        sum += i;
    }

    let cnt = nums.len();

    sum / (cnt as i32)
}

fn median(nums: &Vec<i32>) -> i32 {
    let mut v2: Vec<i32> = Vec::new();
    for i in nums {
        v2.push(*i);
    }

    v2.sort();

    let idx = nums.len() / (2 as usize);
    v2[idx]
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in nums {
        let cnt = map.entry(*i).or_insert(1);
        *cnt += 1;
    }

    let mut max = 0;
    let mut max_k = 0;
    for (k, v) in &map {
        if *v > max {
            max = *v;
            max_k = *k;
        }
    }

    max_k
}
