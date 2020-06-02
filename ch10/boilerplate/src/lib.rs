#![no_std]

use linux_kernel_module::c_types;
use linux_kernel_module::println;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    println!("boilerplate: Loaded");
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    println!("boilerplate: Unloaded");
}

#[link_section = ".modinfo"]
#[used]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
