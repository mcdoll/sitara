//! The memory map of the Sitara as given by the manual
// Author: Moritz Doll
// License: MIT

use armv7::PhysicalAddress;

/// Start of the RAM
pub const DRAM_START: PhysicalAddress = PhysicalAddress::new(0x8000_0000);
/// End of the RAM
pub const DRAM_END: PhysicalAddress = PhysicalAddress::new(0x9FFF_FFFF);

/// List of special memory addresses (in 16MB, i.e. addresses 0xab**_****)
pub const DEVICES: [u8; 4] = [0x44, 0x47, 0x48, 0x4A];

/// Uart
pub const UART0: PhysicalAddress = PhysicalAddress::new(0x44E0_9000);
pub const UART1: PhysicalAddress = PhysicalAddress::new(0x4802_2000);
pub const UART2: PhysicalAddress = PhysicalAddress::new(0x4802_4000);
pub const UART3: PhysicalAddress = PhysicalAddress::new(0x481A_6000);
pub const UART4: PhysicalAddress = PhysicalAddress::new(0x481A_8000);
pub const UART5: PhysicalAddress = PhysicalAddress::new(0x481A_A000);

/// Interrupt controller
pub const IRQ_CONTROLLER: PhysicalAddress = PhysicalAddress::new(0x4820_0000);
/// Generic timer
pub const TIMER0: PhysicalAddress = PhysicalAddress::new(0x44E0_5000);
/// Watchdog
pub const WATCHDOG: PhysicalAddress = PhysicalAddress::new(0x44E3_5000);
/// GPIOs
pub const GPIO0: PhysicalAddress = PhysicalAddress::new(0x44E0_7000);
pub const GPIO1: PhysicalAddress = PhysicalAddress::new(0x4804_C000);
pub const GPIO2: PhysicalAddress = PhysicalAddress::new(0x481A_C000);
pub const GPIO3: PhysicalAddress = PhysicalAddress::new(0x481A_E000);
/// Control Module
pub const CONTROL: PhysicalAddress = PhysicalAddress::new(0x44E1_0000);
