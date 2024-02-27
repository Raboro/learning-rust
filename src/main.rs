struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn is_full_age(&self) -> bool {
        self.age >= 18
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} with age: {} and is {}full age",
            self.name,
            self.age,
            if self.is_full_age() { "" } else { "not" }
        )
    }
}

fn main() {
    let person: Person = Person {
        name: "Peter".to_string(),
        age: 19,
    };
    println!("{}", person);
}
