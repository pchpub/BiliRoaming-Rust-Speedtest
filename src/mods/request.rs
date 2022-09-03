use chrono::Local;
use curl::easy::{Easy, List};
use std::io::Read;
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
    //println!("{:?}",curl::Version::get());
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
    handle.connect_timeout(Duration::new(10, 0)).unwrap();
    //handle.accept_encoding("gzip,deflate,br").unwrap();
    if !need_body {
        handle.nobody(true).unwrap();
    }
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (()),
            Err(_value) => {
                return Err("-404".to_owned());
            }
        }
    }
    //println!("{:?}",handle.connect_time());
    let time = handle.connect_time().unwrap().as_secs_f64() * 1000.0;
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
    //println!("{}",getwebpage_string);
    Ok((response_code, getwebpage_string, time))
}

pub fn postwebpage(
    url: &str,
    content: &str,
    user_agent: &str,
) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    let mut request_data = content.as_bytes();
    let mut headers = List::new();
    headers
        .append("Content-Type: application/x-www-form-urlencoded")
        .unwrap();
    headers.append("charset=utf-8").unwrap();
    handle.http_headers(headers).unwrap();
    handle.url(&url).unwrap();
    handle.follow_location(true).unwrap();
    handle.ssl_verify_peer(false).unwrap();
    handle.post(true).unwrap();
    handle.post_field_size(request_data.len() as u64).unwrap();
    handle.useragent(&user_agent).unwrap();
    handle.connect_timeout(Duration::new(20, 0)).unwrap();

    {
        let mut transfer = handle.transfer();
        transfer
            .read_function(|into| Ok(request_data.read(into).unwrap()))
            .unwrap();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        match transfer.perform() {
            Ok(()) => (()),
            _error => {
                return Err(());
            }
        }
    }

    let getwebpage_string: String = match String::from_utf8(data) {
        Ok(value) => value,
        Err(_) => {
            return Err(());
        }
    };
    Ok(getwebpage_string)
}

pub fn get_accesskey_from_token_cn(
    access_key: &str,
    refresh_token: &str,
    user_agent: &str,
) -> Option<(String, String, u64)> {
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    let unsign_request_body = format!(
        "access_token={access_key}&appkey=1d8b6e7d45233436&refresh_token={refresh_token}&ts={ts}"
    );
    let url = "https://passport.bilibili.com/x/passport-login/oauth2/refresh_token";
    let content = format!(
        "{unsign_request_body}&sign={:x}",
        md5::compute(format!(
            "{unsign_request_body}560c52ccd288fed045859ed18bffd973"
        ))
    );
    let getpost_string = match postwebpage(&url, &content, &user_agent) {
        Ok(value) => value,
        Err(_) => return None,
    };
    let getpost_json: serde_json::Value = serde_json::from_str(&getpost_string).unwrap();
    Some((
        getpost_json["data"]["token_info"]["access_token"]
            .as_str()
            .unwrap()
            .to_string(),
        getpost_json["data"]["token_info"]["refresh_token"]
            .as_str()
            .unwrap()
            .to_string(),
        getpost_json["data"]["token_info"]["expires_in"]
            .as_u64()
            .unwrap() + ts,
    ))
}
