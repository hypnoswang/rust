fn main() {
    println!("Fibnacii list 0: ");
    fibonacii(0);
    println!();

    println!("Fibnacii list 1: ");
    fibonacii(1);
    println!();

    println!("Fibnacii list 2: ");
    fibonacii(2);
    println!();

    println!("Fibnacii list 5: ");
    fibonacii(5);
    println!();

    println!("Fibnacii list 10: ");
    fibonacii(10);
    println!();
}

fn fibonacii(n: u32) {
    if n == 0 {
        println!("1");
    } else if n == 1 {
        println!("1,1")
    } else {
        let mut pre = 1;
        let mut ppre = 1;

        print!("1,1");
        for _i in 2..(n + 1) {
            let cur = ppre + pre;
            ppre = pre;
            pre = cur;
            print!(",{}", cur);
        }
        println!();
    }
}
