//! The GPIO

// Author: Moritz Doll
// License: MIT

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
}
