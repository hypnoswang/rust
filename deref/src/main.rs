use deref::{self, MyBox};

fn main() {
    let mut a = MyBox::new(8);

    deref::increase(&mut a, 12);

    println!("After increase: {:#?}", *a);
}
