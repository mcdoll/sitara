//! The UART devices

// Author: Moritz Doll
// License: MIT

use core::ops;
use core::fmt;
use core::arch::arm;
use register::{mmio::*, register_bitfields, Field};
use crate::device::console;


register_bitfields! {
    u32,
    DATA [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    LSR [
        RXFIFOE OFFSET(0) NUMBITS(1) [NotEmpty = 0b1]
    ],
    SSR [
        TXFIFOFULL OFFSET(0) NUMBITS(1) [Full = 0b1]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    DATA: ReadWrite<u32, DATA::Register>,
    _IER: u32,
    _IIR: u32,
    _LCR: u32,
    _MCR: u32,
    LSR: ReadOnly<u32, LSR::Register>,
    _TCR: u32,
    _TLR: u32,
    _MDR1: u32,
    _MDR2: u32,
    __reserved_0: [u32; 6],
    _SCR: u32,
    SSR: ReadOnly<u32, SSR::Register>
}

struct MemoryUart {
    memory_addr: u32,
}

impl ops::Deref for MemoryUart {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.ptr() }
    }
}

impl MemoryUart {
    fn new(mem_addr: u32) -> MemoryUart {
        MemoryUart {memory_addr: mem_addr }
    }
    fn ptr(&self) -> *mut RegisterBlock {
        self.memory_addr as *mut _
    }
}

pub struct Uart {
    memory: MemoryUart,
}

impl Uart {
    pub fn new(address: u32) -> Uart {
        let memory_block = MemoryUart::new(address);
        Uart { memory: memory_block }
    }

    #[inline]
    fn wait(&self, reg: Field<u32, SSR::Register>) {
        loop {
            if !self.memory.SSR.is_set(reg) {
                break;
            }
            unsafe { arm::__nop() };
        }
    }
    pub fn flush_txfifo(&self) {
        self.wait(SSR::TXFIFOFULL);
    }

    pub fn putc(&self, c: char) {
        self.wait(SSR::TXFIFOFULL);
        self.memory.DATA.set(c as u32);
    }
}

impl console::Console for Uart {
    fn getc(&self) -> char {
        loop {
            if self.memory.LSR.is_set(LSR::RXFIFOE) {
                break;
            }
            unsafe { arm::__nop() };
        }
        let mut ret = self.memory.DATA.get() as u8 as char;
        if ret == '\r' {
            ret = '\n'
        }
        ret
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.putc('\r');
                self.putc('\n');
            } else {
                self.putc(c);
            }
        }
        Ok(())
    }
}
