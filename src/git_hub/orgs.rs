use config::Config;

use git_hub::{GitHubRequest, GitHubResponse};

use std::io::Read;

use serde_json;
use serde_json::Value as Json;
// GitHub OrgSummary. See https://developer.github.com/v3/orgs/
// A base GitHub User
#[derive(Deserialize, Serialize)]
pub struct OrgSummary {
    pub login: String,
    pub id: u32,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String
}

#[allow(dead_code)]
pub fn get_authed_user_orgs(config: &Config) -> GitHubResponse {
    let request = requests::ListOrgs{
        config: config.clone(),
    }.as_hyper_request();
    let mut response = request.start().unwrap().send().unwrap();
    let mut body = vec![];
    response.read_to_end(&mut body).unwrap();
    let s: String = String::from_utf8_lossy(&body).into_owned();
    let j: Json = match serde_json::from_str(&s) {
        Ok(j) => j,
        Err(_) => panic!("foo"),
    };
    let github_response = GitHubResponse {
        status: response.status,
        headers: response.headers.clone(),
        body: Some(j),
    };
    github_response
}

#[allow(dead_code)]
pub fn get_user_public_orgs(username: &str, config: &Config) -> GitHubResponse {
    let request = requests::ListUserOrganizations {
        username: username.to_owned(),
        config: config.clone()
    }.as_hyper_request();
    let mut response = request.start().unwrap().send().unwrap();
    let mut body = vec![];
    response.read_to_end(&mut body).unwrap();
    let s: String = String::from_utf8_lossy(&body).into_owned();
    let j: Json = match serde_json::from_str(&s) {
        Ok(j) => j,
        Err(_) => panic!("foo"),
    };
    let github_response = GitHubResponse {
        status: response.status,
        headers: response.headers.clone(),
        body: Some(j),
    };
    github_response
}

mod requests {
use config::Config;
use git_hub;
use git_hub::{add_headers, connector, GitHubRequest};
use hyper::Url;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;


    // Lists organizations for the authenticated user
    // GET /user/orgs
    // See https://developer.github.com/v3/orgs/#list-your-organizations
    // for more information
    #[allow(dead_code)]
    pub struct ListOrgs {
        pub config: Config
    }

    impl GitHubRequest for ListOrgs {
        fn as_hyper_request(&self) -> Request<Fresh> {
            let url = match Url::parse("https://api.github.com/user/orgs") {
                Ok(url) => url,
                Err(_)  => panic!("Could not parse foo bar"),
            };
            let mut req = Request::with_connector(Method::Get,
                                                  url,
                                                  &connector()).unwrap();
            add_headers(req.headers_mut(), &self.config);
            req
        }
    }

    // Lists public organization memberships for a specified user.
    // GET /users/:username/orgs
    // See https://developer.github.com/v3/orgs/#list-user-organizations
    // for more infromation
    #[allow(dead_code)]
    pub struct ListUserOrganizations {
        pub username: String,
        pub config: Config
    }

    impl GitHubRequest for ListUserOrganizations {
        fn as_hyper_request(&self) -> Request<Fresh> {
            let mut endpoint = String::with_capacity(50);
            endpoint.push_str(git_hub::URL);
            endpoint.push_str("/users/");
            endpoint.push_str(&self.username);
            endpoint.push_str("/orgs");
            let url = match Url::parse(&endpoint) {
                Ok(url) => url,
                Err(_)  => {
                    let output = format!("hyper::Url could not parse constructed URL {}", endpoint);
                    panic!(output)
                }
            };
            let mut req = Request::with_connector(Method::Get,
                                                  url,
                                                  &connector()).unwrap();
            add_headers(req.headers_mut(), &self.config);
            req
        }
    }
}
