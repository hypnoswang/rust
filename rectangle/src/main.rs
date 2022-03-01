#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec1: &Rectangle) -> bool {
        self.width >= rec1.width && self.height >= rec1.height
    }

    fn make(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rec1 = Rectangle::make(37, 45);
    println!("rec1 is: {:#?}", rec1);

    let rec2 = Rectangle::make(24, 41);
    println!("rec2 is: {:#?}", rec2);

    let rec3 = Rectangle::make(44, 41);
    println!("rec3 is: {:#?}", rec3);

    println!("The area of rec1 is {}", rec1.area());
    println!("The area of rec2 is {}", rec2.area());
    println!("The area of rec3 is {}", rec3.area());

    println!("Rec1 can hold rec2: {}", rec1.can_hold(&rec2));
    println!("Rec1 can hold rec3: {}", rec1.can_hold(&rec3));
}
