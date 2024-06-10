use std::time::{Duration, Instant};

use chrono::NaiveDate;
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

    unwrapped_date.iter_days().take(4).for_each(|d| println!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2024, 366);
    println!("{}", date2.unwrap().format("%A"));

    //if format varies then program panics
    let birthday = NaiveDate::parse_from_str("2024|||09||12", "%Y|||%m||%d");
    //println!("{:#?}", birthday.err().unwrap());
    println!("{:?}", birthday.ok().unwrap());
}
