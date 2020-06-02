use core::panic::PanicInfo;
pub use crate::bindings;
pub use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
