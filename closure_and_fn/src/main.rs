fn return_closure(greeting: String) -> Box<dyn FnOnce(&str)> {
    Box::new(move |name| {
        println!("Hi, I am {}, {}", name, greeting);
    })
}

fn return_fn(f: fn(&str) -> String) -> fn(i32) -> String {
    let greeting = f("Homer");
    println!("return_fn: {}", greeting);

    fn rv(age: i32) -> String {
        format!("I am {} years old", age)
    }

    rv
}

fn say_hello(name: &str) -> String {
    format!("Haha, I am {}", name)
}

fn main() {
    let cp = return_closure(String::from("Welcome to Spring Field"));
    cp("Lisa");

    let fp = return_fn(say_hello);
    let rv = fp(9);
    println!("Haha, {}", rv);
}
