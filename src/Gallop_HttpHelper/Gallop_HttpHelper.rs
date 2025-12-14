use crate::Gallop_HttpHelper::ura::config;
use hachimi_plugin_sdk::api::{Hachimi, HachimiApi};
use hachimi_plugin_sdk::il2cpp::helpers::Array;
use hachimi_plugin_sdk::il2cpp::types::{Il2CppArray, Il2CppImage};
use once_cell::sync::Lazy;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use ureq;

static TIMEOUT:Lazy<Duration>=Lazy::new(|| Duration::from_millis(config().notifier_timeout_ms));
// https://github.com/algesten/ureq/issues/707
static AGENT: Lazy<ureq::Agent> = Lazy::new(|| {
    ureq::AgentBuilder::new()
        .timeout_connect(*TIMEOUT)
        .timeout_read(*TIMEOUT)
        .timeout_write(*TIMEOUT)
        .build()
});

static REQUEST:Lazy<String> = Lazy::new(||{
    format!("{}{}",config().notifier_host,"/notify/request")
});
static RESPONSE:Lazy<String> = Lazy::new(||{
    format!("{}{}",config().notifier_host,"/notify/response")
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


static mut COMPRESS_REQUEST_ORIG: usize = 0;
static mut DECOMPRESS_RESPONSE_ORIG: usize = 0;

type CompressRequestFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;
type DecompressResponseFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;

#[allow(non_snake_case)]
extern "C" fn CompressRequest(data: *mut Il2CppArray) -> *mut Il2CppArray {
    unsafe {
        let buffer = Array::<u8>::from(data);
        _=SENDER.send((&REQUEST,buffer.as_slice().to_vec()));
        // _ = AGENT.post(&REQUEST).send_bytes(&buffer.as_slice());
    }
    let orig_fn: CompressRequestFn = unsafe { std::mem::transmute(COMPRESS_REQUEST_ORIG) };
    orig_fn(data)
}
#[allow(non_snake_case)]
extern "C" fn DecompressResponse(data: *mut Il2CppArray) -> *mut Il2CppArray {
    let orig_fn: DecompressResponseFn = unsafe { std::mem::transmute(DECOMPRESS_RESPONSE_ORIG) };
    let decompressed=orig_fn(data);
    unsafe {
        let buffer = Array::<u8>::from(decompressed);
        _=SENDER.send((&RESPONSE,buffer.as_slice().to_vec()));
        // _ = AGENT.post(&RESPONSE).send_bytes(&buffer.as_slice());
    }
    decompressed
}

pub fn init(api:HachimiApi,img: *const Il2CppImage) {
    let il2cpp=api.il2cpp();

    let class=il2cpp.get_class(img, c"Gallop", c"HttpHelper");
    let hachimi=Hachimi::instance(&api);
    let interceptor = hachimi.interceptor();

    let compress_request_addr = il2cpp.get_method_addr(class, c"CompressRequest", 1);
    let decompress_response_addr = il2cpp.get_method_addr(class, c"DecompressResponse", 1);

    if let Some(trampoline) = interceptor.hook(compress_request_addr, CompressRequest as _) {
        unsafe { COMPRESS_REQUEST_ORIG = trampoline; }
    }

    if let Some(trampoline) = interceptor.hook(decompress_response_addr, DecompressResponse as _) {
        unsafe { DECOMPRESS_RESPONSE_ORIG = trampoline; }
    }
    // new_hook!(compressrequest_addr, CompressRequest);
    // new_hook!(decompressresponse_addr, DecompressResponse);
}
