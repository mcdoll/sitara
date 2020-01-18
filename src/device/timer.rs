//! Timer for the Sitara SoC
//!
//! Author: Moritz Doll
//! License: MIT

use register::{mmio::*, register_bitfields, Field};
use core::ops;
use core::arch::arm;

register_bitfields! {
    u32,
    MODE [
        MATCH OFFSET(0) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        OVERFLOW OFFSET(1) NUMBITS(1) [Enable = 0b1, Disable = 0b0],
        CAPTURE OFFSET(2) NUMBITS(1) [Enable = 0b1, Disable = 0b0]
    ],
    TCLR [
        ST OFFSET(0) NUMBITS(1) [Start = 1, Stop = 0],
        AR OFFSET(1) NUMBITS(1) [],
        PTV OFFSET(2) NUMBITS(3) [],
        PRE OFFSET(5) NUMBITS(1) [PrescaleEnable = 1, PrescaleDisable = 0],
        CE OFFSET(6) NUMBITS(1) [],
        SCPWM OFFSET(7) NUMBITS(1) [],
        TCM OFFSET(8) NUMBITS(2) [NoCapture = 0, LowToHigh = 1, HighToLow = 2, Both = 3],
        TRG OFFSET(10) NUMBITS(2) [NoTrigger = 0, TriggerOverflow = 1, TriggerOverflowMatch = 2],
        PT OFFSET(12) NUMBITS(1) [],
        CAPT_MODE OFFSET(12) NUMBITS(1) [],
        GPO_CFG OFFSET(12) NUMBITS(1) []
    ],
    TWPS [
        W_PEND_TCLR OFFSET(0) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_TCRR OFFSET(1) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_TLDR OFFSET(2) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_TTGR OFFSET(3) NUMBITS(1) [Write = 1, NoWrite = 0],
        W_PEND_TMAR OFFSET(4) NUMBITS(1) [Write = 1, NoWrite = 0]
    ]
        
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    _TIDR: ReadWrite<u32, ()>,
    _TIOCP_CFG: ReadWrite<u32, ()>,
    _IRQ_EOI: ReadWrite<u32, ()>,
    _IRQSTATUS_RAW: ReadWrite<u32, MODE::Register>,
    _IRQSTATUS: ReadWrite<u32, MODE::Register>,
    IRQENABLE_SET: ReadWrite<u32, MODE::Register>,
    _IRQENABLE_CLR: ReadWrite<u32, MODE::Register>,
    TCLR: ReadWrite<u32, TCLR::Register>,
    _TCRR: ReadWrite<u32, ()>,
    TLDR: ReadWrite<u32, ()>,
    _TCGR: ReadWrite<u32, ()>,
    TWPS: ReadOnly<u32, TWPS::Register>,
    _TMAR: ReadWrite<u32, ()>,
    _TCAR1: ReadWrite<u32, ()>,
    _TSICR: ReadWrite<u32, ()>,
    _TCAR2: ReadWrite<u32, ()>,
}

struct TimerMemory {
    memory_addr: u32,
}

impl ops::Deref for TimerMemory {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.ptr() }
    }
}

impl TimerMemory {
    fn new(mem_addr: u32) -> Self {
        TimerMemory {memory_addr: mem_addr }
    }
    fn ptr(&self) -> *mut RegisterBlock {
        self.memory_addr as *mut _
    }
}

pub struct Timer {
    memory: TimerMemory
}

impl Timer {
    pub fn new(memory_addr: u32) -> Self {
        let memory = TimerMemory::new(memory_addr);
        Timer { memory: memory }
    }
    pub fn start(&self) {
        self.memory.TCLR.modify(TCLR::ST::Start);
        self.wait(TWPS::W_PEND_TCLR);
    }
    pub fn stop(&self) {
        self.memory.TCLR.modify(TCLR::ST::Stop);
        self.wait(TWPS::W_PEND_TCLR);
    }
    pub fn init(&self, length: u32) {
        self.memory.TCLR.modify(TCLR::ST::Stop + TCLR::PRE::PrescaleDisable);
        self.wait(TWPS::W_PEND_TCLR);
        let timer_rate = 0xffff_ffff - length;
        self.memory.TLDR.set(timer_rate);
        self.wait(TWPS::W_PEND_TLDR);
        self.memory.IRQENABLE_SET.write(MODE::OVERFLOW::Enable);
        self.start();
    }
    #[inline]
    fn wait(&self, reg: Field<u32, TWPS::Register>) {
        loop {
            if !self.memory.TWPS.is_set(reg) {
                break;
            }
            unsafe { arm::__nop() };
        }
    }
}
