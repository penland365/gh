use clap::{App, Arg, ArgMatches, SubCommand};

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

pub fn handle(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        ("get", Some(get_matches)) => get::handle(get_matches),
        ("", None)                 => println!("No subcommand was used for users"),
        (_, _)                     => unreachable!()
    }
}

mod get {
use clap::ArgMatches;
use config::load_config;
use git_hub::{GitHubResponse, users};
use hyper::status::StatusCode;
use serde_json::Value as Json;

    pub fn handle(matches: &ArgMatches) -> () {
        let response = match matches.value_of("user") {
            None       => users::get_authed_user::get(&load_config()),
            Some(user) => users::get_single_user::get_user(user, &load_config()),
        };
        let is_json  = match matches.value_of("format") {
            None         => false,
            Some(format) => format == "json"
        };
        let output = &build_output(&response, is_json);
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

    fn format_output(body: &Json, is_json: bool) -> String {
        "".to_string()
        //let orgs: Vec<OrgSummary> = json_ops::from_json_or_die(body, DESERIALIZE_ORG_SUMMARY);
        //if is_json {
        //    json_ops::to_pretty_json_or_die(&orgs, SERIALIZE_ORG_SUMMARY)
        //} else {
        //    let mut output = String::with_capacity(100);
        //    let header = format!("{0: <10} {1: <10} {2: <45} {3: <30}", "login", "id", "url", "description");
        //    output.push_str(&header);
        //    output.push_str(NL);
        //    for org in orgs {
        //        let line = format!("{0: <10} {1: <10} {2: <45} {3: <30}",
        //                           org.login, org.id, org.url, org.description);
        //        output.push_str(&line);
        //        output.push_str(NL);
        //    }
        //    output
        //}
    }

    fn build_200_ok_no_string_body_output() -> String {
        format!("An unknown error occurred. GitHub responded with {}, but no string body was found.",
                StatusCode::Ok)
    }

    const DESERIALIZE_ORG_SUMMARY: &'static str = "Error deserializing GitHub Organization Summary JSON.";
}
