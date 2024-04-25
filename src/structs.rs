use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]

enum VehicleColor {
    Silver,
    Red,
    Yellow,
    Green,
    Black,
    White,
}

#[derive(Debug)]
struct VehicleTuple(String, String, u16);

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle {
            manufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 1990,
            color: VehicleColor::Black,
        };
        return new_vehicle;
    }
}

struct Person<'p> {
    pub first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
    miles_walked: u32,
}

// impl Person {
//     fn walk_miles(&mut self, miles: u32) {
//         self.miles_walked += miles;
//     }
// }

pub fn create_vehicle_tuple() {
    let myvehicletuple = new_vehicle_tuple();
    println!(
        "Manufacturer: {0}, model: {1}",
        myvehicletuple.0, myvehicletuple.1
    );
    println!("{:?}", new_vehicle_tuple());
}
fn new_vehicle_tuple() -> VehicleTuple {
    return VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2017);
}

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Porche".to_string(),
        model: "911".to_string(),
        year: 1991,
        color: VehicleColor::Red,
    };
    v1.paint(VehicleColor::Black);
    return v1;
}

pub fn create_vehicle() {
    //let myvehicle = new_vehicle();
    let mut myvehicle = Vehicle::create_vehicle();
    myvehicle.paint(VehicleColor::White);
    println!("{:?}", myvehicle);
}

fn new_person() -> Person<'static> {
    let p1 = Person {
        first_name: Cell::from("Dennis"),
        last_name: "Lloyd".to_string(),
        birth_month: 6,
        birth_year: 1991,
        visited_europe: false,
        miles_walked: 0,
    };
    p1.first_name.set("Greg");
    return p1;
}

pub fn test_create_person() {
    let myperson = new_person();
    println!(
        "First name: {0}, last name: {1}, birth month: {2}, birth year: {3}, europe?: {4}, walked: {5}",
        myperson.first_name.get(),
        myperson.last_name,
        myperson.birth_month,
        myperson.birth_year,
        myperson.visited_europe,
        myperson.miles_walked,
    );
}
