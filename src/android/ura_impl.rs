use core::option::Option;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

const PACKAGE_NAME:&str="jp.co.cygames.umamusume";

pub static DATA_DIR: Lazy<Option<PathBuf>> =
    Lazy::new(|| Some(Path::new(&format!("{}{}{}","/sdcard/Android/media/",PACKAGE_NAME,"/ura")).to_path_buf()));
