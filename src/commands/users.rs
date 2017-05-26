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
        ("get", Some(get_matches))  => get::handle(get_matches),
        ("", None)                   => println!("No subcommand was used for users"),
        (_, _)                       => unreachable!()
    }
}

mod get {
use clap::ArgMatches;
use config::load_config;
use git_hub::{GitHubResponse, users};
//use git_hub::users::OrgSummary;

    pub fn handle(matches: &ArgMatches) -> () {
//        let response = users::
    }
}
