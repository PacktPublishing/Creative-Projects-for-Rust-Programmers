#![no_std]

extern crate alloc;

use crate::alloc::boxed::Box;
use linux_kernel_module::bindings::{
    __register_chrdev, __unregister_chrdev, _copy_to_user, file, file_operations, loff_t,
};
use linux_kernel_module::c_types;
use linux_kernel_module::println;

struct CharDeviceGlobalData {
    major: c_types::c_uint,
    name: &'static str,
    fops: Option<Box<file_operations>>,
    count: u64,
}

static mut GLOBAL: CharDeviceGlobalData = CharDeviceGlobalData {
    major: 0,
    name: "dots\0",
    fops: None,
    count: 0,
};

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    let mut fops = Box::new(file_operations::default());
    fops.read = Some(read_dot);
    let major = unsafe {
        __register_chrdev(
            0,
            0,
            256,
            GLOBAL.name.as_bytes().as_ptr() as *const i8,
            &*fops,
        )
    };
    if major < 0 {
        return 1;
    }
    unsafe {
        GLOBAL.major = major as c_types::c_uint;
    }
    println!("dots: Loaded with major device number {}", major);
    unsafe {
        GLOBAL.fops = Some(fops);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        println!("dots: Unloaded {}", GLOBAL.count);
        __unregister_chrdev(
            GLOBAL.major,
            0,
            256,
            GLOBAL.name.as_bytes().as_ptr() as *const i8,
        )
    }
}

extern "C" fn read_dot(
    _arg1: *mut file,
    arg2: *mut c_types::c_char,
    _arg3: usize,
    _arg4: *mut loff_t,
) -> isize {
    unsafe {
        GLOBAL.count += 1;
        _copy_to_user(
            arg2 as *mut c_types::c_void,
            if GLOBAL.count % 10 == 0 { "*" } else { "." }.as_ptr() as *const c_types::c_void,
            1,
        );
        1
    }
}

#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
