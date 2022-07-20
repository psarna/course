use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello,");
    sleep(Duration::from_secs(1)).await;
    println!("\tworld!");
}
