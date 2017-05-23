#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;
extern crate clap;

extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate version;

use clap::{App, Arg, SubCommand};

mod commands;
mod config;
mod git_hub;
mod resources;

fn main() {
    let matches = App::new("gh")
        .subcommand(commands::orgs::SUBCOMMAND())
        .subcommand(commands::pullreqs::SUBCOMMAND())
	    .subcommand(SubCommand::with_name("config")
								.about("View and Set GitHub Configuration")
								.version(version!())
        						.author("penland365 <Jeffrey.N.Davis@gmail.com>")
                                .subcommand(SubCommand::with_name("set")
                                                        .about("Sets GitHub Credentials")
                                                        .arg(Arg::with_name("username")
                                                                  .required(true)
                                                                  .help("GitHub user name"))
                                                        .arg(Arg::with_name("access_token")
                                                                  .required(true)
                                                                  .help("GitHub access token")))
                                .subcommand(SubCommand::with_name("show")
                                                        .about("Shows the current GitHub Credentials")
                                                        .arg(Arg::with_name("format")
                                                             .short("f")
                                                             .long("format")
                                                             .help("Sets the output format.")
                                                             .value_name("json")
                                                             .takes_value(true))))
	    .version(version!())
        .author("penland365 <Jeffrey.N.Davis@gmail.com>")
        .get_matches();

    match matches.subcommand() {
        ("config", Some(config_matches)) => {
            match config_matches.subcommand() {
                ("set", Some(set_matches))   => config::set_config(set_matches),
                ("show", Some(show_matches)) => config::show_config(show_matches),
                ("", None) => println!("No subcommand was used for config"),
                (_, _) => unreachable!()
            }
        },
        ("orgs", Some(orgs_matches)) => commands::orgs::handle(orgs_matches),
        ("pullreq", Some(pullreq_matches)) => commands::pullreqs::handle(pullreq_matches),
        ("", None) => println!("NO SUBCOMMAND USED"),
        (_, _)     => unreachable!()
    }
}
