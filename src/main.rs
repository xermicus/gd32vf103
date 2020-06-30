#![no_std]
#![no_main]
#![feature(global_asm)]

use riscv_rt::entry;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

const RCU_APB2EN: u32 = 0x4002_1000 + 0x18;

const GPIOA_CTL0: u32 = 0x4001_0800 + 0x0;
const GPIOA_DATA: u32 = 0x4001_0800 + 0xc;

#[entry]
fn main() -> ! {
    init_ports();
    blink_led();
    loop {}
}

fn init_ports() {
    unsafe {
        // Enable clock to Port A and Port C
        let x = core::ptr::read_volatile(RCU_APB2EN as *mut u32);
        core::ptr::write_volatile(RCU_APB2EN as *mut u32, x | (1 << 2));
        // Enable push-pull o/p Port A, pins 1 and 2.
        let x = core::ptr::read_volatile(GPIOA_CTL0 as *mut u32);
        core::ptr::write_volatile(GPIOA_CTL0 as *mut u32, x | (1 << 4));
    }
}

// don't compile with optimization enabled!
fn delay(mut n: u32) {
    while n != 0 {
        n -= 1;
    }
}

// Blink Green LED (PA1).
fn blink_led() {
    let mut bits: u32 = !(1 << 1);
    loop {
        unsafe {
            // LED on when PA1 bit is 0
            core::ptr::write_volatile(GPIOA_DATA as *mut u32, bits);
        }
        delay(0x4ffff);
        bits = !bits;
    }
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

// With riscv_rt 0.6:
#[cfg(target_arch = "riscv32")]
global_asm!(
    r#"
lui sp, %hi(__stacktop)
call Reset
.globl abort
abort:
  jal zero, abort
"#
);

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
