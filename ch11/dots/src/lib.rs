#![no_std]

extern crate alloc;

use crate::alloc::boxed::Box;
use linux_kernel_module::bindings;
use linux_kernel_module::c_types;
use linux_kernel_module::println;

use linux_kernel_module::bindings::__register_chrdev;
use linux_kernel_module::bindings::__unregister_chrdev;
use linux_kernel_module::bindings::_copy_to_user;

struct CharDeviceGlobalData {
    major: c_types::c_uint,
    name: &'static str,
    fops: Box<bindings::file_operations>,
    count: u64,
}

static mut GLOBAL: Option<CharDeviceGlobalData> = None;

extern "C" fn read_dot(
    _arg1: *mut bindings::file,
    arg2: *mut c_types::c_char,
    _arg3: usize,
    _arg4: *mut bindings::loff_t,
) -> isize {
    unsafe {
        if let Some(global) = &mut GLOBAL {
            global.count += 1;
            _copy_to_user(
                arg2 as *mut c_types::c_void,
                if global.count % 10 == 0 { "*" } else { "." }
                    .as_ptr() as *const c_types::c_void,
                1,
            );
            1
        }
        else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    let mut data = CharDeviceGlobalData {
        major: 0,
        name: "dots\0",
        fops: Box::new(bindings::file_operations::default()),
        count: 0,
    };
    data.fops.read = Some(read_dot);
    let major = unsafe {
        __register_chrdev(
            0,
            0,
            256,
            data.name.as_bytes().as_ptr() as *const i8,
            &*data.fops,
        )
    };
    if major < 0 { return 1; }
    data.major = major as c_types::c_uint;
    println!("dots: Loaded with major device number {}", major);
    unsafe {
        GLOBAL = Some(data);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        if let Some(global) = &GLOBAL {
            println!("dots: Unloaded {}", global.count);
            __unregister_chrdev(
                global.major,
                0,
                256,
                global.name.as_bytes().as_ptr() as *const i8,
            )
        }
    }
}

#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";

