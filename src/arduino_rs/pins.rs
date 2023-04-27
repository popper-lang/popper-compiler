
#[no_std]
#[cfg(target_arch = "avr")]
use core::arch::asm;

#[cfg(target_arch = "avr")]
#[no_mangle]
pub fn initPin() {
    unsafe {
        asm!("
            ldi 0x04, 5
        ");
    }
}

#[cfg(target_arch = "avr")]
#[no_mangle]
#[repr(C)]
pub fn digitalWrite(status: cty::c_int) {
    unsafe {
        if status != 0 {
            asm!("
                ldi 0x05, 3
            ");
        } else {
            asm("
                cbi 0x05, 3
            ")
        }
    }
}