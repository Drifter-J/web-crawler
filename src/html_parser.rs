use html5ever::{tendril::StrTendril, tokenizer::{BufferQueue, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts}};
use log::{debug, info};
use url::Url;
use std::rc::Rc;

/**
 * Parse HTML and get all URLs
 * in case of error, return the error
 * @param target_domain: The domain to extract URLs for
 * @param html_content: A string slice that holds the HTML content to be parsed
 */
pub fn get_urls(target_domain: &str, html_content: &str) -> Result<Vec<String>, std::io::Error> {
    debug!("Parsing HTML content {}", html_content);

    // Convert the String to a Tendril
    let tendril: StrTendril = html_content.into();

    // Create a BufferQueue from the Tendril
    let mut buffer_queue: BufferQueue = BufferQueue::new();
    buffer_queue.push_back(tendril);

    let mut tok = Tokenizer::new(Sink::new(target_domain), TokenizerOpts::default());

    // Feed the HTML content to the tokenizer
    let _ = tok.feed(&mut buffer_queue);
    tok.end();

    let urls = tok.sink.urls;

    info!("Extracted {} URLs", urls.len());
    Ok(urls)
}

/**
 * Token sink to collect URLs from HTML content
 */
struct Sink {
    target_domain: String,
    urls: Vec<String>,
}

impl Sink {
    // Initialize Sink with the target domain extracted from the starting URL
    pub fn new(target_domain: &str) -> Self {
        Sink {
            target_domain: target_domain.to_owned(),
            urls: Vec::new(),
        }
    }

    /**
     * Helper function to determine if the URL belongs to the target domain
     * @param url: The URL to check
     */
    fn is_same_domain(&self, url: &str) -> bool {
        // Process absolute URLs.
        match Url::parse(url) {
            Ok(parsed_url) => {
                parsed_url.domain() == Some(&self.target_domain) &&
                parsed_url.cannot_be_a_base() == false &&
                parsed_url.scheme() == "https"
            },
            Err(_) => false,
        }
    }

    /**
     * Check if the URL is a media or document URL
     * @param url: The URL to check
    */
    fn is_media_or_document_url(url: &str) -> bool {
        let url = url.to_lowercase();
        let media_extensions = [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".mp3", ".mp4", ".wav", ".avi", ".mov"];
        let document_extensions = [".pdf", ".doc", ".docx", ".ppt", ".pptx", ".xls", ".xlsx", ".txt"];
        media_extensions.iter().any(|ext| url.ends_with(ext)) || document_extensions.iter().any(|ext| url.ends_with(ext))
    }
}

impl TokenSink for Sink {
    type Handle = Rc<()>;

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<Rc<()>> {
        match token {
            Token::TagToken(tag) => {
                if tag.kind == html5ever::tokenizer::TagKind::StartTag
                    && tag.name.as_ref() == "a"
                {
                    // Iterate over attributes to find href
                    for attr in tag.attrs.iter() {
                        if attr.name.local.as_ref() == "href" {
                            let mut url_str = attr.value.to_string();
                            if url_str.starts_with("/") {
                                // Resolve the relative URL to an absolute one
                                url_str = format!("https://{}{}", self.target_domain, url_str);
                            }
                            if self.is_same_domain(&url_str) && !Sink::is_media_or_document_url(&url_str) {
                                self.urls.push(url_str.clone());
                                info!("URL found: {}", url_str);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        TokenSinkResult::Continue
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_urls() {
        let target_domain = "example.com";
        let html_content = r#"
            <html>
                <body>
                    <a href="https://example.com/page1">Page 1</a>
                    <a href="https://example.com/page2">Page 2</a>
                    <a href="/page3">Page 3 Relative Path</a>
                    <a href=`#page4`>Fragment Identifier</a>
                    <a href="https://notexample.com/page5">External</a>
                    <a href="http://example.com/page6">Non-HTTPS</a>
                </body>
            </html>
        "#;

        let result = get_urls(target_domain, html_content);

        assert!(result.is_ok());
        let urls = result.unwrap();
        // All https URLs from example.com should be included
        // Relative URL should be included
        assert_eq!(urls.len(), 3);
        assert!(urls.contains(&"https://example.com/page1".to_string()));
        assert!(urls.contains(&"https://example.com/page2".to_string()));
        assert!(urls.contains(&"https://example.com/page3".to_string()));
    }

    #[test]
    fn test_is_same_domain() {
        let sink = Sink::new("example.com");

        // Test URL that matches the domain and is HTTPS
        assert!(sink.is_same_domain("https://example.com/page1"));

        // Test URL that does not match the domain
        assert!(!sink.is_same_domain("https://notexample.com/page1"));

        // Test URL that matches the domain but is not HTTPS
        assert!(!sink.is_same_domain("http://example.com/page1"));

        // Test URL that is malformed
        assert!(!sink.is_same_domain("ht://example.com/page1"));
    }

    #[test]
    fn test_is_media_or_document_url() {
        assert!(Sink::is_media_or_document_url("https://example.com/image.jpg"));
        assert!(Sink::is_media_or_document_url("https://example.com/doc.pdf"));
        // Not a media/document
        assert!(!Sink::is_media_or_document_url("https://example.com/page.html"));
    }
}
