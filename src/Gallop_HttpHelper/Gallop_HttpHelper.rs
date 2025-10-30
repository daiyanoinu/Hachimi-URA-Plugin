use std::time::Duration;
use hachimi_plugin_sdk::api::{Hachimi, HachimiApi};
use hachimi_plugin_sdk::il2cpp::helpers::Array;
use hachimi_plugin_sdk::il2cpp::types::{Il2CppArray, Il2CppImage};
use once_cell::sync::Lazy;
use ureq;

static TIMEOUT:Duration=Duration::from_millis(500);
// https://github.com/algesten/ureq/issues/707
static AGENT: Lazy<ureq::Agent> = Lazy::new(|| {
    ureq::AgentBuilder::new()
        .timeout_connect(TIMEOUT)
        .timeout_read(TIMEOUT)
        .timeout_write(TIMEOUT)
        .build()
});
static REQUEST:&str = concat!("http://127.0.0.1:4693", "/notify/request");
static RESPONSE:&str = concat!("http://127.0.0.1:4693", "/notify/response");

static mut CompressRequest_ORIG: usize = 0;
static mut DecompressResponse_ORIG: usize = 0;

type CompressRequestFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;
type DecompressResponseFn = extern "C" fn(data: *mut Il2CppArray) -> *mut Il2CppArray;

extern "C" fn CompressRequest(data: *mut Il2CppArray) -> *mut Il2CppArray {
    unsafe {
        let buffer = Array::<u8>::from(data);
        let _ = AGENT.post(&REQUEST).send_bytes(&buffer.as_slice());
    }
    let orig_fn: CompressRequestFn = unsafe { std::mem::transmute(CompressRequest_ORIG) };
    orig_fn(data)
}
extern "C" fn DecompressResponse(data: *mut Il2CppArray) -> *mut Il2CppArray {
    let orig_fn: DecompressResponseFn = unsafe { std::mem::transmute(DecompressResponse_ORIG) };
    let decompressed=orig_fn(data);
    unsafe {
        let buffer = Array::<u8>::from(decompressed);
        let _ = AGENT.post(&RESPONSE).send_bytes(&buffer.as_slice());
    }
    decompressed
}

pub fn init(api:HachimiApi,img: *const Il2CppImage) {
    let il2cpp=api.il2cpp();
    let class=il2cpp.get_class(img, c"Gallop", c"HttpHelper");
    let hachimi=Hachimi::instance(&api);
    let interceptor = hachimi.interceptor();

    let COMPRESSREQUEST_ADDR = il2cpp.get_method_addr(class, c"CompressRequest", 1);
    let DECOMPRESSRESPONSE_ADDR = il2cpp.get_method_addr(class, c"DecompressResponse", 1);

    if let Some(trampoline) = interceptor.hook(COMPRESSREQUEST_ADDR, CompressRequest as _) {
        unsafe { CompressRequest_ORIG = trampoline; }
    }

    if let Some(trampoline) = interceptor.hook(DECOMPRESSRESPONSE_ADDR, DecompressResponse as _) {
        unsafe { DecompressResponse_ORIG = trampoline; }
    }
    // new_hook!(COMPRESSREQUEST_ADDR, CompressRequest);
    // new_hook!(DECOMPRESSRESPONSE_ADDR, DecompressResponse);
}
