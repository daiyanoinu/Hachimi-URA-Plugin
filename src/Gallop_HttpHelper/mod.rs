use hachimi_plugin_sdk::api::{Hachimi, HachimiApi};
use crate::API;

pub mod Gallop_HttpHelper;

pub fn init(api: HachimiApi) {
    let il2cpp=api.il2cpp();
    let image=il2cpp.get_assembly_image(c"umamusume.dll");
    Gallop_HttpHelper::init(api,image);
}
