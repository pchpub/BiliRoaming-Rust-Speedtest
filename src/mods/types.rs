use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub user_agent: String,
    pub access_key: String,
    pub access_token: String,
    pub expire_time: u64,
    pub version_code: String,
    pub version_name: String,
    pub servers: Vec<String>,
    pub bili_app_key: String,
    pub bstar_app_key: String,
    pub ftp_url: String,
    pub ftp_user: String,
    pub ftp_password: String,
    pub ftp_open : bool,
}

#[derive(Hash, Clone)]
pub enum SpeedType {
    CnApp,
    HkApp,
    TwApp,
    ThApp,
    CnWeb,
    HkWeb,
    TwWeb,
}

impl PartialEq for SpeedType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for SpeedType {}

impl SpeedType {
    pub fn area_name(&self) -> &str {
        match self {
            SpeedType::CnApp => "cn",
            SpeedType::HkApp => "hk",
            SpeedType::TwApp => "tw",
            SpeedType::ThApp => "th", //这个写不写都一样,反正不发到服务器那里
            SpeedType::CnWeb => "cn",
            SpeedType::HkWeb => "hk",
            SpeedType::TwWeb => "tw",
        }
    }
}

pub struct SpeedTestResult {
    pub server_num: u32,
    pub results: HashMap<String, HashMap<SpeedType, Result<f64, String>>>,
}

impl SpeedTestResult {
    pub fn new(server_lists: &Vec<String>) -> SpeedTestResult {
        let mut new_data = SpeedTestResult {
            server_num: server_lists.len() as u32,
            results: HashMap::with_capacity(server_lists.len()),
        };
        for server_url in server_lists {
            new_data
                .results
                .insert(server_url.to_owned(), HashMap::new());
        }
        new_data
    }

    pub fn sort_vec(
        &self,
    ) -> Vec<(
        String,
        HashMap<SpeedType, Result<f64, String>>,
        Result<f64, String>,
    )> {
        let input_data: Vec<(String, HashMap<SpeedType, Result<f64, String>>)> =
            self.results.clone().into_iter().collect();
        let mut return_sort_vec: Vec<(
            String,
            HashMap<SpeedType, Result<f64, String>>,
            Result<f64, String>,
        )> = Vec::new();
        for item in input_data {
            //应该去除极大极小值然后取平均
            let mut time_sum = 0.0;
            let mut alive_num = 0;
            let mut max_time: f64 = 0.0;
            let mut min_time: f64 = 0.0;
            for (_key, value) in &item.1 {
                match value {
                    Ok(time_value) => {
                        if time_value < &min_time {
                            min_time = time_value.clone();
                        } else if time_value > &max_time {
                            max_time = time_value.clone();
                        }
                        time_sum += time_value;
                        alive_num += 1;
                    }
                    Err(_) => (),
                }
            }
            if alive_num == 0 {
                return_sort_vec.push((item.0.to_owned(), item.1.clone(), Err("寄".to_string())));
            } else {
                if alive_num >= 4 {
                    return_sort_vec.push((
                        item.0.to_owned(),
                        item.1.clone(),
                        Ok((time_sum - min_time - max_time) / (alive_num - 2) as f64),
                    ));
                } else {
                    return_sort_vec.push((
                        item.0.to_owned(),
                        item.1.clone(),
                        Ok(time_sum / alive_num as f64),
                    ));
                }
            }
        }
        return_sort_vec.sort_by_key(|v| match v.2 {
            Ok(value) => value as u64,
            Err(_) => 20000,
        });
        return_sort_vec
    }
}
