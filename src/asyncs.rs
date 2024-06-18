#![allow(unused_variables)]
use std::future::Future;

use futures::{join, select, pin_mut};
use futures::future::FutureExt;

//one way of doing things 
async fn get_number() -> u8 {
    println!("running");//not shown in absence of futures 
    return 8;
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    return 10
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(75));
    12
}

//second way of doing things 
// fn get_number_two() -> impl Future<u8> {
//     return 16;
// }

pub fn test_data() {
    //fuse() only for futures to be fused normal leave as is
    let num1 = get_number().fuse();
    let num2 = num2().fuse();
    let num3 = num3().fuse();

    pin_mut!(num1, num2, num3);
    
    //this or result2 both cant be used together 
    // let result = smol::block_on(
    //     async{
    //         join!(num1, num2, num3)
    //     }
    // );

    let result2 = smol::block_on(
        async{
            loop {
                select! {
                    x = num1 => println!("num1 is completed {}", x),
                    x = num2 => println!("num2 is completed {}", x),
                    x = num3 => println!("num3 is completed {}", x),  
                    complete => {
                        println!("All futures finished, Breaking out of loop"); 
                        break;
                    }      
                }
            }
        }
    );
    // println!("Number: {}", result);
    println!("Numbers {:?}", result2);
}