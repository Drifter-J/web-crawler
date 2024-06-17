mod crawler;
mod fetcher;
mod html_parser;
mod checker;

use checker::EnvArgsProvider;
use log::{error, info};

use crate::checker::extract_domain_from_args;
use crate::crawler::crawl;

#[tokio::main]
async fn main() {
    // Enable logging, since log defaults to silent
    std::env::set_var("RUST_LOG", "info");
    // Init the logger.
    env_logger::init();
    
    // Use the production environment arguments provider
    let env_args_provider = EnvArgsProvider;

    let start_time = chrono::Utc::now();
    match extract_domain_from_args(&env_args_provider) {
        Ok((host, domain_url)) => {
            info!("Starting crawl for domain: {}", host);
            crawl(&host, &domain_url).await;
            info!("Crawl completed for domain: {}", host);
        },
        Err(e) => {
            error!("{}", e);
            // TODO: Emit graphene metrics
            return;
        }
    }
    
    let end_time = chrono::Utc::now();
    let duration = end_time - start_time;
    info!("Crawl completed in: {}", duration);
}
