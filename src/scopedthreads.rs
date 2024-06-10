use std::thread;

struct Person {
    first_name: String,
}

pub fn test_scoped_thread() {
    let age = 34;
    let person1 = Person{first_name: String::from("Train")};

    //let print_age = move || -> for age in thread::spawn 
    let print_age = || {
        println!("Inside the child closure");
        println!("Your age is {age}");
        println!("Your name is: {}", &person1.first_name);
    };

    thread::scope(|scope|{
        scope.spawn(print_age);             //.join() not required unless panic
    });
    //let _result = thread::spawn(print_age).join();

    println!("Your age is {age}");
    println!("Your name is: {}", person1.first_name);
    
    println!("Finished printing age");
    

}