extern crate libc;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::io::{self, Write};

// Define the C structures
#[repr(C)]
pub struct PrintCallback {
    user_data: *mut c_void,
    print_type: c_int,
    utf8_str: *const c_char,
}

#[repr(C)]
pub struct EndCallback {
    user_data: *mut c_void,
}

// Define the C function prototypes
extern "C" {
    fn chatllm_create() -> *mut c_void;
    fn chatllm_append_param(obj: *mut c_void, param: *const c_char);
    fn chatllm_start(
        obj: *mut c_void,
        print_callback: extern "C" fn(*mut c_void, c_int, *const c_char),
        end_callback: extern "C" fn(*mut c_void),
        user_data: *mut c_void,
    ) -> c_int;
    fn chatllm_user_input(obj: *mut c_void, input: *const c_char) -> c_int;
}

// Implement the print callback function
extern "C" fn chatllm_print(_user_data: *mut c_void, print_type: c_int, utf8_str: *const c_char) {
    unsafe {
        let cstr = CStr::from_ptr(utf8_str);
        let str_slice = cstr.to_str().unwrap_or("<invalid utf8>");
        match print_type {
            0 => print!("{}", str_slice),
            _ => println!("{}", str_slice),
        }
        // Ensure the output is flushed immediately
        io::stdout().flush().unwrap();
    }
}

// Implement the end callback function
extern "C" fn chatllm_end(_user_data: *mut c_void) {
    println!("");
}

fn main() {
    unsafe {
        let obj = chatllm_create();
        let args: Vec<String> = std::env::args().collect();
        for arg in args.iter().skip(1) {
            let c_arg = CString::new(arg.as_str()).unwrap();
            chatllm_append_param(obj, c_arg.as_ptr());
        }

        let r = chatllm_start(obj, chatllm_print, chatllm_end, ptr::null_mut());
        if r != 0 {
            println!(">>> chatllm_start error: {}", r);
            return;
        }

        loop {
            print!("You  > ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            print!("A.I. > ");
            let c_input = CString::new(input).unwrap();
            let user_input_result = chatllm_user_input(obj, c_input.as_ptr());
            if user_input_result != 0 {
                println!(">>> chatllm_user_input error: {}", user_input_result);
                break;
            }
        }
    }
}