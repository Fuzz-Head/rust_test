use std::sync::mpsc;

use crate::structs::{create_vehicle, test_create_person};

pub mod datetime;
pub mod default_everything;
pub mod dynamic_dispatch;
pub mod filesystem;
pub mod hashmaps;
pub mod hashsets;
pub mod helpers;
pub mod iters;
pub mod matchtest;
pub mod mpscchannel;
pub mod mutexes;
pub mod operator_overloading;
pub mod optiontest;
pub mod reqwests;
pub mod scopedthreads;
pub mod structs;
pub mod threads;
pub mod traits;
pub mod try_serde;
pub mod vec;
pub mod asyncs;
pub mod tokioooo;


struct Person {
    first_name: String,
    last_name: String,
}

//#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
//make fn main() -> async fn main()
fn main() {
    println!("Hello, Rust people!");

    /*let my_result = helpers::namehelpers::get_full_name("Troy", "Lemur");
    println!("Hello, this is {0}", my_result);

    let new_age = helpers::privatefns::get_age_plus_5(12);
    println!("Age {0}", new_age);
    println!("Hello, world!");

    // let new_age = helpers::privatefns::get_age_plus_5(12);
    // println!("Age {0}", new_age);
    // test_closure_one();
    // test_closure_two();
    // test_closure();
    // matches::test_match_int();

    test_closure_without_params();
    test_closure_with_params();
    test_wow_closures();
    test_long_closures();
    matchtest::test_match_int();
    matchtest::test_match();
    matchtest::test_match_array();
    matchtest::test_match_string();
    let result = optiontest::test_options();
    println!("{}", result.unwrap());

    let more_result = optiontest::test_option_string();
    check_age();

    println!("{}", more_result.unwrap());

    let game_result = optiontest::test_option_chartype();

    if game_result.is_some() {
        println!("User has selected a character");
        println!("{}", game_result.unwrap().to_string());
    } else {
        println!("Character not selected");
    }

    //structs::new_person();
    structs::test_create_person();
    create_vehicle();
    structs::create_vehicle_tuple();

    traits::create_person();

    vec::test_vec_int();
    vec::test_vec_string();
    vec::test_vec_car();

    hashmaps::test_hashmaps();

    hashsets::test_hashsets();

    iters::test_iterators();

    datetime::test_stdtime();

    datetime::test_chrono();

    //threads::test_threads();
    //threads::spawn_thread();
    scopedthreads::test_scoped_thread();

    mutexes::test_mutex();

    mpscchannel::test_mpsc();
    */

    // filesystem::create_dir();
    // filesystem::create_files();
    // //filesystem::remove_dir();
    // filesystem::read_files();

    // try_serde::test_serde();
    // try_serde::test_deser();

    //dynamic_dispatch::dynamic_dispatch_traits();

    //default_everything::test_defaults();

    // operator_overloading::overloading_test();
    // operator_overloading::grocery_bill_generation();

    //reqwests::reqwests_test();

    //asyncs::test_data();
    //tokioooo::test();

    /* for tokio 
   let racer01 = F1Racer::new();
   let mut racer02 = F1Racer::new();

   racer02.lap_times.pop();
   racer02.lap_times.pop();
   racer02.lap_times.push(310);
   racer02.lap_times.push(440);
   racer02.name = "George Russlle".to_string();

   let handle1 = tokio::task::spawn(racer01);
   let handle2 = tokio::task::spawn(racer02);

   loop {
       if handle1.is_finished() && handle2.is_finished() {
           println!("All racers have finished!");
           break;
       }
       std::thread::sleep(std::time::Duration::from_millis(300));
   }
  */
}

/*------------------------------------------------tokio 

struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u16,
    lap_times: Vec<u16>,
}

impl F1Racer {
    fn new() -> F1Racer {
        return F1Racer{name: "Charles Lecere".to_string(), laps: 5, completed_laps: 0, best_lap_time: 600, lap_times: vec![323, 450, 410, 340, 400]};
    }

    fn do_lap(&mut self){
        println!("{} Doing a new lap...", self.name);
        let lap_time = self.lap_times.pop();

        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }
        self.completed_laps += 1;
    }
}

impl std::future::Future for F1Racer {
    type Output = u16;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("Thread assigned is ID: {:?}", std::thread::current().id());
        if self.completed_laps < self.laps{
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        println!("{} has completed all laps!", self.name);
        println!("Best lap time for {} was {}", self.name, self.best_lap_time);
        
        return std::task::Poll::Ready(self.best_lap_time);
    }
}

*/

fn check_age() {
    let age_to_drive = 16u8;

    println!("Enter the person's age:");
    let myinput = &mut String::from("");

    std::io::stdin().read_line(myinput).unwrap(); //discard the returned result with unwrap()

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

#[allow(dead_code)]

fn ill() {}
pub fn test_closure_one() {
    let add_static = || println!("{}+{}={}", 1, 2, 3);
    add_static();
}

pub fn test_closure_two() {
    let add = |x, y| {
        println!("{}, {}", x, y);
        x + y
    };
    let answer = add(112, -345);
    println!("{}", answer);
}

pub fn test_closure() {
    let mut p1 = Person {
        first_name: "Hillary".to_string(),
        last_name: "Clinton".to_string(),
    };

    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
    change_name("Yesthat");
    println!("{}", p1.last_name);
}
