//! The GPIO

// Author: Moritz Doll
// License: MIT

use armv7::VirtualAddress;
use core::marker::PhantomData;
use register::{mmio::*, register_bitfields};

register_bitfields! {
    u32,
    REVISION [
        MINOR OFFSET(0) NUMBITS(6) [],
        MAJOR OFFSET(8) NUMBITS(3) []
    ],
    SYSCONFIG [
        AUTOIDLE OFFSET(0) NUMBITS(1) [AutoIdle = 1, FreeRunning = 0],
        SOFTRESET OFFSET(1) NUMBITS(1) [SoftReset = 1, NormalMode = 0],
        WAKEUP OFFSET(2) NUMBITS(1) [Enable = 1, Disable = 0],
        IDLEMODE OFFSET(3) NUMBITS(2) [
            ForceIdle = 0,
            NoIdle = 1,
            SmartIdle = 2,
            SmartIdleWakeup = 3
        ]
    ],
    SYSSTATUS [
        RESETDONE OFFSET(0) NUMBITS(1) []
    ],
    CTRL [
        DISABLEMODULE OFFSET(0) NUMBITS(1) [],
        GATINGRATIO OFFSET(1) NUMBITS(2) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    _REVISION: ReadOnly<u32, REVISION::Register>,
    __reserved_0: [u32; 3],
    SYSCONFIG: ReadWrite<u32, SYSCONFIG::Register>,
    __reserved_1: [u32; 3],
    _EOI: u32,
    IRQSTATUS_RAW_0: ReadWrite<u32, ()>,
    IRQSTATUS_RAW_1: ReadWrite<u32, ()>,
    IRQSTATUS_0: ReadWrite<u32, ()>,
    IRQSTATUS_1: ReadWrite<u32, ()>,
    IRQSTATUS_SET_0: ReadWrite<u32, ()>,
    IRQSTATUS_SET_1: ReadWrite<u32, ()>,
    IRQSTATUS_CLR_0: ReadWrite<u32, ()>,
    IRQSTATUS_CLR_1: ReadWrite<u32, ()>,
    IRQWAKEN_0: ReadWrite<u32, ()>,
    IRQWAKEN_1: ReadWrite<u32, ()>, // 0x48
    __reserved_2: [u32; 50],
    SYSSTATUS: ReadOnly<u32, SYSSTATUS::Register>, // 0x114
    __reserved_3: [u32; 6],                        // 0x118, 11C, 120, 124, 128, 12c
    CTRL: ReadWrite<u32, CTRL::Register>,          // 0x130
    OE: ReadWrite<u32, ()>,                        // 0x134
    DATAIN: ReadOnly<u32, ()>,                     // 0x138
    DATAOUT: ReadWrite<u32, ()>,                   // 0x13C
    LEVELDETECT_0: ReadWrite<u32, ()>,             // 0x140
    LEVELDETECT_1: ReadWrite<u32, ()>,             // 0x144
    RISINGDETECT: ReadWrite<u32, ()>,              // 0x148
    FALLINGDETECT: ReadWrite<u32, ()>,             // 0x14C
    DEBOUNCEENABLE: ReadWrite<u32, ()>,            // 0x150
    DEBOUNINGTIME: ReadWrite<u32, ()>,             // 0x154
    __reserved_4: [u32; 14],
    CLEARDATAOUT: ReadWrite<u32, ()>, // 0x190
    SETDATAOUT: ReadWrite<u32, ()>,   // 0x194
}

#[derive(Debug)]
pub struct Output;
#[derive(Debug)]
pub struct Input;

pub struct Pin<T> {
    number: u8,
    memory: &'static RegisterBlock,
    gpio_type: PhantomData<T>,
}

impl<T> Pin<T> {
    fn new(number: u8, memory: &'static RegisterBlock) -> Self {
        Pin {
            number,
            memory,
            gpio_type: PhantomData,
        }
    }
    #[inline]
    fn bitmask(&self) -> u32 {
        1 << (self.number as u32)
    }
}

impl Pin<Output> {
    pub fn to_input(self) -> Pin<Output> {
        unimplemented! {}
    }
    pub fn read(&self) -> bool {
        (self.memory.DATAOUT.get() & self.bitmask()) != 0
    }
    pub fn set(&self) {
        self.memory.SETDATAOUT.set(self.bitmask());
        //self.memory.DATAOUT.set(self.bitmask());
    }
    pub fn clear(&self) {
        self.memory.CLEARDATAOUT.set(self.bitmask());
    }
    pub fn switch(&self) {
        let bits = self.memory.DATAOUT.get() ^ self.bitmask();
        self.memory.DATAOUT.set(bits);
    }
}

impl Pin<Input> {
    pub fn to_output(self) -> Pin<Output> {
        unimplemented! {}
    }
}

pub struct Gpio {
    memory: &'static RegisterBlock,
    owned: u32,
}

impl Gpio {
    pub unsafe fn new(memory_addr: VirtualAddress) -> Self {
        let memory = &*(memory_addr.as_u32() as *mut RegisterBlock);
        Gpio { memory, owned: 0 }
    }
    pub fn get_pin_as_input(&mut self, number: u8) -> Option<Pin<Input>> {
        if number > 31 {
            return None;
        }
        let bit = 1 << (number as u32);
        // Check whether the pin was already given to someone
        if self.owned & bit == 1 {
            return None;
        }
        self.owned |= bit;
        let bitset = self.memory.OE.get();
        self.memory.OE.set(bitset | bit);
        Some(Pin::new(number, self.memory))
    }
    pub fn get_pin_as_output(&mut self, number: u8) -> Option<Pin<Output>> {
        if number > 31 {
            return None;
        }
        let bit = 1 << (number as u32);
        // Check whether the pin was already given to someone
        if self.owned & bit == 1 {
            return None;
        }
        self.owned |= bit;
        let bitset = self.memory.OE.get();
        self.memory.OE.set(bitset & !bit);
        Some(Pin::new(number, self.memory))
    }
}
