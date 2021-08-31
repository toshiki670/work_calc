use log::{debug, error};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Plans {
    pub number: String,
    pub percent: f64,
    pub remark: String,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub plans: Vec<Plans>,
}

pub const DEFAULT_PATH: &str = "./setting.toml";

impl Setting {
    pub fn read(setting_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match fs::read_to_string(setting_path) {
            Ok(setting_raw) => {
                debug!("setting_raw: {:?}", &setting_raw);
                match toml::from_str(&setting_raw) {
                    Ok(setting) => Ok(setting),
                    Err(e) => {
                        error!("設定ファイルのフォーマットに問題があります。");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                error!("ファイルを読み込めませんでした。");
                Err(Box::new(e))
            }
        }
    }
}
