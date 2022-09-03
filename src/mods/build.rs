use chrono::Local;

use super::types::{Config, SpeedType};

pub fn build_request(base_url: &str,speed_type: &SpeedType,config: &Config) -> Result<(String,Vec<String>),()> { //url & Headers
    match speed_type {
        SpeedType::CnApp => build_request_bili_app(base_url,speed_type,"266323",config),
        SpeedType::HkApp => build_request_bili_app(base_url,speed_type,"425578",config),
        SpeedType::TwApp => build_request_bili_app(base_url,speed_type,"285951",config),
        SpeedType::ThApp => build_request_bstar(base_url,speed_type,"377544",config),
        SpeedType::CnWeb => build_request_bili_web(base_url,speed_type,"266323",config),
        SpeedType::HkWeb => build_request_bili_web(base_url,speed_type,"425578",config),
        SpeedType::TwWeb => build_request_bili_web(base_url,speed_type,"285951",config),
    }

    //Err(())
}

fn build_request_bili_app(base_url: &str,speed_type: &SpeedType,ep_id: &str,config: &Config) -> Result<(String,Vec<String>),()> {
    let dt = Local::now();
    let ts = dt.timestamp();
    let ts_str = ts.to_string();
    let mut query_vec = vec![
        ("access_key", &config.access_key[..]),
        ("appkey", &config.bili_app_key[..]),
        ("area",speed_type.area_name()),
        ("build", "6800300"),
        ("device", "android"),
        ("ep_id",ep_id),//可以放到config.json里但没必要
        ("fnval", "464"),
        ("fnver", "0"),
        ("fourk", "1"),
        ("platform", "android"),
        ("qn", "125"),
        ("ts", &ts_str),
    ];
    query_vec.sort_by_key(|v|v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "https://{base_url}/pgc/player/api/playurl?{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}",appkey_to_sec(&config.bili_app_key).unwrap()))
    );
    Ok((signed_url,vec![format!("Build:{}",config.version_code),format!("x-from-biliroaming:{}",config.version_name)]))
}

fn build_request_bili_web(base_url: &str,speed_type: &SpeedType,ep_id: &str,config: &Config) -> Result<(String,Vec<String>),()>{
    let dt = Local::now();
    let ts = dt.timestamp();
    let ts_str = ts.to_string();
    let mut query_vec = vec![
        ("access_key", &config.access_key[..]),
        ("appkey", &config.bili_app_key[..]),
        ("area",speed_type.area_name()),
        ("build", "6800300"),
        ("device", "android"),
        ("ep_id",ep_id),//可以放到config.json里但没必要
        ("fnval", "464"),
        ("fnver", "0"),
        ("fourk", "1"),
        ("platform", "android"),
        ("qn", "125"),
        ("ts", &ts_str),
    ];
    query_vec.sort_by_key(|v|v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    //let unsigned_url = format!("{unsigned_url}");
    // let signed_url = format!( //既然油猴脚本不计算sign,那测速脚本也不应当计算(仿真
    //     "https://api.bilibili.com/pgc/player/api/playurl?{unsigned_url}&sign={:x}",
    //     md5::compute(format!("{unsigned_url}{}",appkey_to_sec(&config.bili_app_key).unwrap()))
    // );
    let unsigned_url = format!("https://{base_url}/pgc/player/web/playurl?{unsigned_url}"); //Shadow
    Ok((unsigned_url,vec![]))
}

fn build_request_bstar(base_url: &str,speed_type: &SpeedType,ep_id: &str,config: &Config) -> Result<(String,Vec<String>),()>{
    let dt = Local::now();
    let ts = dt.timestamp();
    let ts_str = ts.to_string();
    let mut query_vec = vec![
        ("access_key", &config.access_key[..]),
        ("appkey", &config.bili_app_key[..]),
        ("area",speed_type.area_name()),
        ("build", "6800300"),
        ("device", "android"),
        ("ep_id",ep_id),
        ("fnval", "464"),
        ("fnver", "0"),
        ("fourk", "1"),
        ("platform", "android"),
        ("qn", "125"),
        ("ts", &ts_str),
        ("s_locale", "zh_SG"),
    ];
    query_vec.sort_by_key(|v|v.0);
    let unsigned_url = qstring::QString::new(query_vec);
    let unsigned_url = format!("{unsigned_url}");
    let signed_url = format!(
        "https://{base_url}/intl/gateway/v2/ogv/playurl?{unsigned_url}&sign={:x}",
        md5::compute(format!("{unsigned_url}{}",appkey_to_sec(&config.bili_app_key).unwrap()))
    );
    Ok((signed_url,vec![format!("Build:{}",config.version_code),format!("x-from-biliroaming:{}",config.version_name)]))
}

fn appkey_to_sec(appkey:&str) -> Result<String, ()> {
	match appkey {
        "9d5889cf67e615cd" => Ok("8fd9bb32efea8cef801fd895bef2713d".to_string()), // Ai4cCreatorAndroid
		"1d8b6e7d45233436" => Ok("560c52ccd288fed045859ed18bffd973".to_string()), // Android 
		"07da50c9a0bf829f" => Ok("25bdede4e1581c836cab73a48790ca6e".to_string()), // AndroidB
		"8d23902c1688a798" => Ok("710f0212e62bd499b8d3ac6e1db9302a".to_string()), // AndroidBiliThings
		"dfca71928277209b" => Ok("b5475a8825547a4fc26c7d518eaaa02e".to_string()), // AndroidHD
		"bb3101000e232e27" => Ok("36efcfed79309338ced0380abd824ac1".to_string()), // AndroidI
		"4c6e1021617d40d9" => Ok("e559a59044eb2701b7a8628c86aa12ae".to_string()), // AndroidMallTicket
		"c034e8b74130a886" => Ok("e4e8966b1e71847dc4a3830f2d078523".to_string()), // AndroidOttSdk
		"4409e2ce8ffd12b8" => Ok("59b43e04ad6965f34319062b478f83dd".to_string()), // AndroidTV
		"37207f2beaebf8d7" => Ok("e988e794d4d4b6dd43bc0e89d6e90c43".to_string()), // BiliLink
		"9a75abf7de2d8947" => Ok("35ca1c82be6c2c242ecc04d88c735f31".to_string()), // BiliScan
		"7d089525d3611b1c" => Ok("acd495b248ec528c2eed1e862d393126".to_string()), // BstarA
        "178cf125136ca8ea" => Ok("34381a26236dd1171185c0beb042e1c6".to_string()), // AndroidB
        "27eb53fc9058f8c3" => Ok("c2ed53a74eeefe3cf99fbd01d8c9c375".to_string()), // ios
        "57263273bc6b67f6" => Ok("a0488e488d1567960d3a765e8d129f90".to_string()), // Android
        "7d336ec01856996b" => Ok("a1ce6983bc89e20a36c37f40c4f1a0dd".to_string()), // AndroidB
        "85eb6835b0a1034e" => Ok("2ad42749773c441109bdc0191257a664".to_string()), // unknown
        "8e16697a1b4f8121" => Ok("f5dd03b752426f2e623d7badb28d190a".to_string()), // AndroidI
        "aae92bc66f3edfab" => Ok("af125a0d5279fd576c1b4418a3e8276d".to_string()), // PC	投稿工具
        "ae57252b0c09105d" => Ok("c75875c596a69eb55bd119e74b07cfe3".to_string()), // AndroidI
        "bca7e84c2d947ac6" => Ok("60698ba2f68e01ce44738920a0ffe768".to_string()), // login
        "iVGUTjsxvpLeuDCf" => Ok("aHRmhWMLkdeMuILqORnYZocwMBpMEOdt".to_string()), //Android	取流专用
        "YvirImLGlLANCLvM" => Ok("JNlZNgfNGKZEpaDTkCdPQVXntXhuiJEM".to_string()), //ios	取流专用
        //_ => Ok("560c52ccd288fed045859ed18bffd973".to_string()),
        _ => Err(())
    }
}