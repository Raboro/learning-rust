struct Person {
    name: String,
    age: i8,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} with age: {}", self.name, self.age)
    }
}

fn main() {
    let person: Person = Person {
        name: "Peter".to_string(),
        age: 19,
    };
    println!("{}", person);
}
