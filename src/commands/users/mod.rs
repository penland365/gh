use clap::{App, Arg, ArgMatches, SubCommand};
use {GithubError, GithubResult};

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

pub fn handle(matches: &ArgMatches) -> GithubResult {
    match matches.subcommand() {
        ("get", Some(get_matches)) => Ok("asdf".to_owned()),//get::handle(get_matches),
        ("", None)                 => Err(GithubError{status_code: None,
            help_str: Some("qwer".to_owned())}),
        (_, _)                     => unreachable!()
    }
}

mod get {
use evidence::json_ops;
use super::super::NEW_LINE;
use clap::ArgMatches;
use config::load_config;
use git_hub::{GitHubResponse, users};
use hyper::status::StatusCode;
use serde_json;
use serde_json::Value as Json;

    pub fn handle(matches: &ArgMatches) -> () {
        let is_json = match matches.value_of("format") {
            None    => false,
            Some(f) => f == "json",
        };
        match matches.value_of("user") {
            None       => (), //users::get_authed_user::get(&load_config()),
            Some(user) => single_user(&user, is_json),
        }
    }

    // handle Single User
    fn single_user(user: &str, is_json: bool) -> () {
        let response = users::get_single_user::get_user(user, &load_config());
        let output   = build_single_user_output(&response, is_json);
        println!("{}", output);
    }

    fn build_output(response: &GitHubResponse, is_json: bool) -> String {
        match response.status {
            StatusCode::NotFound     => "".to_owned(),
            StatusCode::Ok           => match response.body {
                None           => build_200_ok_no_string_body_output(),
                Some(ref body) => format_output(body, is_json),
            },
            x                        => format!("Unexpected Http Response Code {}", x)
        }
    }

    fn build_single_user_output(response: &GitHubResponse, is_json: bool) -> String {
//        evidence::println_stderr("asdf");
        "".to_owned()
        //match response.status {
        //    StatusCode::NotFound => 
    
        //}
        //"".to_owned()
    }

    fn format_output(body: &Json, is_json: bool) -> String {
        if is_json {
            match serde_json::to_string_pretty(body) {
                Ok(s)  => s,
                Err(e) => "asdf".to_string(),
            }
        } else {
            let mut output = String::with_capacity(100);
            let header = format!("{0: <12} {1: <10} {2: <45}", "login", "id", "url");
            output.push_str(&header);
            output.push_str(NEW_LINE);

            let login = json_ops::get(&body, "login");
            let id    = json_ops::get(&body, "id");
            let url   = json_ops::get(&body, "url");
            let line  = format!("{0: <12} {1: <10} {2: <45}", login, id, url);
            output.push_str(&line);
            output.push_str(NEW_LINE);
            output
        }
    }

    fn build_200_ok_no_string_body_output() -> String {
        format!("An unknown error occurred. GitHub responded with {}, but no string body was found.",
                StatusCode::Ok)
    }

    const DESERIALIZE_ORG_SUMMARY: &'static str = "Error deserializing GitHub Organization Summary JSON.";
}