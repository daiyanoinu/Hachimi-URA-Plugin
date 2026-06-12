use std::ffi::{c_char, c_void};

use crate::il2cpp::types::*;

pub const VERSION: i32 = 2;

pub type HachimiGetApiFn = extern "C" fn(name: *const c_char) -> *mut c_void;
pub type HachimiInitFn = extern "C" fn(vtable: *const Vtable, version: i32) -> InitResult;
pub type HachimiInitV3Fn = extern "C" fn(get_api: HachimiGetApiFn, version: i32) -> InitResult;
pub type GuiMenuCallback = extern "C" fn(userdata: *mut c_void);
pub type GuiMenuSectionCallback = extern "C" fn(ui: *mut c_void, userdata: *mut c_void);
pub type GuiUiCallback = extern "C" fn(ui: *mut c_void, userdata: *mut c_void);
pub type GameInitializedCallback = unsafe extern "C" fn(userdata: *mut c_void);
pub type PresentCallback = unsafe extern "C" fn(swapchain: *mut c_void, userdata: *mut c_void);
pub type GuiWindowCallback = extern "C" fn(ui: *mut c_void, userdata: *mut c_void);


#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InitResult {
    Error,
    Ok,
}

impl InitResult {
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok => true,
            _ => false,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vtable {
    pub hachimi_instance: unsafe extern "C" fn() -> *const c_void,
    pub hachimi_get_interceptor: unsafe extern "C" fn(this: *const c_void) -> *const c_void,

    pub interceptor_hook: unsafe extern "C" fn(
        this: *const c_void, orig_addr: *mut c_void, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_hook_vtable: unsafe extern "C" fn(
        this: *const c_void, vtable: *mut *mut c_void, vtable_index: usize, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_get_trampoline_addr: unsafe extern "C" fn(
        this: *const c_void, hook_addr: *mut c_void
    ) -> *mut c_void,
    pub interceptor_unhook: unsafe extern "C" fn(this: *const c_void, hook_addr: *mut c_void) -> *mut c_void,

    pub il2cpp_resolve_symbol: unsafe extern "C" fn(name: *const c_char) -> *mut c_void,
    pub il2cpp_get_assembly_image: unsafe extern "C" fn(assembly_name: *const c_char) -> *const Il2CppImage,
    pub il2cpp_get_class: unsafe extern "C" fn(
        image: *const Il2CppImage, namespace: *const c_char, class_name: *const c_char
    ) -> *mut Il2CppClass,
    pub il2cpp_get_method: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *const MethodInfo,
    pub il2cpp_get_method_overload: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *const MethodInfo,
    pub il2cpp_get_method_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *mut c_void,
    pub il2cpp_get_method_overload_addr: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, params: *const Il2CppTypeEnum, param_count: usize
    ) -> *mut c_void,
    pub il2cpp_get_method_cached: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *const MethodInfo,
    pub il2cpp_get_method_addr_cached: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char, args_count: i32
    ) -> *mut c_void,
    pub il2cpp_find_nested_class: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char
    ) -> *mut Il2CppClass,
    pub il2cpp_resolve_icall: unsafe extern "C" fn(name: *const c_char) -> Il2CppMethodPointer,
    pub il2cpp_class_get_methods: unsafe extern "C" fn(klass: *mut Il2CppClass, iter: *mut *mut c_void) -> *const MethodInfo,
    pub il2cpp_get_field_from_name: unsafe extern "C" fn(
        class: *mut Il2CppClass, name: *const c_char
    ) -> *mut FieldInfo,
    pub il2cpp_get_field_value: unsafe extern "C" fn(
        obj: *mut Il2CppObject, field: *mut FieldInfo, out_value: *mut c_void
    ),
    pub il2cpp_set_field_value: unsafe extern "C" fn(
        obj: *mut Il2CppObject, field: *mut FieldInfo, value: *const c_void
    ),
    pub il2cpp_get_static_field_value: unsafe extern "C" fn(
        field: *mut FieldInfo, out_value: *mut c_void
    ),
    pub il2cpp_set_static_field_value: unsafe extern "C" fn(
        field: *mut FieldInfo, value: *const c_void
    ),
    pub il2cpp_object_new: unsafe extern "C" fn(klass: *const Il2CppClass) -> *mut Il2CppObject,
    pub il2cpp_unbox: unsafe extern "C" fn(obj: *mut Il2CppObject) -> *mut c_void,
    pub il2cpp_get_main_thread: unsafe extern "C" fn() -> *mut Il2CppThread,
    pub il2cpp_get_attached_threads: unsafe extern "C" fn(out_size: *mut usize) -> *mut *mut Il2CppThread,
    pub il2cpp_schedule_on_thread: unsafe extern "C" fn(thread: *mut Il2CppThread, callback: unsafe extern "C" fn()),
    pub il2cpp_create_array: unsafe extern "C" fn(
        element_type: *mut Il2CppClass, length: il2cpp_array_size_t
    ) -> *mut Il2CppArray,
    pub il2cpp_get_singleton_like_instance: unsafe extern "C" fn(class: *mut Il2CppClass) -> *mut Il2CppObject,

    pub log: unsafe extern "C" fn(level: i32, target: *const c_char, message: *const c_char),
    pub gui_register_menu_item: unsafe extern "C" fn(
        label: *const c_char,
        callback: Option<GuiMenuCallback>,
        userdata: *mut c_void
    ) -> bool,
    pub gui_register_menu_section: unsafe extern "C" fn(
        callback: Option<GuiMenuSectionCallback>,
        userdata: *mut c_void
    ) -> bool,
    pub gui_show_notification: unsafe extern "C" fn(message: *const c_char) -> bool,
    pub gui_ui_heading: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_label: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_separator: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_small_button: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char) -> bool,
    pub gui_ui_checkbox: unsafe extern "C" fn(ui: *mut c_void, text: *const c_char, value: *mut bool) -> bool,
    pub gui_ui_text_edit_singleline: unsafe extern "C" fn(
        ui: *mut c_void,
        buffer: *mut c_char,
        buffer_len: usize
    ) -> bool,
    pub gui_ui_horizontal: unsafe extern "C" fn(
        ui: *mut c_void,
        callback: Option<GuiUiCallback>,
        userdata: *mut c_void
    ) -> bool,
    pub gui_ui_grid: unsafe extern "C" fn(
        ui: *mut c_void,
        id: *const c_char,
        columns: usize,
        spacing_x: f32,
        spacing_y: f32,
        callback: Option<GuiUiCallback>,
        userdata: *mut c_void
    ) -> bool,
    pub gui_ui_end_row: unsafe extern "C" fn(ui: *mut c_void) -> bool,
    pub gui_ui_colored_label: unsafe extern "C" fn(
        ui: *mut c_void,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        text: *const c_char
    ) -> bool,
    pub gui_register_menu_item_icon: unsafe extern "C" fn(
        label: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize
    ) -> bool,
    pub gui_register_menu_section_with_icon: unsafe extern "C" fn(
        title: *const c_char,
        icon_uri: *const c_char,
        icon_ptr: *const u8,
        icon_len: usize,
        callback: Option<GuiMenuSectionCallback>,
        userdata: *mut c_void
    ) -> bool,
    // Window management (version >= 3)
    pub gui_new_window_id: unsafe extern "C" fn() -> i32,
    pub gui_show_window: unsafe extern "C" fn(
        id: i32,
        title: *const c_char,
        contents_callback: Option<GuiWindowCallback>,
        bottom_callback: Option<GuiWindowCallback>,
        userdata: *mut c_void
    ) -> bool,
    pub gui_close_window: unsafe extern "C" fn(id: i32),

    pub android_dex_load: unsafe extern "C" fn(dex_ptr: *const u8, dex_len: usize, class_name: *const c_char) -> u64,
    pub android_dex_unload: unsafe extern "C" fn(handle: u64) -> bool,
    pub android_dex_call_static_noargs: unsafe extern "C" fn(handle: u64, method: *const c_char, sig: *const c_char) -> bool,
    pub android_dex_call_static_string: unsafe extern "C" fn(handle: u64, method: *const c_char, sig: *const c_char, arg: *const c_char) -> bool,

    pub il2cpp_runtime_object_init: unsafe extern "C" fn(object: *mut Il2CppObject),
    pub il2cpp_string_new: unsafe extern "C" fn(text: *const c_char) -> *mut Il2CppString,
    pub il2cpp_string_chars: unsafe extern "C" fn(s: *mut Il2CppString) -> *mut u16,
    pub il2cpp_string_length: unsafe extern "C" fn(s: *mut Il2CppString) -> i32,
    pub gui_ui_combo_menu: unsafe extern "C" fn(
        ui: *mut c_void,
        id: *const c_char,
        selected_index: *mut i32,
        items: *const *const c_char,
        item_count: usize,
        search_term: *mut c_char,
        search_term_len: usize,
    ) -> bool,
    pub hachimi_register_on_game_initialized: unsafe extern "C" fn(
        callback: Option<GameInitializedCallback>,
        userdata: *mut c_void,
    ) -> bool,
    pub hachimi_register_present_callback: unsafe extern "C" fn(
        callback: Option<PresentCallback>,
        userdata: *mut c_void,
    ) -> bool,
    pub gui_get_menu_width: unsafe extern "C" fn() -> f32,
    pub gui_set_menu_width: unsafe extern "C" fn(width: f32),
    pub hachimi_get_base_dir: unsafe extern "C" fn() -> *const c_char,
    pub hachimi_get_data_path: unsafe extern "C" fn() -> *const c_char,
}
