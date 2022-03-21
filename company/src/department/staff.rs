pub struct Staff {
    name: String,
    age: u8,
    salary: u32,
}

impl Staff {
    pub fn new(name: String, age: u8, salary: u32) -> Staff {
        Staff { name, age, salary }
    }

    pub fn info(&self) {
        println!(
            "Staff: name={}, age={}, salary={}",
            self.name, self.age, self.salary
        );
    }

    pub fn spawn(&self) -> Staff {
        Staff {
            name: self.name.clone(),
            age: self.age,
            salary: self.salary,
        }
    }
}
