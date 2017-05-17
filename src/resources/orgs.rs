//extern crate rustc_serialize;
//extern crate serde;
//extern crate serde_json;

//use serde_json::{Value, Error};

use std::io::{stdout, Write};
use std::str::from_utf8;
use curl::easy::{Easy, List};
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

// GitHub OrgSummary. See https://developer.github.com/v3/orgs/
// A base GitHub User
#[derive(RustcDecodable, RustcEncodable, Serialize, Deserialize)]
pub struct OrgSummary {
    pub login: String,
    pub id: u32,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String
}
