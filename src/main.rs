trait Greeter {
    fn greet(&self);
}

struct Person {
    name: String,
    age: i8,
    alive: Option<bool>,
}

impl Person {
    fn is_full_age(&self) -> bool {
        self.age >= 18
    }

    fn am_i_alive(&self) -> bool {
        match self.alive {
            Some(a) => a,
            None => false,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} with age: {} and is {}full age and i am {}alive",
            self.name,
            self.age,
            if self.is_full_age() { "" } else { "not " },
            if self.am_i_alive() { "" } else { "not " }
        )
    }
}

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello my name is {}", self.name);
    }
}

fn main() {
    let person: Person = Person {
        name: "Peter".to_string(),
        age: 19,
        alive: None,
    };
    println!("{}", person);
    person.greet();
}
