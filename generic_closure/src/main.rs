trait Greeting {
    fn get_name(&self) -> &str;
    fn hello(&self);
}

struct Family<T: Greeting> {
    name: String,
    members: Vec<T>,
}

impl<T: Greeting> Family<T> {
    fn new(n: &str) -> Family<T> {
        Family {
            name: String::from(n),
            members: vec![],
        }
    }

    fn add(&mut self, m: T) {
        self.members.push(m);
    }

    fn greeting(&self) {
        self.members.iter().for_each(|m| {
            println!("Hi, I am {}, one of {}", m.get_name(), self.name);
            m.hello();
        });
    }
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
        }
    }
}

impl Greeting for Person {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn hello(&self) {
        println!("Hello, Everyone!");
    }
}

fn main() {
    let homer = Person::new("Homer");
    let lisa = Person::new("Lisa");

    let mut f: Family<Person> = Family::new("Simpsons");
    f.add(homer);
    f.add(lisa);

    f.greeting();
}
