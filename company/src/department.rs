pub mod staff;

use staff::Staff;

pub struct Department {
    pub name: String,
    manager: String,
    staffs: Vec<Staff>,
}

impl Department {
    pub fn new(name: String, manager: String) -> Department {
        Department {
            name,
            manager,
            staffs: Vec::new(),
        }
    }

    pub fn info(&self) {
        println!();
        println!("Department: name={}, manager={}", self.name, self.manager);
        for s in &self.staffs {
            s.info();
        }
        println!();
    }

    pub fn add_staffs(&mut self, rookies: &Vec<Staff>) {
        for r in rookies {
            let nr = r.spawn();
            self.staffs.push(nr);
        }
    }

    // 暂时不知道引用和具体类型之间怎么转换,只能先手工copy了
    pub fn spawn(&self) -> Department {
        let mut nd = Department::new(self.name.clone(), self.manager.clone());
        for s in self.staffs.iter() {
            let ns = s.spawn();
            nd.staffs.push(ns);
        }

        nd
    }
}
