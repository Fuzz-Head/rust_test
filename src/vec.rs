use std::net::ToSocketAddrs;

pub fn test_vec_int() {
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(20);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(90);

    println!("Size is {}", my_ints.len());
    println!("Capacity is {}", my_ints.capacity());

    println!("{:?}", my_ints);

    println!("The first element in vec is {}", my_ints[0]);
    println!("The second element in vec is {}", my_ints[1]);


    println!("Wow a slice: {:?}", &(&my_ints).as_slice()[0..=2]);
    println!("great get {:?}", my_ints.get(2));
}


pub fn test_vec_string() {
    let first_names= vec!["Billy", "Rachel", "Nancy", "Shannon"];

    for first_name in first_names.clone() {
        println!("processing... {}", first_name);
    }

    println!("{:?}", first_names);
}

#[derive(Debug)]
struct Car {
    manufacture: String,
    model: String,
}

pub fn test_vec_car (){
    let mut car_list: Vec<Car> = vec![];
    let mut car_list2: Vec<Car> = vec![];

    for _ in 1..=100u8 {
        car_list.push(Car{manufacture: "Porche".to_string(), model: "Panamera".to_string()});
    }

    for _ in 1..=100u8 {
        car_list2.push(Car{manufacture: "Hyundai".to_string(), model: "Sonata".to_string()});
    }

    car_list.append(&mut car_list2);
    car_list.insert(0, Car{manufacture: "Mercedes".to_string(), model: "S-Class".to_string()});
    car_list.remove(6);

    let keep = |e: &Car| {if e.manufacture == "Porche" { return true;} {return false;}};
    car_list.retain(keep);


    car_list.reserve(5000);

    println!("{:?}", car_list);
    println!("Size is {}", car_list.len());
    println!("Capacity is {}", car_list.capacity());

    println!("{:?}", car_list.get(0).unwrap());

    let mut input:String ="".to_string();
    //std::io::stdin().read_line(&mut input).expect("Something bad happened");

}
