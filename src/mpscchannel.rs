use std::{sync::mpsc, time::Duration};

pub fn test_mpsc() {
    let (transmitter, receiver) = mpsc::channel::<u8>();
    // drop(receiver);
    let send_result = transmitter.send(100);

    println!("Send status {}", send_result.is_ok());
    transmitter.send(152);

    let receive_result = receiver.recv_timeout(Duration::from_millis(300));
    println!("Receive result is: {}", receive_result.is_ok());
    println!("Receive result is: {}", receive_result.unwrap());

    let receive_result = receiver.recv_timeout(Duration::from_millis(300));
    println!("Receive result is: {}", receive_result.is_ok());
    println!("Receive result is: {}", receive_result.unwrap());

    let processor_code = move || {
        println!("Starting processor thread...");
        let mut failed_count = 0u8;
        loop {
            println!("Attempting to receive message from channel...");
            let receive_result = receiver.recv_timeout(Duration::from_millis(300));
            if receive_result.is_ok() {
                println!("Received message: {}", receive_result.unwrap());
            } else {
                failed_count += 1;
            }
            if failed_count > 12 {
                println!("Aborting process... No work here");
                break;
            }
        }
    };
    //main thread creating and sending numbers in loop
    // processor thread is consuming
    for x in 1..6 {
        let send_result = transmitter.send(x);
        println!("Send status: {}", send_result.is_ok());
        std::thread::sleep(Duration::from_millis(200));
    }

    std::thread::spawn(processor_code).join();
}
