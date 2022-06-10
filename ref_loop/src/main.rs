use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

trait Worker {
    fn work(&mut self);
}

trait Human {
    fn dinner(&mut self);
}

struct Person {
    name: String,
    company: Weak<RefCell<Company>>,
    family: Weak<RefCell<Family>>,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            company: Weak::new(),
            family: Weak::new(),
        }
    }

    fn employed(&mut self, c: Rc<RefCell<Company>>) {
        self.company = Rc::downgrade(&c);
    }

    fn home(&mut self, f: Rc<RefCell<Family>>) {
        self.family = Rc::downgrade(&f);
    }

    fn self_intro(&self) {
        let c = self.company.upgrade().unwrap();
        let f = self.family.upgrade().unwrap();

        println!(
            "Hi, I am {}, one of {}. I work for {}",
            self.name,
            f.borrow().get_host(),
            c.borrow().get_name()
        );
    }
}

impl Worker for Person {
    fn work(&mut self) {
        println!("{} is working......", self.name);
    }
}

impl Human for Person {
    fn dinner(&mut self) {
        println!("{} is having dinner......", self.name);
    }
}

struct Company {
    name: String,
    staff: Vec<Rc<RefCell<Person>>>,
}

impl Company {
    fn new(name: &str) -> Company {
        Company {
            name: String::from(name),
            staff: vec![],
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn employ(&mut self, p: Rc<RefCell<Person>>) {
        self.staff.push(p);
    }

    fn work(&mut self) {
        println!("{} working time....", self.name);
        for p in self.staff.iter() {
            p.borrow_mut().work();
        }
    }
}

struct Family {
    host: String,
    members: Vec<Rc<RefCell<Person>>>,
}

impl Family {
    fn new(name: &str) -> Family {
        Family {
            host: String::from(name),
            members: vec![],
        }
    }

    fn get_host(&self) -> &str {
        &self.host
    }

    fn welcome(&mut self, p: Rc<RefCell<Person>>) {
        self.members.push(p);
    }

    fn dinner(&self) {
        println!("{} dinner time....", self.host);
        for p in self.members.iter() {
            p.borrow_mut().dinner();
        }
    }
}

fn main() {
    let qax = Rc::new(RefCell::new(Company::new("qax")));
    let simpsons = Rc::new(RefCell::new(Family::new("Simpsons")));

    let homer = Rc::new(RefCell::new(Person::new("Homer")));
    let lisa = Rc::new(RefCell::new(Person::new("Lisa")));

    //这里所有的borrow_mut调用返回的都是临时变量, 所以, 多次调用borrow_mut不违反借用规则
    qax.borrow_mut().employ(Rc::clone(&homer));
    homer.borrow_mut().employed(Rc::clone(&qax));
    simpsons.borrow_mut().welcome(Rc::clone(&homer));
    homer.borrow_mut().home(Rc::clone(&simpsons));

    qax.borrow_mut().employ(Rc::clone(&lisa));
    lisa.borrow_mut().employed(Rc::clone(&qax));
    simpsons.borrow_mut().welcome(Rc::clone(&lisa));
    lisa.borrow_mut().home(Rc::clone(&simpsons));

    qax.borrow_mut().work();
    simpsons.borrow_mut().dinner();

    homer.borrow().self_intro();
    lisa.borrow().self_intro();
}
