use std::ffi::{c_ulong, c_void};
use windows::Win32::Foundation::{BOOL, HMODULE, TRUE};

pub static mut DLL_HMODULE: HMODULE = HMODULE(0);
const DLL_PROCESS_ATTACH: std::os::raw::c_ulong = 1;
const DLL_PROCESS_DETACH: std::os::raw::c_ulong = 0;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn DllMain(hmodule: HMODULE, call_reason: c_ulong, _reserved: *mut c_void) -> BOOL {
    if call_reason==DLL_PROCESS_ATTACH {
        unsafe { DLL_HMODULE = hmodule; }
    }
    TRUE
}
