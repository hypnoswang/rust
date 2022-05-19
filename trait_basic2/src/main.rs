trait Citizen {
    fn pay_wages(&mut self, wages: i32) -> i32;
    fn name(&self) -> String;
    fn self_introduce(&self) {
        println!("Hi, I am {}", self.name());
    }
}

struct Homer {
    account: i32,
}

struct Lisa {
    money_box: i32,
}

impl Citizen for Homer {
    fn name(&self) -> String {
        String::from("Homer")
    }

    fn pay_wages(&mut self, wages: i32) -> i32 {
        self.account += wages - 1000;
        self.account
    }
}

impl Citizen for Lisa {
    fn name(&self) -> String {
        String::from("Lisa")
    }

    fn pay_wages(&mut self, wages: i32) -> i32 {
        self.money_box += wages;
        self.money_box
    }

    fn self_introduce(&self) {
        println!("Hi, everyone, {} here", self.name());
        println!("I'm Lisa, my father is Homer, my mother is Maggie");
    }
}

fn pay(c: &mut impl Citizen, wages: i32) {
    println!(
        "Pay {} to {}, now he/she has {}",
        wages,
        c.name(),
        c.pay_wages(wages)
    );
}

fn main() {
    let mut h = Homer { account: 250 };
    let mut l = Lisa { money_box: 30 };

    pay(&mut h, 3000);
    pay(&mut l, 500);

    h.self_introduce();
    l.self_introduce();
}
