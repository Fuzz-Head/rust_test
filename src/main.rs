fn main() {
    println!("Hello, world!");
    test_closure_without_params();
    test_closure_with_params();
    test_wow_closures();
    test_long_closures();
    check_age();
}

fn check_age() {
    let age_to_drive = 16u8;

    println!("Enter the person's age:");
    let myinput = &mut String::from("");

    std::io::stdin().read_line(myinput).unwrap();

    let age = myinput.replace("/n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's license..");
    }
}

pub fn test_closure_without_params() {
    let add = || println!("Returning some text");
    add();
}
pub fn test_closure_with_params() {
    let add_more = |x: i32| println!("Return some number {}", x);
    add_more(123i32);
}

pub fn test_wow_closures() {
    // (x) wont work, but for closure |x| will work
    let add_more_more = |x, y, z| println!("return {}{}{}", x, y, z);
    add_more_more('A', 89u16, "Cool");
}

pub fn test_long_closures() {
    let sum = |x, y| {
        println!("x: {}, y: {}", x, y);
        x + y //the return statement never has semicolons;
    };
    let result = sum(312, -90);

    let print_result = || println!("The result is: {}", result);
    print_result();
}
