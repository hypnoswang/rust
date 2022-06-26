pub mod ffi;

pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    pub fn display(&self) {
        println!("Person: name={}, age={}", self.name, self.age);
    }
}

pub struct Family {
    name: String,
    count: usize,
    members: Vec<Person>,
}

impl Family {
    pub fn new(name: &str, count: usize) -> Family {
        Family {
            name: name.to_string(),
            count,
            members: vec![],
        }
    }

    pub fn add_person(&mut self, p: Person) {
        if self.members.len() < self.count {
            self.members.push(p);
        }
    }

    pub fn display(&self) {
        println!("This is the {}:", self.name);
        for p in self.members.iter() {
            p.display();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut f = Family::new("Simpons", 5);

        let pnames = vec!["Homer", "Marge", "Bart", "Lisa", "Maggie"];
        for name in pnames {
            let p = Person::new(name, 14);
            f.add_person(p);
        }

        f.display();
    }
}
