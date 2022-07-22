use futures::future::join_all;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use std::future::Future;
use tokio::time::Instant;

async fn scrape_script_count(url: &str) -> usize {
    println!("Fetching from {}", url);
    let req = reqwest::get(url).await.unwrap();

    let status = req.status();
    println!("\tStatus of {}: {}", url, status);
    let body = req.text().await.unwrap();
    let words = body.split(|c: char| !c.is_alphabetic());
    println!("\tBody of {} processed!", url);
    words.filter(|word| word == &"script").count()
}

async fn scrape_counts_one_by_one(urls: &[&str]) {
    let mut results: Vec<usize> = Vec::new();
    for url in urls {
        results.push(scrape_script_count(url).await)
    }
    println!("Results: {:?}", results);
}

async fn scrape_counts_join_all(urls: &[&str]) {
    let mut futures = vec![];
    for url in urls {
        futures.push(scrape_script_count(url));
    }
    let results = join_all(futures).await;
    println!("Results: {:?}", results);
}

async fn scrape_counts_join_all_iter(urls: &[&str]) {
    let futures = urls.iter().map(|url| scrape_script_count(url));
    let results = join_all(futures).await;
    println!("Results: {:?}", results);
}

async fn scrape_counts_futures_unordered(urls: &[&str]) {
    let results = urls
        .iter()
        .map(|url| scrape_script_count(url))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<usize>>()
        .await;
    println!("Results: {:?}", results);
}

async fn measure(fut: impl Future<Output = ()>) {
    let start = Instant::now();
    fut.await;
    println!("Elapsed: {}ms", (Instant::now() - start).as_millis());
}

#[tokio::main]
async fn main() {
    let urls = [
        "https://alternator-traces.sarna.dev",
        "https://scyllabook.sarna.dev",
        "https://compare-crates.sarna.dev",
        "https://bio.sarna.dev",
        "http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp",
        "http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol",
        "http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio",
        "http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest",
        // "incorrect-url", // uncomment this one to trigger a panic
    ];

    println!("One by one:");
    measure(scrape_counts_one_by_one(&urls)).await;
    println!("----------");
    println!("With join_all:");
    measure(scrape_counts_join_all(&urls)).await;
    println!("----------");
    println!("With join_all (iter version):");
    measure(scrape_counts_join_all_iter(&urls)).await;
    println!("----------");
    println!("With FuturesUnordered:");
    measure(scrape_counts_futures_unordered(&urls)).await;
    println!("----------");
}
