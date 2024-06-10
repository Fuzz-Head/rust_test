use std::thread::spawn;

pub fn test_threads () {
    let mut _x = 0u128;
    for i in 1..500_000{
      _x += i;  
    }
    println!("\x1b[38;2;100;100;255mMain thread finished a little bit of work.. lets go check worker threads.\x1b[0m");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000_000{
            x += i;  
        }
        println!("{x}");
    };

    println!("Worker thread created");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Worker thread comlpeted");

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, lets get out of here!");
            break;
        }
    }


    // handle.join();
    // handle2.join();
}