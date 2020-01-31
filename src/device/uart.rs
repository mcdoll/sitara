//! The UART devices

// Author: Moritz Doll
// License: MIT

use crate::device::console;
use core::arch::arm;
use core::fmt;
use register::{mmio::*, register_bitfields, Field};
use armv7::VirtualAddress;

register_bitfields! {
    u32,
    DATA [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    IER [
        RHR OFFSET(0) NUMBITS(1) [Enable = 1, Disable = 0],
        THR OFFSET(1) NUMBITS(1) [Enable = 1, Disable = 0],
        LINESTS OFFSET(2) NUMBITS(1) [Enable = 1, Disable = 0],
        MODEMSTS OFFSET(3) NUMBITS(1) [Enable = 1, Disable = 0],
        SLEEPMODE OFFSET(4) NUMBITS(1) [Enable = 1, Disable = 0],
        XOFF OFFSET(5) NUMBITS(1) [Enable = 1, Disable = 0],
        RTS OFFSET(6) NUMBITS(1) [Enable = 1, Disable = 0],
        CTS OFFSET(7) NUMBITS(1) [Enable = 1, Disable = 0]
    ],
    IIR [
        PENDING OFFSET(0) NUMBITS(1) [Active = 1, NonActive = 0],
        IT_TYPE OFFSET(1) NUMBITS(5) [
            Modem = 0,
            Thr = 1,
            Rhr = 2,
            LineStatus = 3,
            RxTimeout = 6,
            Xoff = 8,
            Cts = 10
        ],
        FCR_MIRROR OFFSET(0) NUMBITS(2) []
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
    SSR: ReadOnly<u32, SSR::Register>,
}

pub struct Uart {
    memory: &'static RegisterBlock,
}

impl Uart {
    pub unsafe fn new(memory_addr: VirtualAddress) -> Uart {
        let memory = &*(memory_addr.as_u32() as *mut RegisterBlock);
        Uart { memory }
    }
    pub unsafe fn new_from_u32(memory_addr: u32) -> Uart {
        let memory = &*(memory_addr as *mut RegisterBlock);
        Uart { memory }
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
