use company::{self, Company};

fn main() {
    let mut c = Company::new(String::from("Styx Tech Ltd."), String::from("Hypnos"));

    let mut dept1 = company::Department::new(String::from("HR"), String::from("Homer"));
    for i in 0..10 {
        let s = company::Staff::new(format!("HR-{}", i), i + 20, (10 * i) as u32);
        dept1.add_staff(&s);
    }

    c.add_department(&dept1);

    c.info();
}
