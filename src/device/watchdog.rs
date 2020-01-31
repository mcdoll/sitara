//! The Sitara watchdog timer
//!
// Author: Moritz Doll
// License: MIT

use core::arch::arm;
use register::{mmio::*, register_bitfields, Field};
use armv7::VirtualAddress;

register_bitfields! {
    u32,
    WDT_WWPS [
        W_PEND_WCLR OFFSET(0) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WCRR OFFSET(1) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WLDR OFFSET(2) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WTGR OFFSET(3) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WSPR OFFSET(4) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_WDLY OFFSET(5) NUMBITS(1) [Write = 1, NoWrite = 0]
    ],
    WDT_WIRQ [
        OVERFLOW OFFSET(0) NUMBITS(1) [Enable = 1, Disable = 0],
        DELAY OFFSET(1) NUMBITS(1) [Enable = 1, Disable = 0]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    _WIDR: ReadOnly<u32, ()>,                        // 0x0
    __reserved_0: [u32; 3],                          // 0x4, 0x8, 0xc
    _WDSC: ReadWrite<u32, ()>,                       // 0x10
    _WDST: ReadOnly<u32, ()>,                        // 0x14
    WISR: ReadWrite<u32, WDT_WIRQ::Register>,        // 0x18
    WIER: ReadWrite<u32, WDT_WIRQ::Register>,        // 0x1c
    __reserved_1: u32,                               // 0x20
    _WCLR: ReadWrite<u32, ()>,                       // 0x24
    _WCRR: ReadWrite<u32, ()>,                       // 0x28
    WLDR: ReadWrite<u32, ()>,                        // 0x2c
    WTGR: ReadWrite<u32, ()>,                        // 0x30
    WWPS: ReadOnly<u32, WDT_WWPS::Register>,         // 0x34
    __reserved_2: [u32; 3],                          // 0x38, 0x3c, 0x40
    WDLY: ReadWrite<u32, ()>,                        // 0x44
    WSPR: ReadWrite<u32, ()>,                        // 0x48
    __reserved_3: [u32; 2],                          // 0x4c, 0x50
    WIRQSTATRAW: ReadWrite<u32, WDT_WIRQ::Register>, // 0x54
    WIRQSTAT: ReadWrite<u32, WDT_WIRQ::Register>,    // 0x58
    WIRQENSET: ReadWrite<u32, WDT_WIRQ::Register>,   // 0x5c
    WIRQENCLR: ReadWrite<u32, WDT_WIRQ::Register>,   // 0x60
}

pub struct Watchdog {
    memory: &'static RegisterBlock,
    counter: u32,
}

impl Watchdog {
    pub unsafe fn new(memory_addr: VirtualAddress) -> Self {
        let memory = &*(memory_addr.as_u32() as *mut RegisterBlock);
        let counter = memory.WTGR.get();
        Watchdog { memory, counter }
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

    pub fn enable_irq(&self) {
        self.memory
            .WIRQENSET
            .write(WDT_WIRQ::DELAY::Enable + WDT_WIRQ::OVERFLOW::Enable);
    }
    pub fn disable_irq(&self) {
        self.memory
            .WIRQENCLR
            .write(WDT_WIRQ::DELAY::Enable + WDT_WIRQ::OVERFLOW::Enable);
    }
    pub fn enable_delay_irq(&self) {
        self.memory.WIRQENSET.write(WDT_WIRQ::DELAY::Enable);
    }
    pub fn disable_delay_irq(&self) {
        self.memory.WIRQENCLR.write(WDT_WIRQ::DELAY::Enable);
    }
    pub fn enable_overflow_irq(&self) {
        self.memory.WIRQENSET.write(WDT_WIRQ::OVERFLOW::Enable);
    }
    pub fn disable_overflow_irq(&self) {
        self.memory.WIRQENCLR.write(WDT_WIRQ::OVERFLOW::Enable);
    }

    pub fn trigger(&mut self) {
        if self.counter == 0xffff_ffff {
            self.counter = 0;
        } else {
            self.counter += 1;
        }
        self.memory.WTGR.set(self.counter);
    }
}
