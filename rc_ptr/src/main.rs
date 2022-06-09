use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
}

impl Person {
    fn new(n: &str) -> Person {
        Person {
            name: String::from(n),
        }
    }

    fn sing(&self, song: Rc<RefCell<String>>) {
        let mut my_song = song.borrow_mut();
        *my_song = format!("{}, {} finished!", my_song, self.name);

        println!("{} is singing: {}", self.name, my_song);

        eprintln!("The reference count of song is {}", Rc::strong_count(&song));
    }
}

fn family_sing(song: Rc<RefCell<String>>) {
    let families = vec![
        Person::new("Homer"),
        Person::new("Lisa"),
        Person::new("Bart"),
        Person::new("Maggie"),
    ];

    eprintln!(
        "At beginning in fn sing, the reference count of song is {}",
        Rc::strong_count(&song)
    );

    for p in families {
        p.sing(Rc::clone(&song));
    }

    eprintln!(
        "At the end in fn sing, the reference count of song is {}",
        Rc::strong_count(&song)
    );
}

fn main() {
    let song = Rc::new(RefCell::new("I love you, Lily".to_string()));
    eprintln!(
        "At beginning in main, the reference count of song is {}",
        Rc::strong_count(&song)
    );

    family_sing(Rc::clone(&song));

    eprintln!(
        "At the end in main, the reference count of song is {}",
        Rc::strong_count(&song)
    );
}
