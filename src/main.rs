#[tokio::main]
async fn main() {
    for _ in 1..1001 {
        tokio::task::spawn_blocking(|| async move { fun().await }).await;
    }
}

async fn fun() {
    println!("rusty 😲")
}
