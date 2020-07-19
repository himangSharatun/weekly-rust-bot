// Scheduler, and trait for .seconds(), .minutes(), etc.
// use clokwerk::{Scheduler, TimeUnits};
// // Import week days and WeekDay
// // use clokwerk::Interval::*;
// use std::thread;
// use std::time::Duration;

mod scraper;

fn main() {
    let rust_weekly_scrapper =
        scraper::scraper::RustWeeklyScraper::new(String::from("https://this-week-in-rust.org/"));
    let urls = rust_weekly_scrapper.scrap_weekly_rust();
    println!("{}", urls);
}

// fn run_scheduler() {
//     // Create a new scheduler
//     let mut scheduler = Scheduler::new();
//     scheduler
//         .every(1.seconds())
//         .run(|| println!("Periodic task"));
//     for _ in 1..10 {
//         scheduler.run_pending();
//         thread::sleep(Duration::from_millis(10000));
//     }
//     // scheduler.watch_thread(Duration::from_millis(100));
// }
