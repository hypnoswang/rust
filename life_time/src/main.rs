struct Player<'a> {
    name: &'a str,
}

impl<'a> Player<'a> {
    fn new(name: &str) -> Player {
        Player { name }
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_longest<'b>(&'b self, b: &'b str) -> &'b str {
        if self.name.len() > b.len() {
            self.name
        } else {
            b
        }
    }
}

fn main() {
    let p = Player::new("Homer");

    println!("The name is {}", p.get_name());
    println!(
        "The longest between {} and Lias is {}",
        p.get_name(),
        p.get_longest("Lisa")
    );
}
