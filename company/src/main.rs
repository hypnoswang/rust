use company::{self, Company};

fn main() {
    let mut c = Company::new(String::from("Styx Tech Ltd."), String::from("Hypnos"));

    let mut dept1 = company::Department::new(String::from("HR"), String::from("Homer"));
    let mut staffs1: Vec<company::Staff> = Vec::new();
    for i in 0..10 {
        let s = company::Staff::new(format!("HR-{}", i), i, (10 * i) as u32);
        staffs1.push(s);
    }
    dept1.add_staffs(&staffs1);

    c.add_department(&dept1);

    c.info();
}
