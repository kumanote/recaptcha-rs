pub mod error;
mod response;

use http::{Request, StatusCode};
use hyper::body::Buf;
use hyper_rustls::HttpsConnector;
use response::RecaptchaResponse;
use std::collections::HashSet;
use std::io::Read;

pub use error::Error;

/// Verify a recaptcha user response
pub async fn verify(secret: &str, response: &str, remote_ip: Option<&str>) -> Result<(), Error> {
    let mut query = format!("secret={}&response={}", secret, response);
    if let Some(remote_ip) = remote_ip {
        query = query + &format!("&remoteip={}", remote_ip);
    }
    let uri = format!("https://www.google.com/recaptcha/api/siteverify?{}", query);
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .body(hyper::Body::empty())
        .unwrap();
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());
    let response = client.request(request).await?;
    let response_status = response.status().clone();
    let mut response_body = String::new();
    hyper::body::aggregate(response.into_body())
        .await?
        .reader()
        .read_to_string(&mut response_body)?;

    if response_status == StatusCode::OK {
        let recaptcha_response: RecaptchaResponse = serde_json::from_str(&response_body)?;
        match (recaptcha_response.success, recaptcha_response.error_codes) {
            (true, _) => Ok(()),
            (false, Some(errors)) => Err(Error::Codes(errors)),
            (false, _) => Err(Error::Codes(HashSet::new())),
        }
    } else {
        Err(Error::Gateway {
            status_code: response_status.as_u16(),
            reason: response_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_secret_missing_response() {
        use error::Code::*;
        use error::Error::*;
        let secret = "";
        let response = "";
        let response = verify(secret, response, None).await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&InvalidSecret));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }
}
