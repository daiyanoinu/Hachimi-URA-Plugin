extern crate alloc;
extern crate core;

pub mod Gallop_HttpHelper;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "android")]
mod android;

use core::option::Option;
use core::option::Option::None;
use hachimi_plugin_sdk::{api::{Hachimi, HachimiApi}, sys::InitResult};
use hachimi_plugin_sdk::hachimi_plugin;
use log::info;

#[hachimi_plugin]
pub fn main(api: HachimiApi) -> InitResult {
    _ = hachimi_plugin_sdk::log::init(api, log::Level::Info);
    Gallop_HttpHelper::init(api);
    info!("插件加载完成");
    InitResult::Ok
}