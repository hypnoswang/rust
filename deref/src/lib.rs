use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;

#[derive(Debug, PartialEq)]
pub struct MyBox<T> {
    pub v: T,
}

impl<T> MyBox<T> {
    pub fn new(v: T) -> MyBox<T> {
        MyBox { v }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.v
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop MyBox with value");
    }
}

pub fn increase(n: &mut i32, i: i32) {
    *n += i;
}

#[cfg(test)]
mod tests {
    use super::MyBox;

    #[test]
    fn it_works() {
        let mut a = MyBox::new(7);

        super::increase(&mut a, 3);

        assert_eq!(10, *a);
    }
}
