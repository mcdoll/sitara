//! The interrupt controller of the AM335x Sitara SoC
// Author: Moritz Doll
// License: MIT

use armv7::VirtualAddress;
use register::{mmio::*, register_bitfields};
use core::fmt;

register_bitfields! {
    u32,
    REVISION [
        Rev OFFSET(0) NUMBITS(8) []
    ],
    SYSCONFIG [
        AutoIdle OFFSET(0) NUMBITS(1) [AutoIdle = 1],
        SoftReset OFFSET(1) NUMBITS(1) [SoftReset = 1]
    ],
    SYSSTATUS [
        ResetDone OFFSET(0) NUMBITS(1) []
    ],
    CONTROL [
        Irq OFFSET(0) NUMBITS(1) [NewIrqReset = 1],
        Fiq OFFSET(1) NUMBITS(1) [NewFiqReset = 1]
    ],
    IDLE [
        FuncIdle OFFSET(0) NUMBITS(1) [],
        Turbo OFFSET(1) NUMBITS(1) []
    ],
    pub ILR [
        FIQnIRQ OFFSET(0) NUMBITS(1) [IntIRQ = 0, IntFIQ = 1],
        Priority OFFSET(2) NUMBITS(6) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    _REVISION: ReadOnly<u32, REVISION::Register>, // 0x00
    __reserved_0: [u32; 3],
    SYSCONFIG: ReadWrite<u32, SYSCONFIG::Register>, // 0x10
    SYSSTATUS: ReadOnly<u32, SYSSTATUS::Register>,  // 0x14
    __reserved_1: [u32; 2 + 4 * 2],
    SIR_IRQ: ReadWrite<u32, ()>,                // 0x40
    SIR_FIQ: ReadWrite<u32, ()>,                // 0x44
    CONTROL: WriteOnly<u32, CONTROL::Register>, // 0x48
    _PROTECTION: u32,                           // 0x4C
    IDLE: ReadWrite<u32, IDLE::Register>,       // 0x50
    __reserved_2: [u32; 3],
    _IRQ_PRIORITY: u32, // 0x60
    _FIQ_PRIORITY: u32, // 0x64
    _THRESHOLD: u32,    // 0x68
                        //__reserved_3: [u32;1+4],
}

#[allow(non_snake_case)]
#[repr(C)]
struct BankBlock {
    ITR: ReadOnly<u32, ()>,         // 0x80
    MIR: ReadWrite<u32, ()>,        // 0x84
    MIR_CLEAR: WriteOnly<u32, ()>,  // 0x88
    MIR_SET: WriteOnly<u32, ()>,    // 0x8C
    ISR_SET: ReadWrite<u32, ()>,    // 0x90
    ISR_CLEAR: WriteOnly<u32, ()>,  // 0x94
    PENDING_IRQ: ReadOnly<u32, ()>, // 0x98
    PENDING_FIQ: ReadOnly<u32, ()>, // 0x9C
}

#[derive(Debug)]
pub enum InterruptRegister {
    Status,
    Mask,
    MaskClear,
    MaskSet,
    SoftwareSet,
    SoftwareClear,
    PendingIRQ,
    PendingFIQ,
}

/*
    ITR1: ReadOnly<u32, ()>,                            // 0xA0
    MIR1: ReadWrite<u32, ()>,                           // 0xA4
    MIR_CLEAR1: WriteOnly<u32, ()>,                     // 0xA8
    MIR_SET1: WriteOnly<u32, ()>,                       // 0xAC
    ISR_SET1: ReadWrite<u32, ()>,                       // 0xB0
    ISR_CLEAR1: WriteOnly<u32, ()>,                     // 0xB4
    PENDING_IRQ1: ReadOnly<u32, ()>,                    // 0xB8
    PENDING_FIQ1: ReadOnly<u32, ()>,                    // 0xBC
    ITR2: ReadOnly<u32, ()>,                            // 0xC0
    MIR2: ReadWrite<u32, ()>,                           // 0xC4
    MIR_CLEAR2: WriteOnly<u32, ()>,                     // 0xC8
    MIR_SET2: WriteOnly<u32, ()>,                       // 0xCC
    ISR_SET2: ReadWrite<u32, ()>,                       // 0xD0
    ISR_CLEAR2: WriteOnly<u32, ()>,                     // 0xD4
    PENDING_IRQ2: ReadOnly<u32, ()>,                    // 0xD8
    PENDING_FIQ2: ReadOnly<u32, ()>,                    // 0xDC
    ITR3: ReadOnly<u32, ()>,                            // 0xE0
    MIR3: ReadWrite<u32, ()>,                           // 0xE4
    MIR_CLEAR3: WriteOnly<u32, ()>,                     // 0xE8
    MIR_SET3: WriteOnly<u32, ()>,                       // 0xEC
    ISR_SET3: ReadWrite<u32, ()>,                       // 0xF0
    ISR_CLEAR3: WriteOnly<u32, ()>,                     // 0xF4
    PENDING_IRQ3: ReadOnly<u32, ()>,                    // 0xF8
    PENDING_FIQ3: ReadOnly<u32, ()>,                    // 0xFC
    ILR000: ReadWrite<u32, ILR::Register>,
    ILR001: ReadWrite<u32, ILR::Register>,
    ILR002: ReadWrite<u32, ILR::Register>,
    ILR003: ReadWrite<u32, ILR::Register>,
    ILR004: ReadWrite<u32, ILR::Register>,
    ILR005: ReadWrite<u32, ILR::Register>,
    ILR006: ReadWrite<u32, ILR::Register>,
    ILR007: ReadWrite<u32, ILR::Register>,
    ILR008: ReadWrite<u32, ILR::Register>,
    ILR009: ReadWrite<u32, ILR::Register>,
    ILR010: ReadWrite<u32, ILR::Register>,
    ILR011: ReadWrite<u32, ILR::Register>,
    ILR012: ReadWrite<u32, ILR::Register>,
    ILR013: ReadWrite<u32, ILR::Register>,
    ILR014: ReadWrite<u32, ILR::Register>,
    ILR015: ReadWrite<u32, ILR::Register>,
    ILR016: ReadWrite<u32, ILR::Register>,
    ILR017: ReadWrite<u32, ILR::Register>,
    ILR018: ReadWrite<u32, ILR::Register>,
    ILR019: ReadWrite<u32, ILR::Register>,
    ILR020: ReadWrite<u32, ILR::Register>,
    ILR021: ReadWrite<u32, ILR::Register>,
    ILR022: ReadWrite<u32, ILR::Register>,
    ILR023: ReadWrite<u32, ILR::Register>,
    ILR024: ReadWrite<u32, ILR::Register>,
    ILR025: ReadWrite<u32, ILR::Register>,
    ILR026: ReadWrite<u32, ILR::Register>,
    ILR027: ReadWrite<u32, ILR::Register>,
    ILR028: ReadWrite<u32, ILR::Register>,
    ILR029: ReadWrite<u32, ILR::Register>,
    ILR030: ReadWrite<u32, ILR::Register>,
    ILR031: ReadWrite<u32, ILR::Register>,
    ILR032: ReadWrite<u32, ILR::Register>,
    ILR033: ReadWrite<u32, ILR::Register>,
    ILR034: ReadWrite<u32, ILR::Register>,
    ILR035: ReadWrite<u32, ILR::Register>,
    ILR036: ReadWrite<u32, ILR::Register>,
    ILR037: ReadWrite<u32, ILR::Register>,
    ILR038: ReadWrite<u32, ILR::Register>,
    ILR039: ReadWrite<u32, ILR::Register>,
    ILR040: ReadWrite<u32, ILR::Register>,
    ILR041: ReadWrite<u32, ILR::Register>,
    ILR042: ReadWrite<u32, ILR::Register>,
    ILR043: ReadWrite<u32, ILR::Register>,
    ILR044: ReadWrite<u32, ILR::Register>,
    ILR045: ReadWrite<u32, ILR::Register>,
    ILR046: ReadWrite<u32, ILR::Register>,
    ILR047: ReadWrite<u32, ILR::Register>,
    ILR048: ReadWrite<u32, ILR::Register>,
    ILR049: ReadWrite<u32, ILR::Register>,
    ILR050: ReadWrite<u32, ILR::Register>,
    ILR051: ReadWrite<u32, ILR::Register>,
    ILR052: ReadWrite<u32, ILR::Register>,
    ILR053: ReadWrite<u32, ILR::Register>,
    ILR054: ReadWrite<u32, ILR::Register>,
    ILR055: ReadWrite<u32, ILR::Register>,
    ILR056: ReadWrite<u32, ILR::Register>,
    ILR057: ReadWrite<u32, ILR::Register>,
    ILR058: ReadWrite<u32, ILR::Register>,
    ILR059: ReadWrite<u32, ILR::Register>,
}
*/

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct InterruptNumber(u8);

impl InterruptNumber {
    pub fn new(number: u8) -> Option<InterruptNumber> {
        if number > 127 {
            None
        } else {
            Some(InterruptNumber(number))
        }
    }
    pub fn from_u32(number: u32) -> InterruptNumber {
        let bitmask = 127; // = 0x7f
        let truncated = number | bitmask;
        InterruptNumber(truncated as u8)
    }
    fn get_shift(&self) -> u32 {
        (self.0 % 32) as u32
    }
    fn get_offset(&self) -> u32 {
        (self.0 / 32) as u32
    }
    fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug)]
pub struct InterruptLine {
    number: InterruptNumber,
    memory_address: u32,
}

impl InterruptLine {
    pub fn new(number: InterruptNumber, memory_address: u32) -> InterruptLine {
        InterruptLine {
            number,
            memory_address,
        }
    }
    #[inline]
    fn get_bitmask(&self) -> u32 {
        1 << self.number.get_shift()
    }
    #[inline]
    fn get_bank_addr(&self) -> u32 {
        self.memory_address + 0x80 + self.number.get_offset() * 0x20
    }
    fn get_bank_ptr(&self) -> &BankBlock {
        let raw_ptr = self.get_bank_addr() as *mut BankBlock;
        unsafe { &*raw_ptr }
    }
    #[inline]
    fn get_ilr_addr(&self) -> u32 {
        self.memory_address + 0x100 + self.number.as_u32() * 0x4
    }
    // Gives a pointer to the corresponding ILR register
    pub fn get_ilr_ptr(&self) -> &ReadWrite<u32, ILR::Register> {
        let raw_ptr = self.get_ilr_addr() as *mut ReadWrite<u32, ILR::Register>;
        unsafe { &*raw_ptr }
    }
    pub fn reg_read(&self, reg: InterruptRegister) -> bool {
        let val = match reg {
            InterruptRegister::Status => self.get_bank_ptr().ITR.get(),
            InterruptRegister::Mask => self.get_bank_ptr().MIR.get(),
            InterruptRegister::MaskClear => 0, //panic!("Cannot read MIR_CLR"),
            InterruptRegister::MaskSet => 0,   //panic!("Cannot read MIR_SET"),
            InterruptRegister::SoftwareSet => self.get_bank_ptr().ISR_SET.get(),
            InterruptRegister::SoftwareClear => 0, //panic!("Cannot read ISR_CLR"),
            InterruptRegister::PendingIRQ => self.get_bank_ptr().PENDING_IRQ.get(),
            InterruptRegister::PendingFIQ => self.get_bank_ptr().PENDING_FIQ.get(),
        };
        (val | self.get_bitmask()) != 0
    }
    pub fn reg_write(&self, reg: InterruptRegister, val: bool) {
        let bitmask = self.get_bitmask();
        match (val, reg) {
            (_, InterruptRegister::Mask) => {
                let bitset = if val { bitmask } else { 0 };
                let current_mask = self.get_bank_ptr().MIR.get();
                // remove the bit to be changed
                let new_mask = (current_mask & !bitmask) | bitset;
                // add the new bit
                self.get_bank_ptr().MIR.set(new_mask);
            }
            (true, InterruptRegister::MaskClear) => self.get_bank_ptr().MIR_CLEAR.set(bitmask),
            (true, InterruptRegister::MaskSet) => self.get_bank_ptr().MIR_SET.set(bitmask),
            (true, InterruptRegister::SoftwareSet) => self.get_bank_ptr().ISR_SET.set(bitmask),
            (true, InterruptRegister::SoftwareClear) => self.get_bank_ptr().ISR_CLEAR.set(bitmask),
            _ => {}
        };
    }
    pub fn get_itr(&self) -> bool {
        let val = self.get_bank_ptr().ITR.get();
        (val | self.get_bitmask()) != 0
    }
    pub fn enable(&self) {
        self.get_bank_ptr().MIR_CLEAR.set(self.get_bitmask());
    }
    pub fn disable(&self) {
        self.get_bank_ptr().MIR_SET.set(self.get_bitmask());
    }
    pub fn pending(&self) -> bool {
        self.reg_read(InterruptRegister::PendingIRQ) || self.reg_read(InterruptRegister::PendingIRQ)
    }
    pub fn debug_set_software_irq(&self) {
        self.get_bank_ptr().ISR_SET.set(self.get_bitmask());
    }
    pub fn debug_clear_software_irq(&self) {
        self.get_bank_ptr().ISR_CLEAR.set(self.get_bitmask());
    }
}

#[derive(Debug)]
pub struct InterruptController {
    memory_address: u32,
}

impl InterruptController {
    /// Get an interrupt controller
    ///
    /// # Safety
    /// The caller has to make sure that the virtual address maps to the interrupt controller
    pub unsafe fn new(memory_address: VirtualAddress) -> Self {
        InterruptController {
            memory_address: memory_address.as_u32(),
        }
    }
    /// Print the raw status of all interrupt lines
    pub fn dump_raw_status<T: fmt::Write>(&self, writer: &mut T) -> fmt::Result {
        writeln!(writer, "Dumping raw irq controller status:")?;
        let raw_ptr = (self.memory_address + 0x80) as *const u32;
        let itr0 = unsafe { *raw_ptr };
        let raw_ptr = (self.memory_address + 0xa0) as *const u32;
        let itr1 = unsafe { *raw_ptr };
        let raw_ptr = (self.memory_address + 0xc0) as *const u32;
        let itr2 = unsafe { *raw_ptr };
        let raw_ptr = (self.memory_address + 0xe0) as *const u32;
        let itr3 = unsafe { *raw_ptr };
        writeln!(writer, "ITR0 {:#x}, ITR1 {:#x}, ITR2 {:#x}, ITR3 {:#x}", itr0, itr1, itr2, itr3)?;
        Ok(())
    }
    pub fn new_interrupt_line(&self, number: InterruptNumber) -> InterruptLine {
        InterruptLine::new(number, self.memory_address)
    }
    fn get_ptr(&self) -> &RegisterBlock {
        let raw_ptr = self.memory_address as *mut RegisterBlock;
        unsafe { &*raw_ptr }
    }
    /// Get the currently active IRQ
    ///
    /// # Safety
    /// Should make sure that the interrupt number is valid
    pub fn get_active_irq(&self) -> InterruptNumber {
        let active_irq = self.get_ptr().SIR_IRQ.get();
        InterruptNumber::from_u32(active_irq)
    }
    /// Get the currently active FIQ
    ///
    /// # Safety
    /// Should make sure that the interrupt number is valid
    pub fn get_active_fiq(&self) -> InterruptNumber {
        let active_fiq = self.get_ptr().SIR_FIQ.get();
        InterruptNumber::from_u32(active_fiq)
    }
    pub fn reset(&self) {
        self.get_ptr()
            .SYSCONFIG
            .write(SYSCONFIG::SoftReset::SoftReset);
    }
    pub fn autoidle(&self) {
        self.get_ptr()
            .SYSCONFIG
            .write(SYSCONFIG::AutoIdle::AutoIdle);
    }
    pub fn generate_new_irq(&self) {
        self.get_ptr().CONTROL.write(CONTROL::Irq::NewIrqReset);
    }
    pub fn generate_new_fiq(&self) {
        self.get_ptr().CONTROL.write(CONTROL::Fiq::NewFiqReset);
    }
}
