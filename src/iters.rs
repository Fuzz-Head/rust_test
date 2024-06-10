pub fn test_iterators() {
    let fruit_list = vec!["Strawberry", "Mango", "BlueBerry", "Orange"];
    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brazil nuts"];

    let mut fruit_iter = fruit_list.iter();

    // for fruit in fruit_iter {
    //     println!("The fruit is {}", fruit);
    // }

    let first_item = fruit_iter.next();

    println!("The first item is: {}", first_item.unwrap());

    let aggregate_foods = fruit_list.iter().chain(&nut_list);

    let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

    println!("{:?}", all_foods);
    for food in aggregate_foods {
        println!("Eating {}", food);
    }

    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));

    let new_fruits = fruit_list_strings.map(|mut e| {
        e.push_str(" fruit");
        return e;
    });

    new_fruits.clone().for_each(|e| println!("{}", e));

    println!("{}", new_fruits.clone().last().unwrap()); //order of methods is important

    let mut step_by = new_fruits.clone().step_by(2);
    println!("Step: {}", step_by.next().unwrap());
    println!("Step: {}", step_by.next().unwrap());

    let first_names = vec!["Trevor", "Shannon", "James", "Jamie"];

    let first_names_string = first_names.iter().map(|e| String::from(*e));

    let last_names = vec!["Sullivan", "Jones", "Tanner", "Hansen"];

    let last_names_string = last_names.iter().map(|e| String::from(*e));

    let full_names = first_names_string.zip(last_names_string);

    //full_names.for_each(|e| println!("{} {}", e.0, e.1));

    // for (index, value) in full_names.enumerate() {
    //     println!("Index: {} Value: {} {}", index, value.0, value.1);
    // }

    full_names
        .skip(2)
        .take(1)
        .for_each(|e| println!("Did not skip {}", e.0));

    let foods = vec![("Cheese", 50), ("Ham", 55), ("Lettuce", 32)];

    //very intriguing but should be easy to read
    let food_quantity = foods.iter().fold(0u32, |a, e| a + e.1);
    println!("Total food quantity is {}", food_quantity);

    //foods.iter().peekable().next();
    let mut peekable = foods.iter().peekable();
    peekable.next();
    let food = peekable.peek();
    println!(
        "Peeking at food {} with quantity {}",
        food.unwrap().0,
        food.unwrap().1
    );
}
