use hyper::Uri;
use url::Url;

pub trait ArgsProvider {
    fn args(&self) -> Vec<String>;
}

/** 
 * Production implementation that uses the actual environment arguments
 */
pub struct EnvArgsProvider;

impl ArgsProvider for EnvArgsProvider {
    fn args(&self) -> Vec<String> {
        std::env::args().collect()
    }
}

/** 
 * Extracts the domain from command line arguments
 * Returns the domain or an error message
 * @param args_provider: The provider for command line arguments
 */
pub fn extract_domain_from_args<T: ArgsProvider>(args_provider: &T) -> Result<(String, Uri), String> {
    let args = args_provider.args();
    if args.len() <= 1 {
        return Err("Please Provide 'One' Web Domain URL in the Terminal.".to_string());
    }

    let binding = parse_url(&args[1]);
    let domain_url = match binding {
        Ok(url) => url,
        Err(e) => return Err(format!("Invalid URL: {}", e)),
    };

    let host =  match domain_url.host() {
        Some(host) => host.to_string(),
        None => return Err("Invalid URL: No host found in URL.".to_string())
    };

    Ok((host, domain_url))
}


/**
 * Check if the URL is valid
 * @param url: The URL to check
 */
fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok() && url.contains("https://")
}

/**
 * Parse the URL string and return the Uri
 * @param url_str: The URL string to parse
 */
pub fn parse_url(url_str: &str) -> Result<Uri, String> {
    if !is_valid_url(url_str) {
        let err_msg = format!("The Given Uri is not valid: {}", url_str);
        return Err(err_msg);
    }
    match url_str.parse::<Uri>() {
        Ok(url) => Ok(url),
        Err(e) => {
            let err_msg = format!("Failed to parse URL to Uri: {}, {}", url_str, e);
            Err(err_msg)
        }
    }
}

// For testing, to create a mock provider
pub struct MockArgsProvider {
    mock_args: Vec<String>,
}

impl MockArgsProvider {
    pub fn new(mock_args: Vec<String>) -> Self {
        Self { mock_args }
    }
}

impl ArgsProvider for MockArgsProvider {
    fn args(&self) -> Vec<String> {
        self.mock_args.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_success() {
        let url_str = "https://example.com";
        assert!(parse_url(url_str).is_ok());
    }

    #[test]
    fn test_parse_url_failure() {
        let url_str = "invalid url";
        assert!(parse_url(url_str).is_err());
    }
    
    #[test]
    fn test_extract_domain_from_correct_args() {
        let mock_args_provider = MockArgsProvider::new(vec![
            "program_name".to_string(),
            "https://example.com".to_string(),
        ]);

        let result = extract_domain_from_args(&mock_args_provider);
        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn test_extract_domain_from_wrong_args() {
        let mock_args_provider = MockArgsProvider::new(vec![
            "program_name".to_string(),
        ]);

        let result = extract_domain_from_args(&mock_args_provider);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), "Please Provide 'One' Web Domain URL in the Terminal.");
    }

    #[test]
    fn test_extract_domain_from_wrong_url() {
        let mock_args_provider = MockArgsProvider::new(vec![
            "program_name".to_string(),
            "//".to_string(),
        ]);

        let result = extract_domain_from_args(&mock_args_provider);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), "Invalid URL: The Given Uri is not valid: //");
    }

    #[test]
    fn test_extract_domain_from_wrong_scheme() {
        let mock_args_provider = MockArgsProvider::new(vec![
            "program_name".to_string(),
            "http://www.snapchat.com".to_string(),
        ]);

        let result = extract_domain_from_args(&mock_args_provider);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), "Invalid URL: The Given Uri is not valid: http://www.snapchat.com");
    }
}