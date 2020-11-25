use std::thread::sleep;
use std::time::Duration;
use chrono::Local;

async fn hello(str: &str, delays: u64) {
    let t1 = Local::now();
    println!("start: {} ( {} )", str, t1.format("%Y-%m-%d %H:%M:%S").to_string());
    sleep(Duration::from_secs(delays));
    println!("{} - duration: {}", str, delays);
    let t2 = Local::now();
    println!("end: {} ( {} )", str, t2.format("%Y-%m-%d %H:%M:%S").to_string());
}

async fn test() {
    tokio::spawn(async {
        hello("task 1", 2).await;
    });

    tokio::spawn(async {
        hello("task 2", 2).await;
    });

    tokio::spawn(async {
        hello("task 3", 5).await;
    });

    tokio::spawn(async {
        hello("task 4", 5).await;
    });

    tokio::spawn(async {
        hello("task 5", 3).await;
    });

    tokio::spawn(async {
        hello("task 6", 4).await;
    });

    tokio::spawn(async {
        hello("task 7", 5).await;
    });

    tokio::spawn(async {
        hello("task 8", 5).await;
    });
}

#[tokio::main]
async fn main() {
    test().await;
}