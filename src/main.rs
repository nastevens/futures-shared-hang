use futures::future::FutureExt;
use futures::stream::{StreamExt, FuturesUnordered};
use std::fs;

#[tokio::main]
async fn main() {
    let n = std::env::args().nth(1).map(|s| s.parse::<usize>().unwrap()).unwrap_or(100);
    println!("Running with {} futures", n);
    let mut multiple = FuturesUnordered::new();
    for _ in 0..n {
        multiple.push(hang(n));
    }
    while let Some(_) = multiple.next().await {}
}

async fn hang(n: usize) {
    let mut futures = FuturesUnordered::new();
    let fut = async move { 
        tokio::task::spawn_blocking(move || {
            fs::read("test_file.bin").map(Vec::into_boxed_slice).unwrap()
        }).await.unwrap()
    }.shared();
    for _ in 0..n {
        futures.push(fut.clone());
    }
    futures.push(fut);
    while let Some(_) = futures.next().await {}
}
