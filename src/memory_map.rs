//! The memory map of the Sitara as given by the manual
// Author: Moritz Doll
// License: MIT

use armv7::PhysicalAddress;

pub const DRAM_START: PhysicalAddress = PhysicalAddress::new(0x8000_0000);
pub const DRAM_END: PhysicalAddress = PhysicalAddress::new(0x9FFF_FFFF);

// Uart0
pub const UART_BASE: PhysicalAddress = PhysicalAddress::new(0x44E0_9000);
