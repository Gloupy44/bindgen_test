
// Allow/Disable these pragmas if your lib
// follows/does not follow Rust style conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include my_c_lib Bindgen generated bindings
include!(concat!(env!("OUT_DIR"), "/my_c_lib_bindings.rs"));

// For this example, the generated binding for my_c_func
// looks like :
// extern "C" {
//    pub fn my_c_func(s: *mut ::std::os::raw::c_char) -> i32;
// }

// Wrap my_c_func() C function in a Rust safer abstraction
pub fn wrap_my_c_lib_func(s : &std::ffi::CStr) -> i32
{
    unsafe {
        crate::my_c_lib_bindings::my_c_func(s.to_bytes_with_nul().as_ptr() as *const i8)
    }
}
