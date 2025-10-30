pub mod Gallop_HttpHelper;

use hachimi_plugin_sdk::{api::{Hachimi,HachimiApi}, sys::InitResult};
use hachimi_plugin_sdk::{hachimi_plugin};
use log::info;

static mut API: Option<HachimiApi> = None;
static mut HACHIMI: Option<Hachimi> = None;

#[hachimi_plugin]
pub fn main(api: HachimiApi) -> InitResult {
    _ = hachimi_plugin_sdk::log::init(api, log::Level::Info);
    Gallop_HttpHelper::init(api);
    info!("插件加载完成");
    InitResult::Ok
}