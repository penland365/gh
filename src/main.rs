#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate clap;

extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate version;

use clap::{App, Arg, SubCommand};
use hyper::status::StatusCode;
use std::error::Error;
use std::{fmt, str};
use std::io::Write;

mod commands;
mod config;
mod git_hub;
mod resources;
mod evidence;

fn main() {
    let app = App::new("gh")
        //.subcommand(commands::orgs::SUBCOMMAND())
        //.subcommand(commands::pullreqs::SUBCOMMAND())
        .subcommand(commands::users::sub_command())
	    //.subcommand(SubCommand::with_name("config")
		//						.about("View and Set GitHub Configuration")
		//						.version(version!())
        //						.author("penland365 <Jeffrey.N.Davis@gmail.com>")
        //                        .subcommand(SubCommand::with_name("set")
        //                                                .about("Sets GitHub Credentials")
        //                                                .arg(Arg::with_name("username")
        //                                                          .required(true)
        //                                                          .help("GitHub user name"))
        //                                                .arg(Arg::with_name("access_token")
        //                                                          .required(true)
        //                                                          .help("GitHub access token")))
        //                        .subcommand(SubCommand::with_name("show")
        //                                                .about("Shows the current GitHub Credentials")
        //                                                .arg(Arg::with_name("format")
        //                                                     .short("f")
        //                                                     .long("format")
        //                                                     .help("Sets the output format.")
        //                                                     .value_name("json")
        //                                                     .takes_value(true))))
	    .version(version!())
        .author("penland365 <Jeffrey.N.Davis@gmail.com>");

    let mut vec: Vec<u8> = Vec::new();
    app.write_help(&mut vec).ok().expect("failed to write to Vector<u8>");
    let app_str = str::from_utf8(&vec).unwrap();

    let result = match app.get_matches().subcommand() {
        //("config", Some(config_matches)) => {
        //    match config_matches.subcommand() {
        //        ("set", Some(set_matches))   => config::set_config(set_matches),
        //        ("show", Some(show_matches)) => config::show_config(show_matches),
        //        ("", None) => println!("No subcommand was used for config"),
        //        (_, _) => unreachable!()
        //    }
        //},
        //("orgs", Some(orgs_matches))       => commands::orgs::handle(orgs_matches),
        ("users", Some(users_matches))     => commands::users::handle(users_matches),
        //("pullreq", Some(pullreq_matches)) => commands::pullreqs::handle(pullreq_matches),
        ("", None) => Err(GithubError{
            status_code: None,
            help_str: Some(app_str.to_owned()),
        }),
        (_, _)     => unreachable!()
    };

    // write out the result of the program. If no errors have occurred we write to stdout,
    // else we write to stderr. return 0 for a valid program execution and 1 for an invalid
    // execution
    std::process::exit(match result {
        Ok(s)  => {
            println!("{}", s);
            0
        },
        Err(e) => {
            match writeln!(&mut std::io::stderr(), "{}", e) {
                Ok(_)  => 1,
                Err(_) => 1
            }
        }
    });
}

type GithubResult = Result<String, GithubError>;
#[derive(Debug)]
pub struct GithubError {
    pub status_code: Option<StatusCode>,
    pub help_str: Option<String>,
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.status_code, &self.help_str) {
            (None, &None)                     => write!(f, "(None, None)"),
            (Some(_), &None)               => write!(f, "(Some(code), None)"),
            //(Some(code), &Some(ref help_str)) => write!(f, "(Some(code), Some(help_str))"),
            (Some(_), &Some(_)) => write!(f, "(Some(code), Some(help_str))"),
            (None, &Some(ref help_str))       => write!(f, "{}", &help_str),
        }
    }
}

impl Error for GithubError {
    fn description(&self) -> &str {
        "It doesn't work"
    }
    fn cause(&self) -> Option<&Error> { None }
}
