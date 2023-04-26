
#[cfg(target_arch = "avr")]
use core::arch::asm;



#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Configure PB5 as an output pin
        asm!("sbi 0x05, 5");

        loop {
            // Toggle PB5
            asm!("sbi 0x05, 5");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("cbi 0x05, 5");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
        }
    }
}