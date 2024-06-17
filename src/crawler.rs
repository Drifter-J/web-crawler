use std::collections::HashSet;
use std::sync::Arc;
use hyper::Uri;
use log::{error, debug};
use tokio::sync::Mutex;
use tokio::task;

use crate::checker::parse_url;
use crate::fetcher::fetch_all_urls;

const THREADS: usize = 20;

/**
 * Start a new crawl for the given domain
 * @param domain_url: The URL of the domain to crawl
 */
pub async fn crawl(target_domain: &str, domain_url: &Uri) {
    // Arc/Mutex are used to safely share and mutate state across tasks. 
    let urls_to_visit = Arc::new(Mutex::new(vec![domain_url.clone()]));
    let visited_urls: Arc<Mutex<HashSet<Uri>>> = Arc::new(Mutex::new(HashSet::new()));

    let mut tasks = Vec::with_capacity(THREADS);

    for _ in 0..THREADS {
        let urls_to_visit_clone = urls_to_visit.clone();
        let visited_urls_clone = visited_urls.clone();
        let target_domain = target_domain.to_owned();
       
        let task = task::spawn(async move {
            crawler_worker(target_domain, urls_to_visit_clone, visited_urls_clone).await;
        });
        tasks.push(task);
    }

    // Await for all crawler threads to complete
    for task in tasks {
        if let Err(e) = task.await {
            error!("Failed to join task: {:?}", e);
        }
    }
}

async fn crawler_worker(
    target_domain: String,
    urls_to_visit: Arc<Mutex<Vec<Uri>>>,
    visited_urls: Arc<Mutex<HashSet<Uri>>>,
) {
    while let Some(url) = {
        let mut urls = urls_to_visit.lock().await;
        urls.pop()
    } {
        debug!("Spawning new crawler worker {}", url);
        let new_urls = fetch_all_urls(&target_domain, &url).await;
        let mut visited: tokio::sync::MutexGuard<'_, HashSet<Uri>> = visited_urls.lock().await;
        let mut urls = urls_to_visit.lock().await;
        for url in new_urls {
            if let Ok(parsed_url) = parse_url(&url) {
                if !visited.contains(&parsed_url) {
                    urls.push(parsed_url.clone());
                    visited.insert(parsed_url);
                }
            } else {
                error!("Invalid URL: {}", url);
            }
        }
    }
}