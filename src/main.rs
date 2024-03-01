mod fs;

use std::{collections::LinkedList, ops};

const FULL_AGE: i8 = 18;

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
        self.age >= FULL_AGE
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

fn contains_comma(text: &str) -> bool {
    text.contains(',')
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

    let arr: [u8; 4] = [1, 2, 3, 4];

    for ae in arr {
        print!("{}", ae);
    }

    for i in 0..=10 {
        print!("{}", i);
    }

    let tupl: (u8, char, bool, &str) = (10, '1', true, "hi");
    println!("{}, {}, {}, {}", tupl.0, tupl.1, tupl.2, tupl.3);
    println!("{}", contains_comma("he,llo"));

    let vector: Vec<u32> = vec![10, 1, 5];

    for e in vector {
        println!("{}", e);
    }

    let mut linked_list: LinkedList<&str> = LinkedList::new();
    linked_list.push_back("hello");
    linked_list.push_back("hell12");
    let result = linked_list
        .iter()
        .map(|&e| e.chars().last())
        .filter(|&e| e.is_some())
        .filter(|e| e.unwrap_or('a').is_ascii_digit())
        .map(|e| e.unwrap_or('a').to_digit(10).unwrap_or(0))
        .fold(0, |acc, number| number + acc);
    println!("{}", result);
    println!("{:?}", fs::create_file("hello.txt"));
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
