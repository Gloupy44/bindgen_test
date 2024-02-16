use std::ffi::CString;

mod my_c_lib_bindings;
 
fn main() {
    println!("Hello, world!");

    let a = my_c_lib_bindings::wrap_my_c_lib_func(&CString::new("Rust").unwrap());

    println!("a = {}", a);
}