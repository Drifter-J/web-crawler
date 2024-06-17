use hyper::Uri;

use log::debug;
use log::{info, error};

use crate::html_parser::get_urls;

/**
 * Fetche the content of a URL, Returns either String containing html content or Error
 * @param url: The URL to fetch
 */
async fn fetch_url_async(url: &Uri) -> Result<String, reqwest::Error> {
    debug!("Attempt to fetch URL: {}", url.to_string());
    // Create a client instance
    let client = reqwest::Client::new();
    match client.get(url.to_string())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await {
        Ok(resp) => {
            let body = resp.text().await?;
            info!("Successfully fetched URL: {}", url);
            Ok(body)
        },
        Err(e) => {
            error!("Error fetching URL: {}", e);
            Err(e)
            // TODO: Emit Graphene Metrics
        }
    }
}

/**
 * Fetches all URLs found within the content of the given 
 * @param target_domain: The domain to extract URLs for
 * @param url: The URL to fetch
 */
pub async fn fetch_all_urls(target_domain: &str, url: &Uri) -> Vec<String> {
    info!("Fetching all URLs from: {}", url);
    match fetch_url_async(url).await {
        Ok(html_content) => {
            match get_urls(&target_domain, &html_content) {
                Ok(urls) => {
                    urls
                },
                Err(e) => {
                    error!("Failed to get URLs from HTML content: {}", e);
                    // Emit Graphene Metrics
                    Vec::new()
                }
            }
        },
        Err(e) => {
            error!("Failed to fetch URL {}: {}", url, e);
            // Emit Graphene Metrics
            Vec::new()
        }, 
    }
 }

 #[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_fetch_url_async() {
        let _m: mockito::Mock = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html></html>")
            .create();
    
        let url = Uri::try_from(&server_url()).unwrap();
        let result = fetch_url_async(&url).await;
    
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<html></html>");
    }
}
