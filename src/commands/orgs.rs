use clap::{App, Arg, ArgMatches, SubCommand};

#[allow(dead_code)]
pub fn sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("orgs")
                .about("List, Get, Edit your GitHub Organizations")
                .version(version!())
                .author("penland365 <Jeffrey.N.Davis@gmail.com>")
                .subcommand(SubCommand::with_name("list")
                                        .about("Lists GitHub Organizations for the credentialed user.")
                                        .arg(Arg::with_name("user")
                                             .short("u")
                                             .long("user")
                                             .help("Searches for public organizations for this user")
                                             .value_name("octocat")
                                             .takes_value(true))
                                        .arg(Arg::with_name("format")
                                             .short("f")
                                             .long("format")
                                             .help("Sets the output format.")
                                             .value_name("json")
                                             .takes_value(true)))
}

#[allow(dead_code)]
pub fn handle(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        ("list", Some(list_matches)) => list::handle(list_matches),
        ("", None)                   => println!("No subcommand was used for orgs"),
        (_, _)                       => unreachable!()
    }
}

mod list {
use super::super::NEW_LINE;
use clap::ArgMatches;
use config::load_config;
use evidence::json_ops;
use git_hub::{GitHubResponse, orgs};
use hyper::status::StatusCode;
use git_hub::orgs::OrgSummary;
use serde_json::Value as Json;

    #[allow(dead_code)]
    pub fn handle(matches: &ArgMatches) -> () {
        let response = match matches.value_of("user") {
            None       => orgs::get_authed_user_orgs(&load_config()),
            Some(user) => orgs::get_user_public_orgs(user, &load_config()),
        };
        let is_json  = match matches.value_of("format") {
            None         => false,
            Some(format) => format == "json"
        };
        let output = &build_output(&response, is_json);
        println!("{}", output.trim());
    }

    fn build_output(response: &GitHubResponse, is_json: bool) -> String {
        match response.status {
            StatusCode::Forbidden    => FORBIDDEN.to_owned(),
            StatusCode::Unauthorized => UNAUTHORIZED.to_owned(),
            StatusCode::Ok           => match response.body {
                None           => build_200_ok_no_string_body_output(),
                Some(ref body) => format_output(body, is_json),
            },
            x                        => format!("Unexpected Http Response Code {}", x)
        }
    }

    fn build_200_ok_no_string_body_output() -> String {
        format!("An unknown error occurred. GitHub responded with {}, but no string body was found.",
                StatusCode::Ok)
    }

    fn format_output(body: &Json, is_json: bool) -> String {
        let orgs: Vec<OrgSummary> = json_ops::from_json_or_die(body, DESERIALIZE_ORG_SUMMARY);
        if is_json {
            json_ops::to_pretty_json_or_die(&orgs, SERIALIZE_ORG_SUMMARY)
        } else {
            let mut output = String::with_capacity(100);
            let header = format!("{0: <10} {1: <10} {2: <45} {3: <30}", "login", "id", "url", "description");
            output.push_str(&header);
            output.push_str(NEW_LINE);
            for org in orgs {
                let line = format!("{0: <10} {1: <10} {2: <45} {3: <30}",
                                   org.login, org.id, org.url, org.description);
                output.push_str(&line);
                output.push_str(NEW_LINE);
            }
            output
        }
    }

    const DESERIALIZE_ORG_SUMMARY: &'static str = "Error deserializing GitHub Organization Summary JSON.";
    const SERIALIZE_ORG_SUMMARY: &'static str = "Error serializing GitHub Organization Summary JSON.";

    const UNAUTHORIZED: &'static str = "401 Unauthorized. Bad Credentials. See https://developer.github.com/v3";
    const FORBIDDEN: &'static str = "403 Forbidden. Does your OAuth token have suffecient scope? A minimum of `user` or `read:org` is required. See https://developer.github.com/v3/orgs/";

    #[cfg(test)]
    mod tests {
    use git_hub::GitHubResponse;
    use hyper::header::Headers;
    use hyper::status::StatusCode;
    use super::{build_output, FORBIDDEN, UNAUTHORIZED};

        #[test]
        fn test_build_output_forbidden() -> () {
            let response = GitHubResponse {
                status: StatusCode::Forbidden,
                headers: Headers::new(),
                body: None
            };
            assert_eq!(build_output(&response, false), FORBIDDEN);
        }

        #[test]
        fn test_build_output_unauthorized() -> () {
            let response = GitHubResponse {
                status: StatusCode::Unauthorized,
                headers: Headers::new(),
                body: None
            };
            assert_eq!(build_output(&response, false), UNAUTHORIZED);
        }

        #[test]
        fn test_build_output_unknown() -> () {
            let response = GitHubResponse {
                status: StatusCode::ImATeapot,
                headers: Headers::new(),
                body: None
            };
            assert_eq!(build_output(&response, false),
                       "Unexpected Http Response Code 418 I'm a teapot");
        }

        #[test]
        fn test_build_200_ok_no_string_body_output() -> () {
            assert_eq!(super::build_200_ok_no_string_body_output(),
                      "An unknown error occurred. GitHub responded with 200 OK, but no string body was found.");
        }

        #[test]
        fn test_build_output_no_string_body() -> () {
            let response = GitHubResponse {
                status: StatusCode::Ok,
                headers: Headers::new(),
                body: None
            };
            assert_eq!(build_output(&response, false),
                      "An unknown error occurred. GitHub responded with 200 OK, but no string body was found.");

        }
    }
}

// get finagle
// patch finagle
// list
