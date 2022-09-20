use biliroaming_rust_speedtest::mods::{
    build::build_request,
    draw::draw,
    request::{get_accesskey_from_token_cn, getwebpage},
    types::{Config, SpeedTestResult, SpeedType}, upload::upload_file_ftp,
};
use chrono::Local;
use core::time;
//use core::time;
//use std::{fs::File, thread};
use std::{
    fs::{self, File},
    thread,
};

fn main() {
    let config_file: File;
    match File::open("config.json") {
        Ok(value) => {
            config_file = value;
        }
        Err(_) => {
            println!("[error] 配置文件打开失败");
            std::process::exit(78);
        }
    }
    let mut config: Config = serde_json::from_reader(config_file).unwrap();
    let dt = Local::now();
    let ts = dt.timestamp() as u64;
    if config.expire_time - 600 < ts {
        let (new_accesskey, new_token, new_expire_time) = get_accesskey_from_token_cn(
            &config.access_key,
            &config.access_token,
            &config.user_agent,
        )
        .unwrap();
        config = Config {
            access_key: new_accesskey,
            access_token: new_token,
            expire_time: new_expire_time,
            ..config
        };
        fs::write(
            "config.json",
            serde_json::to_string_pretty(&config).unwrap(),
        )
        .unwrap();
    }
    let areas = [
        SpeedType::CnApp,
        SpeedType::HkApp,
        SpeedType::TwApp,
        SpeedType::CnWeb,
        SpeedType::HkWeb,
        SpeedType::TwWeb,
        SpeedType::ThApp,
    ];

    let user_agent = &config.user_agent;
    let mut last_spend_time = 15 * 60;
    loop {
        let ten_millis = time::Duration::from_secs(15 * 60 - last_spend_time);
        thread::sleep(ten_millis);

        let dt = Local::now();
        let ts1 = dt.timestamp();
        let mut server_urls: Vec<String> = Vec::new();
        for server_url in &config.servers {
            let url = format!("https://{}", idna::domain_to_ascii(server_url).unwrap());
            //println!("{}",url);
            match getwebpage(
                &url,
                user_agent,
                &vec!["From: biliroaming_rust_speedtest".to_string()],
                false,
            ) {
                Ok(value) => {
                    if value.0.as_str() == "200" {
                        server_urls.push(server_url.to_string());
                    }
                }
                Err(_) => (), //意味着这个服务器寄的很彻底
            }
        }

        let mut speed_test_result = SpeedTestResult::new(&server_urls);

        for area in &areas {
            for server_url in &server_urls {
                let (url, headers) = build_request(server_url, &area, &config).unwrap();
                match getwebpage(&url, &config.user_agent, &headers, true) {
                    Ok(value) => {
                        if value.0.as_str() == "200" {
                            match serde_json::from_str::<serde_json::Value>(&value.1) {
                                Ok(serde_value) => {
                                    if serde_value["code"].as_i64().unwrap_or(404) == 0 {
                                        (*(speed_test_result.results.get_mut(server_url).unwrap()))
                                            .insert(area.clone(), Ok(value.2));
                                    } else {
                                        (*(speed_test_result.results.get_mut(server_url).unwrap()))
                                            .insert(
                                                area.clone(),
                                                Err(serde_value["code"]
                                                    .as_i64()
                                                    .unwrap_or(404)
                                                    .to_string()),
                                            );
                                    }
                                }
                                Err(_) => {
                                    (*(speed_test_result.results.get_mut(server_url).unwrap()))
                                        .insert(area.clone(), Err("ERR".to_owned()));
                                }
                            }
                        } else {
                            //println!("{}", value.1.as_str());
                            (*(speed_test_result.results.get_mut(server_url).unwrap()))
                                .insert(area.clone(), Err(value.0.to_owned()));
                        }
                    }
                    Err(value) => {
                        (*(speed_test_result.results.get_mut(server_url).unwrap()))
                            .insert(area.clone(), Err(value));
                    }
                }
            }
        }

        let dt = Local::now();
        let ts2 = dt.timestamp();
        last_spend_time = (ts2 - ts1) as u64;

        if let Err(_) = draw("result.png", &config, &speed_test_result) {
            println!("[Error] write result.png failed");
        } else {
            if config.ftp_open {
                upload_file_ftp(&config.ftp_url, &config.ftp_user, &config.ftp_password, "result.png")
            }
        }
    }
}
