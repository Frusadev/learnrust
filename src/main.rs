use std::time;

#[tokio::main]
async fn main() {
    println!("Start...");

    tokio::spawn(async {
        for i in 1..=5 {
            println!("Task A: {i}")
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    });

    for i in 1..=5 {
        println!("Task B: {i}");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    tokio::time::sleep(time::Duration::from_secs(3)).await;
}
