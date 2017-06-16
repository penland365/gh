use config::Config;

use hyper::client::{Request, Response};
use hyper::header::{Authorization, Bearer, ContentLength, Headers, Accept, qitem, UserAgent};
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::net::{Fresh, HttpsConnector};
use hyper::Url;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use serde_json;
use serde_json::Value as Json;

pub mod orgs;
pub mod users;

const URL: &'static str = "https://api.github.com";

pub trait GitHubRequest {
    fn as_hyper_request(&self) -> Request<Fresh>;
}

pub struct GitHubResponse {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Option<Json>,
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

// Takes the endpoint in string form, constructs the hyper::Url.
// If construction of the URL fails, we expects to panic! and die
pub fn build_url_or_die(endpoint: &str) -> Url {
    match Url::parse(endpoint) {
        Ok(url) => url,
        Err(x)  => {
            let panic_message = format!("{} {} {}",
                                        URL_DIE_MESSAGE,
                                        endpoint,
                                        x);
            panic!(panic_message);
        }
    }
}

const URL_DIE_MESSAGE: &'static str = "Fatal error parsing endpoint ";

// Takes a hyper::Method, hyper::Url, hyper::Connector, and Config.
// Builds a hyper::Request<Fresh> or dies if there is a failure
pub fn build_authed_request_or_die(method: &Method,
                                   url: &Url,
                                   connector: &HttpsConnector<NativeTlsClient>,
                                   config: &Config)
                                   -> Request<Fresh> {
    let mut request = match Request::with_connector(method.clone(),
                                                    url.clone(),
                                                    connector) {
        Ok(req) => req,
        Err(x)  => {
            let panic_message = format!("Fatal error building {} Request for url {}. {}",
                                        method,
                                        url,
                                        x);
            panic!(panic_message);
        },
    };
    add_headers(request.headers_mut(), config);
    request
}

// Takes a hyper::client::Request<Fresh>. Starts the request, then
// sends any associated non-header information. Panics if any error
// occurrs.
pub fn start_and_send_request(request: Request<Fresh>) -> Response {
    match request.start() {
        Ok(x)  => match x.send() {
            Ok(y)  => y,
            Err(e) => panic!("Fatal error sending HTTP Request. {}", e),
        },
        Err(e) => panic!("Fatal error starting HTTP Request. {}", e),
    }
}

// Takes a hyper::client::Headers, and returns the body length as a usize
// If no Content-Length header is found, we return 0 assuming no body
fn get_body_length(headers: &Headers) -> usize {
    match headers.get::<ContentLength>() {
        Some(l) => l.0 as usize,
        None    => 0,
    }
}

// Takes a hyper::client::Response and reads the body. Assumes a UTF-8 encoded String.
// Returns an Option<String> depending on what the ContentLength is.
pub fn read_utf8_body(response: Response) -> Option<String> {
    let length = get_body_length(&response.headers);
    if length > 0 {
        let buffer = String::with_capacity(length);
        //let num_bytes = match response.read_to_string(&mut buffer) {
        //    Ok(x)  => x,
        //    Err(e) => panic!("Fatal error reading response body {}", e),
        //};
        Some(buffer)
    } else {
        None
    }
}

pub fn read_json_body(response: Response) -> Option<Json> {
    match read_utf8_body(response).map(|s| serde_json::from_str(&s)) {
        Some(result) => result.ok(),
        None         => None,
    }
}

#[cfg(test)]
mod tests {
use config::Config;

use hyper::client::{Request, Response};
use hyper::header::{Accept, Authorization, Bearer, ContentLength, Headers, Host, qitem, UserAgent};
use hyper::method::Method;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::net::{Fresh, HttpsConnector};
use hyper::Url;
use hyper_native_tls::NativeTlsClient;
use super::{add_auth_header, add_base_headers, add_headers};

    const TEST_URL: &'static str = "https://api.github.com";
    fn build_test_url(endpoint: &str) -> Url {
        match Url::parse(endpoint) {
            Ok(url) => url,
            Err(_)  => panic!("Could not deconstruct test API URL."),
        }
    }

    fn build_test_connector() -> HttpsConnector<NativeTlsClient> {
        let ssl = match NativeTlsClient::new(){
            Ok(client) => client,
            Err(x)     => panic!("Fatal error creating test NativeTlsClient. {}", x),
        };
        HttpsConnector::new(ssl)
    }

    fn build_test_request(method: &Method,
                          url: &Url,
                          connector: &HttpsConnector<NativeTlsClient>)
                          -> Request<Fresh> {
        match Request::with_connector(method.clone(),
                                      url.clone(),
                                      connector) {
            Ok(request) => request,
            Err(x)      => panic!("Fatal error creating test Request. {}", x),
        }
    }

    fn build_test_config() -> Config {
        Config {
            username: "octocat".to_owned(),
            access_token: "abcdefg1234567".to_owned()
        }
    }

    #[test]
    fn test_add_headers() {
        let mut request = build_test_request(&Method::Get,
                                             &build_test_url(TEST_URL),
                                             &build_test_connector());
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
        let mut request = build_test_request(&Method::Get,
                                             &build_test_url(TEST_URL),
                                             &build_test_connector());
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
        let mut request = build_test_request(&Method::Get,
                                             &build_test_url(TEST_URL),
                                             &build_test_connector());
        let config = build_test_config();
        add_auth_header(request.headers_mut(), &config);
        let headers = request.headers();

        // we have 2 and only 2 headers from Auth
        assert!(headers.len() == 2);

        test_host_header(headers);
        test_authorization_header(headers, &config);
    }

    #[test]
    fn test_build_url_or_die_success() {
        let endpoint = "https://api.github.com";
        let result   = super::build_url_or_die(endpoint);
        let url      = match Url::parse(endpoint) {
            Ok(url) => url,
            Err(x)  => panic!(x),
        };
        assert_eq!(result, url);
    }

    #[test]
    #[should_panic]
    fn test_build_url_or_die_panic() {
        let endpoint = "this is not a valid url";
        let url      = match Url::parse(endpoint) {
            Ok(url) => url,
            Err(x)  => panic!(x),
        };
    }

    mod test_build_authed_request_or_die {
    use hyper::method::Method;

        const TEST_URL: &'static str = super::TEST_URL;

        #[test]
        fn test_method() {
            let req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Get,
                                                                   &super::build_test_url(TEST_URL),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());
            assert_eq!(req.method(), result.method());
        }

        #[test]
        #[should_panic]
        fn test_method_panic() {
            let req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Post,
                                                                   &super::build_test_url(TEST_URL),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());
            assert_eq!(req.method(), result.method());
        }

        #[test]
        fn test_url() {
            let req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Get,
                                                                   &super::build_test_url(TEST_URL),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());
            assert_eq!(req.url, result.url);
        }

        #[test]
        #[should_panic]
        fn test_url_panic() {
            let req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Get,
                                                                   &super::build_test_url("https://www.github.com"),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());
            assert_eq!(req.url, result.url);
        }

        #[test]
        fn test_headers() {
            let mut req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Get,
                                                                   &super::build_test_url(TEST_URL),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());

            super::super::add_headers(req.headers_mut(), &super::build_test_config());
            assert_eq!(req.headers(), result.headers());
        }

        #[test]
        #[should_panic]
        fn test_headers_panic() {
            let req = super::build_test_request(&Method::Get,
                                                &super::build_test_url(TEST_URL),
                                                &super::build_test_connector());
            let result = super::super::build_authed_request_or_die(&Method::Get,
                                                                   &super::build_test_url(TEST_URL),
                                                                   &super::build_test_connector(),
                                                                   &super::build_test_config());

            assert_eq!(req.headers(), result.headers());
        }
    }

    #[test]
    fn test_get_body_length_with_header() {
        let mut headers = Headers::new();
        let body_length = 71;
        headers.set(ContentLength(body_length));
        let result = super::get_body_length(&headers);
        assert_eq!(result, body_length as usize);
    }

    #[test]
    fn test_get_body_length_no_header() {
        let mut headers = Headers::new();
        let result = super::get_body_length(&headers);
        assert_eq!(result, 0 as usize);
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
