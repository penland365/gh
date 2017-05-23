use config::Config;

use hyper::client::Request;
use hyper::header::{Authorization, Bearer, Headers, Accept, qitem, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::net::{Fresh, HttpsConnector};
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;

pub mod orgs;

const URL: &'static str = "https://api.github.com";

pub trait GitHubRequest {
    fn as_hyper_request(&self) -> Request<Fresh>;
}

pub struct GitHubResponse {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Option<String>,
}

// Adds all headers to make a GitHub Request,
// 1. UserAgent
// 2. Accept
// 3. Authorization
pub fn add_headers(headers: &mut Headers, config: &Config) -> () {
    add_base_headers(headers);
    add_auth_header(headers, config);
}

// Adds the basic headers to make a GitHub Request
// 1. Accept
// 2. UserAgent
pub fn add_base_headers(headers: &mut Headers) -> () {
    headers.set(
        Accept(vec![
            qitem(Mime(TopLevel::Application,
                       SubLevel::Ext("vnd.github.v3+json".to_owned()),
                       vec![(Attr::Charset, Value::Utf8)])),
        ])
    );
    let user_agent = "gh/".to_owned() + version!();
    headers.set(UserAgent(user_agent));
}

pub fn add_auth_header(headers: &mut Headers, config: &Config) -> () {
    headers.set(
        Authorization(
            Bearer {
                token: config.access_token.to_owned()
            }
        )
    );
}

pub fn connector() -> HttpsConnector<NativeTlsClient> {
    let tls = NativeTlsClient::new().unwrap();
    HttpsConnector::new(tls)
}

#[cfg(test)]
mod tests {
use hyper::header::{Accept, Authorization, Bearer, Headers, Host, qitem, UserAgent};
use hyper::Url;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::net::Fresh;
use hyper::client::Request;
use hyper::method::Method;
use super::{add_auth_header, add_base_headers, add_headers};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use config::Config;

    fn build_test_request() -> Request<Fresh> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let url = match Url::parse("https://api.github.com") {
            Ok(url) => url,
            Err(_)  => panic!("Could not deconstruct test API URL."),
        };
        let req = Request::with_connector(Method::Get, url, &connector);
        req.unwrap()
    }

    fn build_test_config() -> Config {
        Config {
            username: "octocat".to_owned(),
            access_token: "abcdefg1234567".to_owned()
        }
    }

    #[test]
    fn test_add_headers() {
        let mut request = build_test_request();
        let config = build_test_config();
        add_headers(request.headers_mut(), &config);
        let headers = request.headers();

        // we have 4 total headers
        // Host, Accept, UserAgent, Authorization
        assert!(headers.len() == 4);
        test_host_header(headers);
        test_accept_header(headers);
        test_user_agent_header(headers);
        test_authorization_header(headers, &config);
    }

    #[test]
    fn test_add_base_headers() {
        let mut request = build_test_request();
        add_base_headers(request.headers_mut());
        let headers = request.headers();

        // we have 3 and only 3 headers in the default Request
        assert!(headers.len() == 3);

        test_host_header(headers);
        test_accept_header(headers);
        test_user_agent_header(headers);
    }

    #[test]
    fn test_add_auth_header() {
        let mut request = build_test_request();
        let config = build_test_config();
        add_auth_header(request.headers_mut(), &config);
        let headers = request.headers();

        // we have 2 and only 2 headers from Auth
        assert!(headers.len() == 2);

        test_host_header(headers);
        test_authorization_header(headers, &config);
    }

    fn test_host_header(headers: &Headers) -> () {
        assert!(headers.has::<Host>() == true);
        assert!(headers.get::<Host>() == Some(&Host{
            hostname: "api.github.com".to_owned(),
            port:     Some(443)
        }));
    }

    fn test_accept_header(headers: &Headers) -> () {
        assert!(headers.has::<Accept>() == true);
        assert!(headers.get::<Accept>() == Some(
            &Accept(vec![
                qitem(Mime(TopLevel::Application,
                           SubLevel::Ext("vnd.github.v3+json".to_string()),
                           vec![(Attr::Charset, Value::Utf8)])),
            ])
        ));
    }

    fn test_user_agent_header(headers: &Headers) -> () {
        let user_agent = "gh/".to_owned() + version!();
        assert!(headers.has::<UserAgent>() == true);
        assert!(headers.get::<UserAgent>() ==
            Some(&UserAgent(user_agent)));
    }

    fn test_authorization_header(headers: &Headers, config: &Config) -> () {
        assert!(headers.has::<Authorization<Bearer>>() == true);
        assert!(headers.get::<Authorization<Bearer>>() == Some(
            &Authorization(
                Bearer {
                    token: config.access_token.to_owned()
                }
            )
        ));
    }
}
