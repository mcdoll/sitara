//! Power control, reset, and clock management module
//!

use armv7::VirtualAddress;
use core::arch::arm;
use register::{mmio::*, register_bitfields, Field};

register_bitfields! {
    u32,
    CLKSEL_TIMER [
        CLKSEL OFFSET(0) NUMBITS(2) [
            TCLKIN = 0,
            CLK_M_OSC = 1,
            CLK_32KHZ = 2
        ]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterClockModuleDPLL {
    __reserved_0: u32,
    TIMER7: ReadWrite<u32, CLKSEL_TIMER::Register>,
    TIMER2: ReadWrite<u32, CLKSEL_TIMER::Register>,
    TIMER3: ReadWrite<u32, CLKSEL_TIMER::Register>,
    TIMER4: ReadWrite<u32, CLKSEL_TIMER::Register>,
    __reserved_0: u32,
    TIMER5: ReadWrite<u32, CLKSEL_TIMER::Register>,
    TIMER6: ReadWrite<u32, CLKSEL_TIMER::Register>,
}
