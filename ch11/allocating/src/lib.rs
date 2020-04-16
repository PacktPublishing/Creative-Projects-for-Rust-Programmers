#![no_std]

extern crate alloc;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;

use linux_kernel_module::c_types;
use linux_kernel_module::println;

struct GlobalData {
    n: u16,
    msg: String,
    values: Vec<i32>,
}

static mut GLOBAL: GlobalData = GlobalData {
    n: 1000,
    msg: String::new(),
    values: Vec::new(),
};

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    println!("allocating: Loaded");
    unsafe {
        GLOBAL.n += 1;
        GLOBAL.msg += "abcd";
        GLOBAL.values.push(500_000);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        println!("allocating: Unloaded {} {} {}",
            GLOBAL.n,
            GLOBAL.msg,
            GLOBAL.values[0]
        );
    }
}

#[link_section = ".modinfo"]
#[used]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";

