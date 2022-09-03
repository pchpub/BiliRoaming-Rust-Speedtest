use chrono::prelude::*;
use curl::Error;
use curl::easy::{Easy, List};
use std::string::String;
use std::time::Duration;

pub fn getwebpage(
    url: &str,
    user_agent: &str,
    headers: &Vec<String>,
    need_body: bool,
) -> Result<(String, String, f64), String> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    println!("{:?}",curl::Version::get());
    let mut handle_headers = List::new();
    for item in headers.iter() {
        handle_headers.append(item).unwrap();
    }
    handle.http_headers(handle_headers).unwrap();
    handle.url(&url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(true).unwrap();
    handle.post(false).unwrap();
    handle.useragent(&user_agent).unwrap();
    handle.connect_timeout(Duration::new(20, 0)).unwrap();
    handle.accept_encoding("gzip,deflate,br").unwrap();
    if !need_body {
        handle.nobody(true).unwrap();
    }
    let time: f64;
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        let dt = Local::now();
        let ts1 = dt.timestamp_millis();
        match transfer.perform() {
            Ok(()) => (()),
            Err(value) => {
                println!("{:?}",value);
                return Err("-404".to_owned());
            }
        }
        let dt = Local::now();
        let ts2 = dt.timestamp_millis();
        time = (ts2 - ts1) as f64 / 1000.0;
    }
    //println!("{}",handle.response_code());
    let getwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return Err("panic".to_owned());
        }
    };
    let response_code = match handle.response_code() {
        Ok(value) => format!("{}", value),
        Err(_) => "-404".to_owned(),
    };
    println!("{}",getwebpage_string);
    Ok((response_code,getwebpage_string, time))
}
