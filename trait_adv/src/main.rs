trait Fish {
    fn swim(&self);
}

trait Bird {
    fn name(&self) -> String;
    fn fly(&self);
}

trait FeatherCare {
    fn care(&mut self);
}

trait EggSale {
    fn sell(&mut self);
}

#[derive(Clone)]
struct Chicken {}

#[derive(Clone)]
struct Carp {}

struct Farm<T> {
    items: Vec<T>,
}

impl<T: Clone> Farm<T> {
    fn new(is: &Vec<T>) -> Farm<T> {
        let mut new_vec: Vec<T> = Vec::new();
        for i in is {
            new_vec.push(i.clone());
        }

        Farm { items: new_vec }
    }
}

impl<T: Clone + Fish> Farm<T> {
    fn purify_water(&mut self) {
        println!("Start cleaning the water ......");
        println!("The water now is clean");
    }

    fn free_alive(&self) {
        for i in &self.items {
            i.swim();
        }
    }
}

impl<T: Clone + Bird> Farm<T> {
    fn egg_manage(&mut self) {
        println!("Start collecting eggs ......");
        println!("All eggs are collected");
    }

    fn free_sport(&self) {
        for i in &self.items {
            i.fly();
        }
    }
}

impl<T: Clone + Bird> EggSale for Farm<T> {
    fn sell(&mut self) {
        println!("Start selling eggs ......");
        println!("All eggs are sold");
    }
}

impl Bird for Chicken {
    fn name(&self) -> String {
        String::from("chichen")
    }

    fn fly(&self) {
        println!("I'm a chiken, I'm trying to flay ......");
        println!("Oh my God, I can't fly so far");
    }
}

impl Fish for Carp {
    fn swim(&self) {
        println!("I'm a fish, I'm trying to swim ......");
        println!("Enn~~, I am good at swimming");
    }
}

// blancket implementations
impl<T: Bird> FeatherCare for T {
    fn care(&mut self) {
        println!("I'm {}, my feature is so beautiful", self.name());
    }
}

fn main() {
    let mut chs: Vec<Chicken> = Vec::new();
    for i in 0..10 {
        println!("Just use variable i={}", i);
        let ch = Chicken {};
        chs.push(ch);
    }
    let mut chf = Farm::new(&chs);
    chf.egg_manage();
    chf.sell();
    chf.free_sport();

    let mut carps: Vec<Carp> = Vec::new();
    for i in 0..10 {
        println!("Just use variable i={}", i);
        let c = Carp {};
        carps.push(c);
    }
    let mut cf: Farm<Carp> = Farm::new(&carps);
    cf.purify_water();
    cf.free_alive();

    let mut ch1 = Chicken {};
    ch1.care();
}
