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
}
