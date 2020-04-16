#![no_std]

use linux_kernel_module::c_types;
use linux_kernel_module::println;

struct GlobalData { n: u16 }

static mut GLOBAL: GlobalData = GlobalData { n: 1000 };

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    println!("state: Loaded");
    unsafe { GLOBAL.n += 1; }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    println!("state: Unloaded {}", unsafe { GLOBAL.n });
}

#[link_section = ".modinfo"]
#[used]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";

