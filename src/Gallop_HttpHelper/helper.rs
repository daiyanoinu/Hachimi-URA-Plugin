use std::ffi::{c_void, CString};
use std::marker::PhantomData;
use std::os::raw::c_char;
use crate::il2cpp::types::{il2cpp_array_size_t, Il2CppArray, Il2CppClass, Il2CppObject};
use crate::VTABLE;

pub fn str_to_c_char(str:&str) -> *const c_char{
    CString::new(str).unwrap().into_raw()
}

pub unsafe fn get_hachimi_and_interceptor() -> (*const c_void, *const c_void) {
    unsafe {
        let vtable = VTABLE.unwrap();
        let hachimi = (vtable.hachimi_instance)();
        let interceptor = (vtable.hachimi_get_interceptor)(hachimi);
        (hachimi, interceptor)
    }
}

// Il2CppArray wrapper
#[repr(transparent)]
pub struct Array<T = *mut Il2CppObject> {
    pub this: *mut Il2CppArray,
    pub(crate) _phantom: PhantomData<T>
}


impl<T> Array<T> {
    pub fn new(element_type: *mut Il2CppClass, length: il2cpp_array_size_t) -> Array<T> {
        unsafe {
            Array {
                this: (VTABLE.unwrap().il2cpp_create_array)(element_type, length),
                _phantom: PhantomData,
            }
        }
    }

    pub unsafe fn data_ptr(&self) -> *mut T {
        self.this.add(1) as _
    }

    pub unsafe fn as_slice(&self) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.data_ptr(), (*self.this).max_length)
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.this).max_length }
    }
}

impl<T> Into<*mut Il2CppArray> for Array<T> {
    fn into(self) -> *mut Il2CppArray {
        self.this
    }
}

impl<T> From<*mut Il2CppArray> for Array<T> {
    fn from(value: *mut Il2CppArray) -> Self {
        Self {
            this: value,
            _phantom: PhantomData
        }
    }
}

fn log(target:String,message:String) {
    unsafe { (VTABLE.unwrap().log)(log::Level::Info as _, CString::new(target).unwrap().as_ptr(), CString::new(message).unwrap().as_ptr()); }
}