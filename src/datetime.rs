use std::time::{Duration, Instant};
extern crate chrono;

pub fn test_stdtime() {
    let dur1 = Duration::from_secs(16);
    println!("Duration is {:?}", dur1.as_millis());

    let dur2 = Duration::from_millis(12500);
    //let dur3 = dur1.sub(dur2);
    let dur3 = dur1.checked_sub(dur2);

    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(200));
    println!("{}", now.elapsed().as_micros());
    println!("{:?}", now.elapsed());
}

pub fn test_chrono() {
    let utc_time = chrono::Utc::now();
    println!("{}", utc_time.format("%Y %b %d %H"));

    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Y %b %d %H"));

    let date1 = chrono::NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Fri);
    let unwrapped_date = date1.unwrap();
    println!("{}", unwrapped_date.format("Day of the year is: %j"));
}
