pub fn test_match() {
    let y: u8 = 45;
    let x: u8 = 12;

    match y {
        1 => println!("wow"),

        30..=45 if x == 12 => println!("that a perfect match if I have ever seen"),

        30..=45 if x != 12 => println!("just missed"),
        _ => println!("default"),
    }
}

pub fn test_match_string() {
    let car_manufacturer: &str = "Porche";

    match car_manufacturer {
        "Hyundai" => "Hyundai it is!",
        "Porche" => "Porche",
        _ => "This manufacturer is not supported:(",
    };
}

pub fn test_match_array() {
    let prices = [30000, 50000, 90000, 120000];

    match prices[0..=3] {
        [30000, 50000] => println!("You have reasonably priced"),
        [30000, 50000, ..] => println!("variety"),
        [120000] => println!("something"),
        _ => println!("You do not have reasonably priced"),
    }
}
