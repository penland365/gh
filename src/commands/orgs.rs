use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use config;
use GitHub;

use serde_json;

use rustc_serialize::Encodable;
use rustc_serialize::json::{as_pretty_json, self, Encoder};
use resources::orgs::OrgSummary;
use commands::print_pretty_json;

pub fn SUBCOMMAND<'a, 'b>() -> App<'a, 'b> {
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

pub fn handle(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        ("list", Some(list_matches)) => list_user_orgs(list_matches) ,
        ("", None)                   => println!("No subcommand was used for orgs"),
        (_, _)                       => unreachable!()
    }
}

// get finagle
// patch finagle
// list

fn list_user_orgs(matches: &ArgMatches) -> () {
    let json_response = match matches.value_of("user") {
        None       => GitHub::get_user_orgs(&config::load_config()),
        Some(user) => GitHub::get_user_public_orgs(user)
    };
    let decoded: Vec<OrgSummary> = json::decode(&json_response).unwrap();
    match matches.value_of("format") {
        None => print_user_orgs(&decoded),
        Some(format) => print_pretty_json()
        //Some(format) => if format == "json" {
        //    let json = json::as_pretty_json(&decoded);
        //    println!("{}", json);
        //} else {
        //    panic!("unknown format request {}", format);
        //}
    }
}

fn print_user_orgs(orgs: &Vec<OrgSummary>) -> () {
    println!("{0: <10} {1: <10} {2: <45} {3: <30}", "login", "id", "url", "description");
    for x in orgs {
        println!("{0: <10} {1: <10} {2: <45} {3: <30}",
                 x.login, x.id, x.url, x.description);
    }
}
