extern crate ffi_convert;
extern crate lean_sys;

use ffi_convert::{CReprOf, CStringArray};
use std::ffi::CStr;
use std::os::raw::c_char;
use lean_sys::{lean_array_push, lean_mk_empty_array, lean_mk_string, lean_obj_res};

// #[no_mangle]
// pub extern "C" fn list_from_rust(_a : i32, _b : i32) -> CStringArray {
//   let pizza_names = vec!["Diavola".to_string(), "Margarita".to_string(), "Regina".to_string()];
//   let c_pizza_names = CStringArray::c_repr_of(pizza_names).expect("could not convert !");
//   return c_pizza_names;
// }

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {
  return a + b
}

#[no_mangle]
pub extern "C" fn arr() -> lean_obj_res {
    let x = unsafe { lean_mk_empty_array() };
    unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    unsafe { lean_array_push(x, lean_mk_string("http://example.org/test".as_ptr())) };
    unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    unsafe { lean_array_push(x, lean_mk_string("http://example.org/predicate".as_ptr())) };
    unsafe { lean_array_push(x, lean_mk_string("Literal".as_ptr())) };
    unsafe { lean_array_push(x, lean_mk_string("belting".as_ptr())) };
    return unsafe { lean_mk_string("NamedNode".as_ptr()) };
}


// Function to be called from foreign code
// #[no_mangle]
// pub extern "C" fn iterate_strings(strings: *const *const c_char) {
//     if strings.is_null() {
//         return;
//     }

//     let mut current = strings;
//     while unsafe { !(*current).is_null() } {
//         let c_str = unsafe { CStr::from_ptr(*current) };
//         match c_str.to_str() {
//             Ok(str) => {
//                 println!("Received string: {}", str);
//                 // Process the string here
//             }
//             Err(e) => {
//                 eprintln!("Invalid UTF-8 sequence: {}", e);
//                 // Handle the error
//             }
//         }
//         unsafe { current = current.add(1) };
//     }
// }
