use crate::plugin_api::Vtable;

#[allow(non_snake_case)]
pub mod Gallop_HttpHelper;
pub mod ura;
mod helper;
mod gui;

pub unsafe fn init() {
    Gallop_HttpHelper::init();
}
