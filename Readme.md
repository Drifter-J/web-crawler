# Web Crawler

**Author:** Jaye Park

## Problem Statement

We'd like you to write a simple web crawler in a programming language you're familiar with. Given a starting URL, the crawler should visit each URL it finds on the same domain. It should print each URL visited, and a list of links found on that page. The crawler should be limited to one subdomain - so when you start with `https://helsing.ai/`, it would crawl all pages on the helsing.ai website, but not follow external links, for example to facebook.com.

Please do not use frameworks like scrapy or go-colly which handle all the crawling behind the scenes or someone else's code. You are welcome to use libraries to handle things like HTML parsing.

There are no hard requirements as far as I know. I haven’t asked about network bandwidth but I just assume there is no specification for this.

## Tenet

I focus on these things specifically.
- Memory Safety
- Performance
- Concurrency in a Single Machine

Note: horizontal scalability (for distributed system) is not but will be discussed in the Open questions section

## Tech stack

### What language to choose?

**Rust**

**Side note:** I was thinking about c++ (which I am more familiar with) and myhtml but then since I am focusing more on memory safety and efficient concurrency as highlighted in the tenet, chose to write the program on Rust. 

**Why?** 
- Take advantage of html5ever which is one of the fastest and safest HTML parser written in Rust.
- Strong performance for concurrent jobs with memory safety (ref: https://pkolaczk.github.io/memory-consumption-of-async/) 
- ~~Selfish reason (wanted to learn something new 😛)~~

## Running the Rust Web Crawler

This guide will walk you through the steps needed to run the Rust-based web crawler project.

## Prerequisites

1. **Rust and Cargo**: Make sure Rust and Cargo are installed on your system. They can be installed from the [official Rust website](https://www.rust-lang.org/tools/install).

2. **External Dependencies**: The project relies on several external crates. Ensure your `Cargo.toml` file includes all necessary dependencies.

- env_logger: https://github.com/env-logger-rs/env_logger - environment-based logging
- log: https://github.com/rust-lang/log - logging facade
- html5ever: https://github.com/servo/html5ever - HTML Parser library
- url: https://docs.rs/url/latest/url/ - Helper library that helps us to parse url
- hyper: https://hyper.rs/ - low-level HTTP library written in Rust that allows us to connect to the internet 
- tokio: https://github.com/tokio-rs/tokio - asynchronous runtime
- reqwest:https://github.com/seanmonstar/reqwest - higher-level HTTP client functionalities
- futures: https://github.com/rust-lang/futures-rs - foundational asynchronous programming constructs
- mockito: https://docs.rs/mockito/latest/mockito/ - a library for generating and delivering HTTP mocks in Rust 

## Setting Up the Project

### 1. Build the Rust Project

```bash
cd web_crawler
cargo build
```

### 2. Execute the Program

**Disclaimer**: The current web-crawler only supports *https* scheme. 

```bash
cargo run -- https://example.com
```

Replace https://example.com with the URL you wish to crawl.

The below is the result of execution.
```
[2024-02-28T10:56:04Z INFO  web_crawler] Starting crawl for domain: www.scrapingbee.com
[2024-02-28T10:08:32Z INFO  web_crawler::fetcher] Fetching all URLs from: https://www.scrapingbee.com/blog/
[2024-02-28T10:08:32Z INFO  web_crawler::html_parser] Extracted 61 URLs
[2024-02-28T10:08:32Z INFO  web_crawler::fetcher] Successfully fetched URL: https://www.scrapingbee.com/blog/
[2024-02-28T10:08:32Z INFO  web_crawler::html_parser] URL found: https://www.scrapingbee.com/
[2024-02-28T10:08:32Z INFO  web_crawler::html_parser] URL found: https://www.scrapingbee.com/#pricing
```

### 3. Execute the Test

```bash
cargo test
```

The below is the test result.
```
running 8 tests
test checker::tests::test_extract_domain_from_wrong_args ... ok
test checker::tests::test_extract_domain_from_wrong_url ... ok
test checker::tests::test_parse_url_failure ... ok
test checker::tests::test_extract_domain_from_wrong_scheme ... ok
test checker::tests::test_parse_url_success ... ok
test checker::tests::test_extract_domain_from_correct_args ... ok
test html_parser::tests::test_get_urls ... ok
test fetcher::tests::test_fetch_url_async ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parser_test.rs (target/debug/deps/parser_test-e54df4b365de602b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## System Architecture

### High Level Design

There are  parts: 
- **Main** - call checker and crawler.
- **Checker** - Simple checks the validity of the target domain url.
- **Crawler** - Thread will be allocated in the crawler for processing the content. It also prints the visited URLs. This checks if the url was already visited.
- **Fetcher** - This is a dedicated fetching component.
- **HTML Parser** - Each crawler thread can parse the HTML content of the pages it fetches and extract URLs. html5parser is used in this component.

## To Do
- Create a thread pool so that it doens't overwhlem the systme.
- Implement detailed logging for debugging and monitoring the crawler's progress. Include metrics like the number of pages crawled, the number of unique URLs found, and resource usage statistics.
- Better error handling logic like Error Propagation.
- Adhere to robots.txt guidelines and implement rate limiting to prevent server overload.
- Support HTTP as well
- Gracefullly Timing Out
- Rely on `CONTENT_TYPE` for excluding non html

## Open questions
- What if we want this service to be accessible by multiple users from different parts of the world?
- What if the domain is huge or there are multiple domains that we want to crawl so that we want to scale up horizontally?
  - Use of cloud services for auto scaling & global distribution (deploy your service across multiple geographical regions) & easy storage solution (e.g. MongoDB, Cassandra)
  - Hashing
  - Introduce a message queue (e.g. RabbitMQ) where the concurrent tasks are allocated to the workers so that when a node is available for a crawling task, they can pick it up. This allows an efficient load balancing, dynamic scaling of workers as well as fault tolerance.      
  - Distributed Caching (e.g. Memcache like Redis)
  - I don’t think web crawling requires high realtime accuracy, hence we can introduce a distributed caching layer to store and share crawl states, such as which URLs have been visited, across multiple nodes.

- A lot of websites use rate limiting mechanisms to protect their websites, wouldn’t that be an obstacle for this service to thrive? How can we bypass rate limiting by web domain?
  - Distributing requests across multiple IP addresses using proxy & VPN can technically reduce the chance of hitting rate limits. But this might not be fully legal, we should abide by the Terms of Services of the websites.
