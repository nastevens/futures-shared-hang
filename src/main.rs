use futures::future::FutureExt;
use futures::stream::{StreamExt, FuturesUnordered};
use std::future::Future;

#[tokio::main]
async fn main() {
    let n = std::env::args().nth(1).map(|s| s.parse::<usize>().unwrap()).unwrap_or(1);
    println!("Running with {} futures", n);
    let mut multiple = FuturesUnordered::new();
    for _ in 0..n {
        multiple.push(shared());
    }
    while let Some(_) = multiple.next().await {}
}

fn shared() -> impl Future<Output = ()> {
    tokio::spawn(async {}).then(|_| async {}).shared()
}
