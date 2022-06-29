#![allow(dead_code)]

use family::Family;
use std::fmt::Display;
use std::ops::Deref;

trait Worker {
    fn work(&self);
}

trait Farmer {
    fn rest();
    fn work(&self);
}

struct MigrantWorker;
impl MigrantWorker {
    fn rest() {
        println!("Never rest as I have a family to feed");
    }

    fn work(&self) {
        println!("Find some part-time works to do...");
    }
}

impl Worker for MigrantWorker {
    fn work(&self) {
        println!("Hard work, go go go......");
    }
}

impl Farmer for MigrantWorker {
    fn rest() {
        println!("Have a cigrette and then go on......")
    }

    fn work(&self) {
        println!("Day by day, plant and harvest, work as a famer......");
    }
}

trait AgriculturalScientist: Farmer {
    fn research(&self);
}

struct Investigator;

impl AgriculturalScientist for Investigator {
    fn research(&self) {
        self.work();
        println!("Find out why the plants grow so slow......");
    }
}

impl Farmer for Investigator {
    fn rest() {
        println!("Have a tea...");
    }

    fn work(&self) {
        println!("Plant more vegetables......");
    }
}

struct FmtFamily(Family);
impl Display for FmtFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is the dispplay implementation for FmtFamily")
    }
}

impl Deref for FmtFamily {
    type Target = Family;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let i = MigrantWorker {};

    i.work();
    MigrantWorker::rest();

    Worker::work(&i);
    Farmer::work(&i);

    <MigrantWorker as Farmer>::rest();

    let xiaowang = Investigator {};
    xiaowang.research();

    let ff = FmtFamily(Family::new("Simpsons", 3));
    println!("The display trait output: {}", ff);
}
