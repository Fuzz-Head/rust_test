//#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
}

fn new_person() -> Person {
    let p1 = Person{first_name: "Dennis".to_string(), last_name: "Lloyd".to_string(), birth_month: 6, birth_year: 1991, visited_europe: false};
    return p1;
}

pub fn test_create_person() {
    let myperson = new_person();
    println!("First name: {0}, last name: {1}, birth month: {2}, birth year: {3}, europe?: {4}",
     myperson.first_name, myperson.last_name, myperson.birth_month, myperson.birth_year, myperson.visited_europe);
}