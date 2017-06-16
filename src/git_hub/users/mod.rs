// GitHub Public User
// See https://developer.github.com/v3/users/#get-a-single-user
// for more information.
#[derive(Deserialize, Serialize)]
pub struct PublicUser {
	login: String,
	id: u32,
	avatar_url: String,
	gravatar_id: String,
	url: String,
	html_url: String,
	followers_url: String,
	following_url: String,
	gists_url: String,
	starred_url: String,
	subscriptions_url: String,
	organizations_url: String,
	repos_url: String,
	events_url: String,
	received_events_url: String,
	site_admin: bool,
	name: String,
	company: String,
	blog: String,
	location: String,
	email: Option<String>,
	hireable: bool,
	bio: String,
	public_repos: u32,
	public_gists: u32,
	following: u32,
	followers: u32,
	created_at: String,
	updated_at: String
}

pub mod get_single_user {
use config::Config;
use git_hub;
use git_hub::GitHubResponse;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;

    // GET /users/:username . GitHub public user information. See
    // https://developer.github.com/v3/users/#get-a-single-user
    // for more information.
    pub fn get_user(username: &str, config: &Config) -> GitHubResponse {
        let request      = build_get_user_request(username, config);
        let response = git_hub::start_and_send_request(request);
        GitHubResponse {
            status:  response.status.clone(),
            headers: response.headers.clone(),
            body:    git_hub::read_json_body(response),
        }
    }

    // Takes the user name, builds the valid url
    // https://api.github.com/users/:username
    fn build_get_user_endpoint(username: &str) -> String {
        let mut endpoint = String::with_capacity(50);
        endpoint.push_str(git_hub::URL);
        endpoint.push_str("/users/");
        endpoint.push_str(username);
        endpoint
    }

    // Takes the username and Config, builds a valid
    // hyper::Request<Fresh>
    fn build_get_user_request(username: &str,
                              config: &Config) -> Request<Fresh> {
        let endpoint  = build_get_user_endpoint(username);
        let url       = git_hub::build_url_or_die(&endpoint);
        let connector = git_hub::connector();
        git_hub::build_authed_request_or_die(&Method::Get,
                                             &url,
                                             &connector,
                                             config)
    }

    #[cfg(test)]
    mod tests {
    use config::Config;
    use git_hub;

        #[test]
        fn test_build_get_user_endpoint() {
            assert_eq!(super::build_get_user_endpoint("octocat"), TEST_URL);
        }

        #[test]
        fn test_build_get_user_request() {
            let request = super::build_get_user_request("octocat",
                                                        &build_test_config());
            let url = git_hub::build_url_or_die(TEST_URL);
            assert_eq!(request.url, url);
        }

        const TEST_URL: &'static str = "https://api.github.com/users/octocat";

        fn build_test_config() -> Config {
            Config {
                username: "octocat".to_owned(),
                access_token: "abcdefg1234567".to_owned()
            }
        }
    }
}

pub mod get_authed_user {
use config::Config;
use git_hub;
use git_hub::GitHubResponse;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Fresh;

    // GET /user. The authenticated GitHub user private information. See
    // https://developer.github.com/v3/users/#get-the-authenticated-user
    // for more information.
    #[allow(dead_code)]
    pub fn get(config: &Config) -> GitHubResponse {
        let request      = build_request(config);
        let response = git_hub::start_and_send_request(request);
        GitHubResponse {
            status:  response.status.clone(),
            headers: response.headers.clone(),
            body:    git_hub::read_json_body(response),
        }
    }

    const AUTH_USER_URL: &'static str = "https://api.github.com/user";

    // Takes the username and Config, builds a valid
    // hyper::Request<Fresh>
    #[allow(dead_code)]
    fn build_request(config: &Config) -> Request<Fresh> {
        let url       = git_hub::build_url_or_die(AUTH_USER_URL);
        let connector = git_hub::connector();
        git_hub::build_authed_request_or_die(&Method::Get,
                                             &url,
                                             &connector,
                                             config)
    }

    #[cfg(test)]
    mod tests {
    use config::Config;
    use git_hub;

        #[test]
        fn test_build_request() {
            let request = super::build_request(&build_test_config());
            let url = git_hub::build_url_or_die(super::AUTH_USER_URL);
            assert_eq!(request.url, url);
        }

        fn build_test_config() -> Config {
            Config {
                username: "octocat".to_owned(),
                access_token: "abcdefg1234567".to_owned()
            }
        }
    }
}
