//! The UART devices

use crate::device::console;
use armv7::VirtualAddress;
use core::arch::arm;
use core::fmt;
use register::{mmio::*, register_bitfields, Field};

register_bitfields! {
    u32,
    DATA [
        DATA OFFSET(0) NUMBITS(8) []
    ],
    DLL [
        CLOCK OFFSET(0) NUMBITS(8) []
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
    DLH [
        CLOCK_MSB OFFSET(0) NUMBITS(6) []
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
    EFR [
        ENHANCED OFFSET(4) NUMBITS(1) [Enable = 1, Disable = 0]
    ],
    LCR [
        CHAR_LENGTH OFFSET(0) NUMBITS(2) [
            BIT5 = 0,
            BIT6 = 1,
            BIT7 = 2,
            BIT8 = 3
        ],
        NB_STOP OFFSET(2) NUMBITS(1) [],
        PARITY OFFSET(3) NUMBITS(1) [Enable = 1, Disable = 0],
        PARITY_TYPE OFFSET(4) NUMBITS(1) [Odd = 0, Even = 1],
        PARITY_TYPE2 OFFSET(5) NUMBITS(1) [Normal = 0, Force = 1],
        BREAK OFFSET(6) NUMBITS(1) [Normal = 0, ForceTXLow = 1],
        DIV_EN OFFSET(7) NUMBITS(1) [Normal = 0, DivisiorLatchEnable = 1]
    ],
    MCR [
        DTR OFFSET(0) NUMBITS(1) [High = 0, Low = 1],
        RTS OFFSET(1) NUMBITS(1) [High = 0, Low = 1],
        RISTSCH OFFSET(2) NUMBITS(1) [High = 0, Low = 1],
        CDSTSCH OFFSET(3) NUMBITS(1) [],
        LOOPBACK OFFSET(4) NUMBITS(1) [],
        XONEN OFFSET(5) NUMBITS(1) [],
        TCRTLR OFFSET(6) NUMBITS(1) []
    ],
    LSR [
        RXFIFOE OFFSET(0) NUMBITS(1) [NotEmpty = 0b1]
    ],
    TCR [
        RX_FIFO_TRIG_HALT OFFSET(0) NUMBITS(4) [],
        RX_FIFO_TRIG_START OFFSET(4) NUMBITS(4) []
    ],
    TLR [
        TX_FIFO_TRIG_DMA OFFSET(0) NUMBITS(4) [],
        RX_FIFO_TRIG_DMA OFFSET(4) NUMBITS(4) []
    ],
    MDR1 [
        MODESELECT OFFSET(0) NUMBITS(3) [
            Uart16 = 0,
            Sir = 1,
            Uart16Auto = 2,
            Uart13 = 3,
            Mir = 4,
            Fir = 5,
            Cir = 6,
            Disable = 7
        ]
    ],
    MDR2 [
        IRTXUNDERRUN OFFSET(0) NUMBITS(1) []
    ],
    UASR [
        SPEED OFFSET(0) NUMBITS(5) [],
        BITBYCHAR OFFSET(5) NUMBITS(1) [],
        PARITYTYPE OFFSET(6) NUMBITS(2) []
    ],
    SCR [
        DMAMODECTL OFFSET(0) NUMBITS(1) []
    ],
    SSR [
        TXFIFOFULL OFFSET(0) NUMBITS(1) [Full = 0b1]
    ],
    MVR [
        MINORREV OFFSET(0) NUMBITS(6) [],
        MAJORREV OFFSET(8) NUMBITS(3) []
    ],
    SYSC [
        AUTOIDLE OFFSET(0) NUMBITS(1) [],
        SOFTRESET OFFSET(1) NUMBITS(1) [Reset = 1],
        ENAWAKEUP OFFSET(2) NUMBITS(1) [Enable = 1, Disable = 0],
        IDLEMODE OFFSET(3) NUMBITS(2) [
            ForceIdle = 0,
            NoIdle = 1,
            SmartIdle = 2,
            IdleWakeup = 3
        ]
    ],
    SYSS [
        RESETDONE OFFSET(0) NUMBITS(1) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    // This struct is for the normal operation mode
    DATA: ReadWrite<u32, DATA::Register>, // 0x00
    IER: ReadWrite<u32, IER::Register>,   // 0x04
    IIR: ReadWrite<u32, IIR::Register>,   // 0x08
    LCR: ReadWrite<u32, LCR::Register>,   // 0x0C
    MCR: ReadWrite<u32, MCR::Register>,   // 0x10
    LSR: ReadOnly<u32, LSR::Register>,    // 0x14
    TCR: ReadWrite<u32, TCR::Register>,   // 0x18
    TLR: ReadWrite<u32, TLR::Register>,   // 0x1C
    MDR1: ReadWrite<u32, MDR1::Register>, // 0x20
    MDR2: ReadWrite<u32, MDR2::Register>, // 0x24
    __reserved_0: [u32; 6],
    SCR: ReadWrite<u32, SCR::Register>, // 0x40
    SSR: ReadOnly<u32, SSR::Register>,  // 0x44
    __reserved_1: [u32; 2],
    MVR: ReadOnly<u32, MVR::Register>,    // 0x50
    SYSC: ReadWrite<u32, SYSC::Register>, // 0x54
    SYSS: ReadOnly<u32, SYSS::Register>,  // 0x58
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlockConfigB {
    // This struct is for the normal operation mode
    DLL: ReadWrite<u32, DLL::Register>,   // 0x00
    DLH: ReadWrite<u32, DLH::Register>,   // 0x04
    EFR: ReadWrite<u32, EFR::Register>,   // 0x08
    LCR: ReadWrite<u32, LCR::Register>,   // 0x0C
    MCR: ReadWrite<u32, MCR::Register>,   // 0x10
    LSR: ReadOnly<u32, LSR::Register>,    // 0x14
    TCR: ReadWrite<u32, TCR::Register>,   // 0x18
    TLR: ReadWrite<u32, TLR::Register>,   // 0x1C
    MDR1: ReadWrite<u32, MDR1::Register>, // 0x20
    MDR2: ReadWrite<u32, MDR2::Register>, // 0x24
    __reserved_0: [u32; 4],
    UASR: ReadOnly<u32, UASR::Register>, // 0x38
    __reserved_1: u32,
    SCR: ReadWrite<u32, SCR::Register>, // 0x40
    SSR: ReadOnly<u32, SSR::Register>,  // 0x44
    __reserved_2: [u32; 2],
    MVR: ReadOnly<u32, MVR::Register>,    // 0x50
    SYSC: ReadWrite<u32, SYSC::Register>, // 0x54
    SYSS: ReadOnly<u32, SYSS::Register>,  // 0x58
}

pub enum BaudRate {
    Baud115200,
}

pub struct UartConfigB {
    memory: &'static RegisterBlockConfigB,
    saved_lcr: u32,
}
impl UartConfigB {
    pub unsafe fn new(memory_addr: VirtualAddress, saved_lcr: u32) -> Self {
        let memory = &*(memory_addr.as_u32() as *mut RegisterBlockConfigB);
        Self { memory, saved_lcr }
    }
    pub fn to_operating_mode(self) -> Uart {
        self.memory.LCR.set(self.saved_lcr);
        let raw_ptr = self.memory as *const RegisterBlockConfigB;
        unsafe { Uart::new(VirtualAddress::from_ptr(raw_ptr)) }
    }
    pub fn enable_all_ier(&self) {
        self.memory.EFR.write(EFR::ENHANCED::Enable);
    }
    pub fn set_baud(&self) {
        self.memory.DLH.set(0);
        self.memory.DLL.set(0x1A);
    }
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

    pub unsafe fn reset(&self) {
        self.memory.SYSC.write(SYSC::SOFTRESET::Reset);
        loop {
            if self.memory.SYSS.is_set(SYSS::RESETDONE) {
                break;
            }
            arm::__nop();
        }
    }
    pub fn disable_irq(&self) {
        self.memory.IER.set(0);
    }

    pub fn to_config_b(self) -> UartConfigB {
        let raw_ptr = self.memory as *const RegisterBlock;
        let saved_lcr = self.memory.LCR.get();
        self.memory.LCR.set(0x00BF);
        // do stuff
        unsafe { UartConfigB::new(VirtualAddress::from_ptr(raw_ptr), saved_lcr) }
    }
    pub fn disable(&self) {
        self.memory.MDR1.write(MDR1::MODESELECT::Disable);
    }
    pub fn enable(&self) {
        self.memory.MDR1.write(MDR1::MODESELECT::Uart16);
    }
    pub fn initialize(self) -> Self {
        self.disable();
        self.disable_irq();
        let config_b = self.to_config_b();
        config_b.enable_all_ier();
        config_b.set_baud();
        config_b.memory.MCR.write(MCR::DTR::Low + MCR::RTS::Low);
        config_b.memory.LCR.write(LCR::CHAR_LENGTH::BIT8);
        config_b.memory.MDR1.write(MDR1::MODESELECT::Uart16);
        let uart = config_b.to_operating_mode();
        uart.memory.IIR.set(0); // Writes FCR
        uart.disable_irq();
        //uart.enable();
        uart
    }
    pub fn debug_lcr(&self) -> u32 {
        self.memory.LCR.get()
    }
    pub fn debug_mdr1(&self) -> u32 {
        self.memory.MDR1.get()
    }
    pub fn debug_mdr2(&self) -> u32 {
        self.memory.MDR2.get()
    }
    pub fn debug_lsr(&self) -> u32 {
        self.memory.LSR.get()
    }
    pub fn dump_registers<T: fmt::Write>(&self, serial: &mut T) -> fmt::Result {
        writeln!(serial, "Registers:\nLCR: {:#x}\nMDR1: {:#x}\nMDR2: {:#x}\nLSR: {:#x}\nMCR: {:#x}\nIIR: {:#x}\nSCR: {:#x}",
                 self.memory.LCR.get(),
                 self.memory.MDR1.get(),
                 self.memory.MDR2.get(),
                 self.memory.LSR.get(),
                 self.memory.MCR.get(),
                 self.memory.IIR.get(),
                 self.memory.SCR.get(),
                 )
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
