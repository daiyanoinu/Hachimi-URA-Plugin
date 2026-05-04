use crate::il2cpp::helper::{get_gallop_class, get_method};
use crate::il2cpp::types::Il2CppArray;
use crate::Gallop_HttpHelper::helper::{get_hachimi_and_interceptor, Array};
use crate::Gallop_HttpHelper::ura::get_config;
use crate::VTABLE;
use once_cell::sync::Lazy;
use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::thread;
use ureq;
use crate::Gallop_HttpHelper::gui;

static TIMEOUT:Lazy<Duration>=Lazy::new(|| Duration::from_millis(get_config().notifier_timeout_ms));
// https://github.com/algesten/ureq/issues/707
static AGENT: Lazy<ureq::Agent> = Lazy::new(|| {
    ureq::AgentBuilder::new()
        .timeout_connect(*TIMEOUT)
        .timeout_read(*TIMEOUT)
        .timeout_write(*TIMEOUT)
        .build()
});

static REQUEST:Lazy<String> = Lazy::new(||{
    format!("{}{}", get_config().notifier_host, "/notify/request")
});
static RESPONSE:Lazy<String> = Lazy::new(||{
    format!("{}{}", get_config().notifier_host, "/notify/response")
});

static SENDER: Lazy<Sender<(&str,Vec<u8>)>> = Lazy::new(|| {
    let (tx, rx) = channel::<(&str,Vec<u8>)>();
    thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            let _ = AGENT.post(task.0).send_bytes(task.1.as_slice());
        }
    });
    tx
});


static mut COMPRESS_REQUEST_ORIG: * mut c_void = null_mut();
static mut DECOMPRESS_RESPONSE_ORIG: * mut c_void = null_mut();

type CompressRequestFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;
type DecompressResponseFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;


#[allow(non_snake_case)]
extern "C" fn CompressRequest(data: *mut Il2CppArray) -> *mut Il2CppArray {
    if get_config().enable{
        unsafe {
            let buffer = Array::<u8>::from(data);
            _ = SENDER.send((&REQUEST, buffer.as_slice().to_vec()));
            // _ = AGENT.post(&REQUEST).send_bytes(&buffer.as_slice());
        }
    }
    let orig_fn: CompressRequestFn = unsafe { std::mem::transmute(COMPRESS_REQUEST_ORIG) };
    orig_fn(data)
}
#[allow(non_snake_case)]
extern "C" fn DecompressResponse(data: *mut Il2CppArray) -> *mut Il2CppArray {
    // log("ura".to_string(),"运行了DecompressResponse".to_string());
    let orig_fn: DecompressResponseFn = unsafe { std::mem::transmute(DECOMPRESS_RESPONSE_ORIG) };
    let decompressed=orig_fn(data);
    if get_config().enable{
        unsafe {
            let buffer = Array::<u8>::from(decompressed);
            _ = SENDER.send((&RESPONSE, buffer.as_slice().to_vec()));
            // _ = AGENT.post(&RESPONSE).send_bytes(&buffer.as_slice());
        }
    }
    decompressed
}

pub fn init() {
    unsafe {
        gui::init(VTABLE.unwrap());
        let class = get_gallop_class("HttpHelper");
        let (_, interceptor) = get_hachimi_and_interceptor();
        let vtable = VTABLE.unwrap();

        let compress_request_orig = get_method(
            class,
            "CompressRequest",
            1);
        let decompress_request_orig = get_method(
            class,
            "DecompressResponse",
            1,
        );

        COMPRESS_REQUEST_ORIG = (vtable.interceptor_hook)(
            interceptor,
            compress_request_orig,
            CompressRequest as _,
        );

        DECOMPRESS_RESPONSE_ORIG = (vtable.interceptor_hook)(
            interceptor,
            decompress_request_orig,
            DecompressResponse as _,
        );
    }

}
