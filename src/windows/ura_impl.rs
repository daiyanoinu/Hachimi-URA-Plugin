use alloc::string::String;
use core::option::Option;
use once_cell::sync::Lazy;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
fn get_current_dll_path() -> Option<String> {
    unsafe {
        let mut buffer: [u16; 260] = [0; 260];
        let len = GetModuleFileNameW(crate::windows::main::DLL_HMODULE, buffer.as_mut_slice());
        if len == 0 {
            return None;
        }
        let os_string = OsString::from_wide(&buffer[..len as usize]);
        Some(os_string.to_str()?.to_string())
    }
}

pub static DATA_DIR: Lazy<Option<PathBuf>> = Lazy::new(|| {
    Some(Path::new(&get_current_dll_path()?).parent()?.to_path_buf())
});
