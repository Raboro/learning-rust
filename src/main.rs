use std::ops;

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
        self.alive.unwrap_or(false)
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

impl ops::Add for Person {
    type Output = Person;

    fn add(mut self, other: Self) -> Self::Output {
        self.age += other.age;
        self.alive = self.alive.or(other.alive.or(None));
        self
    }
}

trait Dialog {
    fn display(&self);
}

struct WindowsDialog {}

impl Dialog for WindowsDialog {
    fn display(&self) {
        println!("Windows")
    }
}

struct LinuxDialog {}

impl Dialog for LinuxDialog {
    fn display(&self) {
        println!("Linux")
    }
}

struct MacDialog {}

impl Dialog for MacDialog {
    fn display(&self) {
        println!("Mac")
    }
}

struct DialogFactory {}

enum DialogName {
    Windows,
    Linux,
    Mac,
}

impl DialogFactory {
    fn create(name: DialogName) -> Box<dyn Dialog> {
        match name {
            DialogName::Windows => Box::new(WindowsDialog {}),
            DialogName::Linux => Box::new(LinuxDialog {}),
            DialogName::Mac => Box::new(MacDialog {}),
        }
    }
}

fn main() {
    DialogFactory::create(DialogName::Windows).display();
    DialogFactory::create(DialogName::Mac).display();
    DialogFactory::create(DialogName::Linux).display();

    let person: Person = Person {
        name: "Peter".to_string(),
        age: 19,
        alive: None,
    };
    println!("{}", person);
    person.greet();

    println!(
        "{}",
        person
            + Person {
                name: "alf".to_string(),
                age: 20,
                alive: Some(true)
            }
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn person_should_be_over_age() -> Result<(), String> {
        let p: Person = Person {
            name: "Peter".to_string(),
            age: 17,
            alive: None,
        };
        assert!(!p.is_full_age());
        Ok(())
    }
}
