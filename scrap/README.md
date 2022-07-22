# Web scraper

A very simple program for analyzing Web resources.

Example run:
```sh
[sarna@localhost scrap]$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `/home/sarna/repo/course/target/debug/scrape`
One by one:
Fetching from https://alternator-traces.sarna.dev
	Status of https://alternator-traces.sarna.dev: 200 OK
	Body of https://alternator-traces.sarna.dev processed!
Fetching from https://scyllabook.sarna.dev
	Status of https://scyllabook.sarna.dev: 200 OK
	Body of https://scyllabook.sarna.dev processed!
Fetching from https://compare-crates.sarna.dev
	Status of https://compare-crates.sarna.dev: 200 OK
	Body of https://compare-crates.sarna.dev processed!
Fetching from https://bio.sarna.dev
	Status of https://bio.sarna.dev: 200 OK
	Body of https://bio.sarna.dev processed!
Fetching from http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp
	Status of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp processed!
Fetching from http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol
	Status of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol processed!
Fetching from http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio
	Status of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio processed!
Fetching from http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest
	Status of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest processed!
Results: [4, 24, 6, 2, 6, 6, 6, 6]
Elapsed: 2163ms
----------
With join_all:
Fetching from https://alternator-traces.sarna.dev
Fetching from https://scyllabook.sarna.dev
Fetching from https://compare-crates.sarna.dev
Fetching from https://bio.sarna.dev
Fetching from http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp
Fetching from http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol
Fetching from http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio
Fetching from http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest
	Status of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp: 200 OK
	Status of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp processed!
	Body of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol processed!
	Status of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio processed!
	Status of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest processed!
	Status of https://bio.sarna.dev: 200 OK
	Body of https://bio.sarna.dev processed!
	Status of https://alternator-traces.sarna.dev: 200 OK
	Status of https://scyllabook.sarna.dev: 200 OK
	Body of https://alternator-traces.sarna.dev processed!
	Status of https://compare-crates.sarna.dev: 200 OK
	Body of https://compare-crates.sarna.dev processed!
	Body of https://scyllabook.sarna.dev processed!
Results: [4, 24, 6, 2, 6, 6, 6, 6]
Elapsed: 176ms
----------
With join_all (iter version):
Fetching from https://alternator-traces.sarna.dev
Fetching from https://scyllabook.sarna.dev
Fetching from https://compare-crates.sarna.dev
Fetching from https://bio.sarna.dev
Fetching from http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp
Fetching from http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol
Fetching from http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio
Fetching from http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest
	Status of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp processed!
	Status of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol: 200 OK
	Status of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio processed!
	Status of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest processed!
	Body of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol processed!
	Status of https://compare-crates.sarna.dev: 200 OK
	Body of https://compare-crates.sarna.dev processed!
	Status of https://alternator-traces.sarna.dev: 200 OK
	Body of https://alternator-traces.sarna.dev processed!
	Status of https://scyllabook.sarna.dev: 200 OK
	Status of https://bio.sarna.dev: 200 OK
	Body of https://bio.sarna.dev processed!
	Body of https://scyllabook.sarna.dev processed!
Results: [4, 24, 6, 2, 6, 6, 6, 6]
Elapsed: 170ms
----------
With FuturesUnordered:
Fetching from https://alternator-traces.sarna.dev
Fetching from https://scyllabook.sarna.dev
Fetching from https://compare-crates.sarna.dev
Fetching from https://bio.sarna.dev
Fetching from http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp
Fetching from http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol
Fetching from http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio
Fetching from http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest
	Status of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest: 200 OK
	Status of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=futures&crate2=tokio processed!
	Status of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=hyper&crate2=reqwest processed!
	Body of http://compare-crates.sarna.dev/?crate1=scylla&crate2=cassandra-cpp processed!
	Status of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol: 200 OK
	Body of http://compare-crates.sarna.dev/?crate1=tokio&crate2=smol processed!
	Status of https://bio.sarna.dev: 200 OK
	Body of https://bio.sarna.dev processed!
	Status of https://alternator-traces.sarna.dev: 200 OK
	Body of https://alternator-traces.sarna.dev processed!
	Status of https://compare-crates.sarna.dev: 200 OK
	Status of https://scyllabook.sarna.dev: 200 OK
	Body of https://compare-crates.sarna.dev processed!
	Body of https://scyllabook.sarna.dev processed!
Results: [6, 6, 6, 6, 2, 4, 6, 24]
Elapsed: 187ms
----------

```
