
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;
use arc_swap::ArcSwap;
use core::default::Default;
use core::option::Option;
use core::prelude::v1::Ok;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use log::{error, info};
#[cfg(target_os = "android")]
use crate::android::ura_impl::DATA_DIR;

#[cfg(target_os = "windows")]
use crate::windows::ura_impl::DATA_DIR;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config{
    #[serde(default = "Config::default_notifier_host")]
    pub notifier_host: String,
    #[serde(default = "Config::default_notifier_timeout_ms")]
    pub notifier_timeout_ms: u64,
}

static CONFIG:Lazy<ArcSwap<Config>>=Lazy::new(||{
    ArcSwap::new(Arc::new(load_config()))
});

pub fn config()->Arc<Config>{
    CONFIG.load_full()
}

fn load_config()-> Config{

    if let Some(data_dir)= DATA_DIR.as_ref(){
        if !fs::exists(data_dir).unwrap() {
            match fs::create_dir_all(data_dir){
                Ok(_)=>{}
                Err(e)=>{
                    error!("创建文件夹失败: {}",e);
                    return Config::default()
                }
            }
        }

        let config_path=data_dir.as_path().join("config.json");
        if fs::metadata(&config_path).is_ok(){
            if let Ok(config_json)=fs::read_to_string(&config_path){
                if let Ok(config) =serde_json::from_str(&config_json) {
                    return config;
                }
            }
        }

        let default=Config::default();
        if let Ok(json)=serde_json::to_string_pretty(&default){
            fs::write(&config_path, json).expect("save default error");
        }
        return default
    }
    Config::default()
}
impl Config {
    fn default_notifier_host() -> String { "http://127.0.0.1:4693".to_owned() }
    fn default_notifier_timeout_ms() -> u64 { 100 }
}

impl Default for Config {
    fn default() -> Self {
        default_serde_instance().expect("default instance")
    }
}

fn default_serde_instance<'a, T: Deserialize<'a>>() -> Option<T> {
    let empty_data = std::iter::empty::<((), ())>();
    let empty_deserializer = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(empty_data);
    T::deserialize(empty_deserializer).ok()
}