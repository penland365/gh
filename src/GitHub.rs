//use std::io::{Read, stdout, Write};
use std::io::Read;
use std::str::from_utf8;
use curl::easy::{Easy, List};
use config::Config;

fn build_headers(token: &str) -> List {
    let mut xs = List::new();
    let _ = xs.append("User-Agent: gh/0.0.1-SNAPSHOT");
    let _ = xs.append("Accept: application/vnd.github.v3+json");
    let auth_header = format!("Authorization: token {}", token);
    let _ = xs.append(&auth_header);
    xs
}

fn build_headers_no_auth() -> List {
    let mut xs = List::new();
    let _ = xs.append("User-Agent: gh/0.0.1-SNAPSHOT");
    let _ = xs.append("Accept: application/vnd.github.v3+json");
    xs
}

pub fn get(token: &str, url: &str) -> Vec<u8> {
    let headers = build_headers(token);
    let mut handle = Easy::new();
    handle.http_headers(headers).unwrap();
    handle.url(url).unwrap();
    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    data
    //handle.header_function(|header| {
    //        print!("header: {}", str::from_utf8(header).unwrap());
    //            true
    //}).unwrap();
    //let y = match from_utf8(&data) {
    //    Ok(t)  => t,
    //    Err(e) => "Oh noes!"
    //};
}

pub fn post(token: &str, url: &str, mut body: &[u8]) -> Vec<u8> {
    let headers = build_headers(token);
    let mut handle = Easy::new();
    handle.http_headers(headers).unwrap();
    handle.url(url).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(body.len() as u64).unwrap();
    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        let _ = transfer.read_function(|buf| {
            Ok(body.read(buf).unwrap_or(0))
        });
        let _ = transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    data
}

fn get_no_auth(url: &str) -> Vec<u8> {
    let headers = build_headers_no_auth();
    let mut handle = Easy::new();
    handle.http_headers(headers).unwrap();
    handle.url(url).unwrap();
    let mut data = Vec::new();
    {
        let mut transfer = handle.transfer();
        let _ = transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    data
}

pub fn get_user_orgs(config: &Config) -> String {
    let url = "https://api.github.com/user/orgs";
    let response = get(&config.access_token, url);
    parse_json(&response).to_string()
}

pub fn get_user_public_orgs(user: &str) -> String {
    let url = {
        let root = "https://api.github.com/users/".to_string();
        root + user + "/orgs"
        //"https://api.github.com/users/:username/orgs";
    };
    let response = get_no_auth(&url);
    parse_json(&response).to_string()
}

pub fn parse_json(xs: &Vec<u8>) -> &str {
    match from_utf8(xs) {
        Ok(x)  => x,
        Err(_) => ""
    }
}
