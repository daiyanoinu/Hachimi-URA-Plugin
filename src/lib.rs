extern crate alloc;
extern crate core;

use crate::plugin_api::{InitResult, Vtable};

pub mod Gallop_HttpHelper;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "android")]
mod android;

mod il2cpp;
pub mod plugin_api;

pub static mut VTABLE: Option<&'static Vtable> = None;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hachimi_init(vtable: *mut Vtable, version: i32) -> InitResult {
    if vtable.is_null() {
        return InitResult::Ok
    }
    VTABLE=Some(&*vtable);
    il2cpp::helper::init(VTABLE.unwrap());
    Gallop_HttpHelper::init();
    InitResult::Ok
}