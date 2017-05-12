extern crate curl;
extern crate rustc_serialize;

use std::io::{stdout, Write};
use std::str::from_utf8;
use std::{option, result};
use curl::easy::{Easy, List};
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

// GitHub users. See https://developer.github.com/v3/users/
// A base GitHub User
#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    pub login: String,
    pub id: u32,
    pub location: String,
    pub bio: Option<String>
}

impl User {
    //pub fn display_bio(&self) -> &str {"foo bar"}
    pub fn display_bio(&self) -> &str {
        match self.bio {
            Some(ref b) => b,
            None    => ""
        }
    }
}

pub fn get_user(login: String) -> User {
    let mut handle = Easy::new();
    let mut xs = List::new();
    xs.append("User-Agent: gh/0.0.1-SNAPSHOT");
    xs.append("Accept: application/vnd.github.v3+json");
    handle.http_headers(xs).unwrap();
    //handle.url("https://api.github.com/users/pengwynn").unwrap();
    handle.url("https://api.github.com/users/penland365").unwrap();
    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let y = match from_utf8(&data) {
        Ok(t)  => t,
        Err(e) => "Oh noes!"
    };
    //println!("{:?}", y);
    let decoded: User = json::decode(y).unwrap();
    //let display_bio = match decoded.bio {
    //    Some(ref b) => b,
    //    None        => "No bio found"
    //};
    println!("Wow! the login is {}, the id is {}, and the bio is {}",
            decoded.login,
            decoded.id,
            decoded.display_bio());

    decoded
}
