use clap::{App, Arg, ArgMatches, SubCommand};
use {GithubError, GithubResult};
use hyper::status::StatusCode;
use std::str;
use util;

pub fn sub_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("users")
                .about("Get, Edit GitHub Users")
                .version(version!())
                .author("penland365 <Jeffrey.N.Davis@gmail.com>")
                .subcommand(SubCommand::with_name("get")
                                        .about("Gets the Authenticated GitHub user if no user is passed.")
                                        .arg(Arg::with_name("user")
                                             .short("u")
                                             .long("user")
                                             .help("Gets the GitHub user's public information")
                                             .value_name("octocat")
                                             .takes_value(true))
                                        .arg(Arg::with_name("format")
                                             .short("f")
                                             .long("format")
                                             .help("Sets the output format.")
                                             .value_name("json")
                                             .takes_value(true)))
}

// handle the users subcommand from main
pub fn handle(matches: &ArgMatches) -> GithubResult {
    match matches.subcommand() {
        ("get", Some(get_matches)) => get::handle(get_matches),
        ("", None)                 => {
            let help_str = util::build_app_help_string(&sub_command());
            Err(GithubError{
                status_code: None,
                help_str: Some(help_str)
            })
        },
        (_, _)                     => unreachable!()
    }
}

mod get {
use evidence::json_ops;
use super::super::NEW_LINE;
use clap::ArgMatches;
use config::load_config;
use git_hub::{GitHubResponse, users};
use {GithubError, GithubResult};
use hyper::status::StatusCode;
use serde_json;
use serde_json::Value as Json;

    pub fn handle(matches: &ArgMatches) -> GithubResult {
        let is_json = match matches.value_of("format") {
            None    => false,
            Some(f) => f == "json",
        };
        match matches.value_of("user") {
            None       => Ok("asdf".to_owned()), //users::get_authed_user::get(&load_config()),
            Some(user) => single_user(&user, is_json),
        }
    }

    // handle Single User
    fn single_user(user: &str, is_json: bool) -> GithubResult {
        let response = users::get_single_user::get_user(user, &load_config());
        return match response.status {
            StatusCode::Ok => build_single_user_output(&response, is_json),
            StatusCode::Forbidden => Err(GithubError{
                status_code: Some(StatusCode::Forbidden),
                help_str: Some("asdf".to_owned())
            }),
            x => Err(GithubError{
                status_code: Some(x),
                help_str: Some("qwer".to_owned()),
            })
        }
    }

    //#[allow(dead_code)]
    //fn build_output(response: &GitHubResponse, is_json: bool) -> String {
    //    match response.status {
    //        StatusCode::NotFound     => "".to_owned(),
    //        StatusCode::Ok           => match response.body {
    //            None           => build_200_ok_no_string_body_output(),
    //            Some(ref body) => format_output(body, is_json),
    //        },
    //        x                        => format!("Unexpected Http Response Code {}", x)
    //    }
    //}

    fn build_single_user_output(response: &GitHubResponse, is_json: bool) -> GithubResult {
        return match response.body {
            None       => Err(GithubError{
                status_code: None,
                help_str: Some(build_200_ok_no_string_body_output())
            }),
            Some(ref json) => if is_json {
                match serde_json::to_string_pretty(&json) {
                    Ok(s)  => Ok(s),
                    Err(e) => Err(GithubError{
                        status_code: None,
                        help_str: Some(format!("{}", e))
                    })
                }
            } else {
                Ok(format_output(&json))
            }
        }
    }

    fn format_output(body: &Json) -> String {
        let login = json_ops::get(&body, "login");
        let id    = json_ops::get(&body, "id");
        let url   = json_ops::get(&body, "url");

        let mut output = String::with_capacity(100);
        let login_len = login.len();
        let id_len    = id.len();
        let url_len   = url.len();
        //let login_fmt = format!("{0: <12}", login_len);
        //let header = format!("{login: <12} {1: <10} {2: <45}", "login", "id", "url");
        let header = format!("{0: <12} {1: <10} {2: <45}", "login", "id", "url");
        output.push_str(&header);
        output.push_str(NEW_LINE);

        let line  = format!("{0: <12} {1: <10} {2: <45}", login, id, url);
        output.push_str(&line);
        output.push_str(NEW_LINE);
        return output
    }

    fn build_200_ok_no_string_body_output() -> String {
        format!("An unknown error occurred. GitHub responded with {}, but no string body was found.",
                StatusCode::Ok)
    }

    //const DESERIALIZE_ORG_SUMMARY: &'static str = "Error deserializing GitHub Organization Summary JSON.";
}
