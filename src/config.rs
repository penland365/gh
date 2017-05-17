extern crate clap;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fmt;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use std::io::{BufReader, BufWriter, Write};
use std::str::from_utf8;
use std::{option, result};
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder, as_pretty_json};

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub username: String,
    pub access_token: String
}

fn config_from_args(matches: &ArgMatches) -> Config {
    Config {
        username: matches.value_of("username").unwrap().to_string(),
        access_token: matches.value_of("access_token").unwrap().to_string()
    }
}

pub fn load_config() -> Config {
    let credentials_path = {
        let home_dir = get_home_dir();
        let mut xs = home_dir;
        xs.push(".config");
        xs.push("gh");
        xs.push("credentials");
        xs
    };
    if !credentials_path.exists() {
        panic!("no configuration found");
    }
    let file = match File::open(&credentials_path) {
        Ok(f)  => f,
        Err(e) => panic!("could not open credentials file {}", e)
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    json::decode(&contents).unwrap()
}

pub fn show_config(matches: &ArgMatches) -> () {
    let credentials_path = {
        let home_dir = get_home_dir();
        let mut xs = home_dir;
        xs.push(".config");
        xs.push("gh");
        xs.push("credentials");
        xs
    };
    if !credentials_path.exists() {
        panic!("no configuration found");
    }
    let file = match File::open(&credentials_path) {
        Ok(f)  => f,
        Err(e) => panic!("could not open credentials file {}", e)
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    let decoded: Config = json::decode(&contents).unwrap();
    match matches.value_of("format") {
        None => print_config(&decoded),
        Some(format) => if format == "json" {
            let config_json = json::as_pretty_json(&decoded);
            println!("{}", config_json);
        } else {
            panic!("unknown format request {}", format);
        }
    }
}

fn print_config(config: &Config) -> () {
    let login_length = config.username.len();
    let access_token_length = config.access_token.len();
    println!("{0: <10} {1: <40}", "login", "access token");
    println!("{0: <10} {1: <10}", config.username, config.access_token);
}

pub fn set_config(matches: &ArgMatches) -> () {
    let config = config_from_args(matches);
    let config_json = json::as_pretty_json(&config);
    let st = config_json.to_string();
    let home_dir = get_home_dir();
    let config_dir = ensure_config_dir_exists(home_dir);
    let gh_dir = ensure_gh_dir_exists(config_dir);
    let credentials = {
        let mut xs = gh_dir;
        xs.push("credentials");
        xs
    };
    let file = match OpenOptions::new().read(true)
                                       .write(true)
                                       .create(true)
                                       .open(&credentials) {
        Ok(f)  => f,
        Err(e) => panic!(e)
    };
    if credentials.exists() {
        let mut buf = BufWriter::new(&file);
        buf.write_all(st.as_bytes()).expect("Unable to write config");
    } else {
        let mut buf = BufWriter::new(&file);
        buf.write_all(st.as_bytes()).expect("Unable to write config");
    }
    let len = {
        let x: usize = st.as_bytes().len();
        x as u64
    };
    file.set_len(len);
    println!("Completed set_config!");
}

// Returns the $HOME directory or panics if it can't find it.
fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(path) => path,
        None       => panic!("$HOME directory not found")
    }
}

// Ensures the config directory exists. Creates the directory if it doesn't.
// $HOME/.config is the expected config directory. Returns the
// PathBuf after ensure it exists.
fn ensure_config_dir_exists(home_path: PathBuf) -> PathBuf {
    let config_path = {
        let mut xs = home_path;
        xs.push(".config");
        xs
    };
    if !config_path.exists() {
        fs::create_dir(&config_path);
    }
    config_path
}

// Ensures the gh directory exists. Creates the directory if it doesn't.
// $HOME/.config/gh is the expected gh directory. Returns the
// PathBuf after ensure it exists.
fn ensure_gh_dir_exists(config_path: PathBuf) -> PathBuf {
    let gh_path = {
        let mut xs = config_path;
        xs.push("gh");
        xs
    };
    if !gh_path.exists() {
        fs::create_dir(&gh_path);
    }
    gh_path
}
