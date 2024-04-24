pub fn test_match_int() {
    let myage: u16 = 45;

    match myage {
        35 => {
            println!("yea you are old");
        }
        36..=100 =>{
            println!("You lived a good life");
        }

        101 | 111 => println!("You are lucky"),
        
        _ => {
            println!("listen this is default");
            println!("or else do it exhaustive");

        }
    }
}

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
