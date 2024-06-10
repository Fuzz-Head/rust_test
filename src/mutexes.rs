use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope, sleep, spawn};
use std::time::Duration;
pub fn test_mutex() {
    let score = Mutex::new(0u32);

    // let unlocked_score = score.lock();
    // let mut data = unlocked_score.unwrap();
    // data.add_assign(5);

    // println!("{:?}", data);
    // drop(data);
    let myfun = || {
        loop {
            println!("Thread 1 is waiting for a mutex lock...");
            let guard = score.try_lock();

            // drop(data);
            // panic!("Thread 2 panic");
            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..=10 {
                    data.add_assign(i);
                    println!("Thread 1 is adding {i}");
                }
                break;
            }
            sleep(Duration::from_millis(300));
        }
    };

    let myfun2 = || {
        println!("Thread 2 is waiting for a mutex lock...");
        let mut data = score.lock().unwrap();
        for i in 1..=10 {
            data.add_assign(i);
            println!("Thread 2 is adding {i}");
            sleep(Duration::from_millis(300));
        }
    };

    _ = scope(|s| {
        //no join() as they wait for each other
        // let handle1 = s.spawn(myfun).join();
        // let handle2 = s.spawn(myfun2).join();
        s.spawn(myfun2);
        s.spawn(myfun);

        //because we dont panic
        // if handle2.is_err() {
        //     println!("Thread 2 had an error");
        // }
    });

    println!("{:?}", score.lock().unwrap());
}
