//! The Sitara watchdog timer
//!
// Author: Moritz Doll
// License: MIT


use core::ops;
use core::arch::arm;
use register::{mmio::*, register_bitfields, Field};

register_bitfields! {
    u32,
    WDT_WWPS [
        W_PEND_WCLR OFFSET(0) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WCRR OFFSET(1) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WLDR OFFSET(2) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WTGR OFFSET(3) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WSPR OFFSET(4) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WDLY OFFSET(5) NUMBITS(1) [Write = 1, NoWrite = 0]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    _WIDR: ReadOnly<u32, ()>,       // 0x0
    __reserved_0: [u32; 3],         // 0x4, 0x8, 0xc
    _WDSC: ReadWrite<u32, ()>,      // 0x10
    _WDST: ReadOnly<u32, ()>,       // 0x14
    _WISR: ReadWrite<u32, ()>,      // 0x18
    _WIER: ReadWrite<u32, ()>,      // 0x1c
    __reserved_1: u32,              // 0x20
    _WCLR: ReadWrite<u32, ()>,      // 0x24 
    _WCRR: ReadWrite<u32, ()>,      // 0x28
    _WLDR: ReadWrite<u32, ()>,      // 0x2c
    _WTGR: ReadWrite<u32, ()>,      // 0x30
    WWPS: ReadOnly<u32, WDT_WWPS::Register>,   //0x34
    __reserved_2 : [u32; 3],        // 0x38, 0x3c, 0x40
    _WDLY: ReadWrite<u32, ()>,      // 0x44
    WSPR: ReadWrite<u32, ()>,       // 0x48
}

struct WatchdogMemory {
    memory_addr: u32,
}

impl ops::Deref for WatchdogMemory {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.ptr() }
    }
}

impl WatchdogMemory {
    fn new(memory_addr: u32) -> Self {
        WatchdogMemory { memory_addr }
    }
    fn ptr(&self) -> *mut RegisterBlock {
        self.memory_addr as *mut _
    }
}

pub struct Watchdog {
    memory: WatchdogMemory
}

impl Watchdog {
    pub fn new(memory_addr: u32) -> Self {
        let memory = WatchdogMemory::new(memory_addr);
        Watchdog { memory }
    }

    #[inline]
    fn wait(&self, reg: Field<u32, WDT_WWPS::Register>) {
        loop {
            if !self.memory.WWPS.is_set(reg) {
                break;
            }
            unsafe { arm::__nop() };
        }
    }


    pub fn disable(&self) {
        self.memory.WSPR.set(0x0000_aaaa);
        self.wait(WDT_WWPS::W_PEND_WSPR);
        self.memory.WSPR.set(0x0000_5555);
        self.wait(WDT_WWPS::W_PEND_WSPR);
    }

    pub fn enable(&self) {
        self.memory.WSPR.set(0x0000_bbbb);
        self.wait(WDT_WWPS::W_PEND_WSPR);
        self.memory.WSPR.set(0x0000_4444);
        self.wait(WDT_WWPS::W_PEND_WSPR);
    }

    pub fn _alive() {
    }
}
