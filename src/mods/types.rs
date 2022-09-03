use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub user_agent: String,
    pub access_key: String,
    pub access_token: String,
    pub expire_time: String,
    pub version_code: String,
    pub version_name: String,
    pub servers: Vec<String>,
    pub bili_app_key: String,
    //pub bili_app_sec: String,
    pub bstar_app_key: String,
    //pub bstar_app_sec: String,
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

// impl Display for SpeedType { //不优雅!
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             SpeedType::CnApp => todo!(),
//             SpeedType::HkApp => todo!(),
//             SpeedType::TwApp => todo!(),
//             SpeedType::ThApp => todo!(),
//             SpeedType::CnWeb => todo!(),
//             SpeedType::HkWeb => todo!(),
//             SpeedType::TwWeb => todo!(),
//         }
//     }
// }

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
            let mut time_sum = 0.0;
            let mut alive_num = 0;
            for (_key, value) in &item.1 {
                match value {
                    Ok(time_value) => {
                        time_sum += time_value;
                        alive_num += 1;
                    }
                    Err(_) => (),
                }
            }
            if alive_num == 0 {
                return_sort_vec.push((item.0.to_owned(), item.1.clone(), Err("寄".to_string())));
            } else {
                return_sort_vec.push((
                    item.0.to_owned(),
                    item.1.clone(),
                    Ok(time_sum / alive_num as f64),
                ));
            }
        }
        return_sort_vec.sort_by_key(|v| match v.2 {
            Ok(value) => (value * 100.0) as u64,
            Err(_) => 20000,
        });
        return_sort_vec
    }
}
