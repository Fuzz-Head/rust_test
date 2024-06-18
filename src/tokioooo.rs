
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

pub async fn test(){
    //test_something().await;
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

    //data not accessible as it is moved to the spawn 
    //let best_lap_time = racer01.await;
    //println!("Best lap time was {}", racer01.best_lap_time);
    
    //let best_lap_time = racer02.await;
    //println!("Best lap time was {}", racer02.best_lap_time);
}

async fn test_something() {
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("Hello from Tokio");
}