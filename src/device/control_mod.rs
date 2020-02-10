//! Low level access to the control module functional group


use register::{mmio::*, register_bitfields, Field};
use armv7::VirtualAddress;

register_bitfields! {
    u32,
    CONF_MOD [
        MODE OFFSET(0) NUMBITS(3) [],
        PULL_EN OFFSET(3) NUMBITS(1) [Disable = 0b1, Enable = 0b0],
        PULL OFFSET(4) NUMBITS(1) [Up = 0b1, Down = 0b0],
        RX OFFSET(5) NUMBITS(1) [Enable = 1, Disable = 0],
        SLEW OFFSET(6) NUMBITS(1) [Fast = 0, Slow = 1]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
struct RegisterBlock {
    CONF_MOD: [ReadWrite<u32, CONF_MOD::Register>; CONF_NUM],
}

pub const CONF_NUM: usize = 141;

pub struct Control {
    memory: &'static RegisterBlock,
}

impl Control {
    pub unsafe fn new(memory_addr: VirtualAddress) -> Self {
        // We have to add 0x800 to the memory address
        let memory = &*((0x800 + memory_addr.as_u32()) as *mut RegisterBlock);
        Self { memory }
    }
    pub fn set(&self, index: usize, value: u32) {
        if index > CONF_NUM { return }
        self.memory.CONF_MOD[index].set(value);
    }
    pub fn get(&self, index: usize) -> Option<u32>{
        if index > CONF_NUM { return None }
        Some(self.memory.CONF_MOD[index].get())
    }
}
