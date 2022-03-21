pub use department::staff::Staff;
pub use department::Department;
use std::collections::HashMap;

pub mod department;

pub struct Company {
    name: String,
    ceo: String,
    departments: HashMap<String, Department>,
}

impl Company {
    pub fn new(name: String, ceo: String) -> Company {
        Company {
            name,
            ceo,
            departments: HashMap::new(),
        }
    }

    pub fn info(&self) {
        println!("Company: name={}, ceo={}", self.name, self.ceo);
        println!();
        println!();

        for (dn, d) in &self.departments {
            println!("Department: {}", dn);
            d.info();
        }
    }

    pub fn add_department(&mut self, d: &Department) {
        let nd = d.spawn();
        let name = nd.name.clone();
        self.departments.insert(name, nd);
    }
}
