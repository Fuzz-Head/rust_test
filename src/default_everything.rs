use std::default::{self, Default};
//#[derive(Default)]
#[derive(Debug)]
struct Person {
    first_name: FirstName, //String to FirstName
    last_name: String,
    age: u8,
    location: String,
}

impl Default for Person {
    fn default() -> Self {
        return Person {
            first_name: FirstName::default(),
            last_name: "Morris".to_string(),
            age: 35,
            location: "Phoenix".to_string(),
        };
    }
}

//custom for Strings and other types from our module

#[derive(Debug)]
struct FirstName(String);

impl Default for FirstName {
    fn default() -> Self {
        return FirstName("SomeName".to_string());
    }
}

fn new_person() -> Person {
    return Person {
        first_name: FirstName::default(),
        last_name: "Morries".to_string(),
        age: 35,
        location: "Phoenix".to_string(),
    };
} //sadly no standard conventions

pub fn test_defaults() {
    //one way of doing it but rust will complain if something is missing
    Person {
        first_name: FirstName::default(),
        last_name: "Holmes".to_string(),
        age: 45,
        location: "Utah".to_string(),
    };

    //other way is constructor above

    //keeping code to see output

    println!("Default String: {}", String::default());
    // let p1 = Person {
    //     first_name: "Jose".to_string(),
    //     ..default()
    // }; this is a nightly feature
    let p1 = Person::default();
    let p2 = Person::default();
    println!("Person 1 {:#?}, person 2 {:#?}", p1, p2);
}
