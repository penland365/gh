#[macro_use]
extern crate version;
extern crate curl;
extern crate rustc_serialize;
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;

mod config;
mod resources;

fn main() {
    let matches = App::new("gh")
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
        ("", None) => println!("NO SUBCOMMAND USED"),
        (_, _)     => unreachable!()
    }

    //resources::function();

    //let user = resources::users::get_user("pengwynn".to_string());
    //println!("{}", user.login);
    //


}
