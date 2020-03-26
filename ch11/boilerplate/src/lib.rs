#![no_std]

//extern crate alloc;
//use crate::alloc::string::{String, ToString};
use linux_kernel_module::c_types;
use linux_kernel_module::println;

struct BoilerplateModule {
    id: u32,
    //message: String,
}

impl linux_kernel_module::KernelModule for BoilerplateModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let id = 1237;
        println!("boilerplate: loaded {}", id);
        Ok(BoilerplateModule {
            id: id,
            //message: id.to_string(),
        })
    }
}

impl Drop for BoilerplateModule {
    fn drop(&mut self) {
        //println!("boilerplate: unloaded {}, {}", self.id, self.message);
        println!("boilerplate: unloaded {}", self.id);
    }
}

static mut MODULE: Option<BoilerplateModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <BoilerplateModule as linux_kernel_module::KernelModule>::init() {
        Ok(m) => {
            unsafe {
                MODULE = Some(m);
            }
            return 0;
        }
        Err(_e) => {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        MODULE = None;
    }
}

#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
