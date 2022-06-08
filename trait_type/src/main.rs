use std::fmt::Display;

pub trait Greeting {
    fn say_hello(&self);
}

struct Person<T: Display> {
    name: String,
    v: T,
}

impl<T> Person<T>
where
    T: Display,
{
    fn new(name: &str, v: T) -> Person<T> {
        Person {
            name: String::from(name),
            v,
        }
    }
}

impl<T> Greeting for Person<T>
where
    T: Display,
{
    fn say_hello(&self) {
        println!("Hi, i'am {}, {}", self.name, self.v);
    }
}

fn main() {
    let p = Person::new("Homer", 14);

    p.say_hello();
}
