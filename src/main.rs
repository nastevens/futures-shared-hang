use futures::future::{self, FutureExt};
use hyper::{Client, Uri};

#[tokio::main]
async fn main() {
    let n = std::env::args().nth(1).map(|s| s.parse::<usize>().unwrap()).unwrap_or(100);
    println!("Running with {} futures", n);
    hang(n).await;
}

async fn hang(n: usize) {
    env_logger::init();
    let mut futures = Vec::new();
    let client = Client::new();
    // let request = client.get(Uri::from_static("http://localhost:8000/test_file.bin"));
    let request = client.get(Uri::from_static("http://www.google.com"));
    let fut = async move { request.await.unwrap(); }.shared();
    for _ in 0..n {
        futures.push(fut.clone());
    }
    futures.push(fut);
    future::join_all(futures).await;
}
