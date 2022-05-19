trait Animal {
    fn name(&self) -> String;
    fn eat(&mut self, amount: i32) -> i32;
    fn scream(&self);
    fn fly(&self) {
        println!("I'm {}, I'm flying", self.name());
    }
}

struct Dog {
    age: i32,
}

struct Hawk {
    weight: i32,
}

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("dog")
    }

    fn eat(&mut self, amount: i32) -> i32 {
        self.age += amount;
        self.age / 100
    }

    fn scream(&self) {
        println!("Wang Wang Wang!!");
    }

    fn fly(&self) {
        println!("I'm dog, I can't fly");
    }
}

impl Animal for Hawk {
    fn name(&self) -> String {
        String::from("hawk")
    }

    fn eat(&mut self, amount: i32) -> i32 {
        self.weight += amount;
        self.weight / 10
    }

    fn scream(&self) {
        println!("Jiu Jiu Jiu!!");
    }
}

fn feed(a: &mut impl Animal, amt: i32) {
    println!("After eat {}, {} is {}", amt, a.name(), a.eat(amt));
}

fn fly(a: &impl Animal) {
    a.fly();
}

fn main() {
    let mut dog = Dog { age: 0 };
    let mut hawk = Hawk { weight: 0 };

    feed(&mut dog, 1227);
    feed(&mut hawk, 986);

    fly(&dog);
    fly(&hawk);

    dog.scream();
    hawk.scream();
}
