//use armv7::VirtualAddress;
//use armv7::PhysicalAddress;
use crate::device::control_mod::*;
//use register::{mmio::*, register_bitfields, Field};


fn get_pin_addr(gpio_pin: u8) -> Option<u32> {
    // calculate the offset in the control module accoirding to P9Header
    // omitted allocated pins
    match gpio_pin {
        38 => Some(0x018),
        39 => Some(0x01C),
        34 => Some(0x008),
        35 => Some(0x00c),
        66 => Some(0x090),
        67 => Some(0x094),
        69 => Some(0x09c),
        68 => Some(0x098),
        45 => Some(0x034),
        44 => Some(0x030),
        23 => Some(0x024),
        26 => Some(0x028),
        47 => Some(0x03C),
        46 => Some(0x038),
        27 => Some(0x02C),
        65 => Some(0x08C),
        22 => Some(0x020),
        63 => Some(0x084),
        62 => Some(0x080),
        37 => Some(0x014),
        36 => Some(0x010),
        33 => Some(0x004),
        32 => Some(0x000),
        61 => Some(0x07C),
        86 => Some(0x0e0),
        88 => Some(0x0e8),
        87 => Some(0x0e4),
        89 => Some(0x0eC),
        10 => Some(0x0d8),
        11 => Some(0x0dc),
        9 => Some(0x0d4),
        81 => Some(0x0cc),
        8 => Some(0x0d0),
        80 => Some(0x0c8),
        78 => Some(0x0c0),
        79 => Some(0x0c4),
        76 => Some(0x0b8),
        77 => Some(0x0bc),
        74 => Some(0x0b0),
        75 => Some(0x0b4),
        72 => Some(0x0a8),
        73 => Some(0x0ac),
        70 => Some(0x0a0),
        71 => Some(0x0a4),

        30 => Some(0x070),
        60 => Some(0x078),
        31 => Some(0x074),
        50 => Some(0x048),
        48 => Some(0x040),
        51 => Some(0x04c),
        5 => Some(0x15c),
        4 => Some(0x158),
        3 => Some(0x154),
        2 => Some(0x150),
        49 => Some(0x044),
        15 => Some(0x184),
        14 => Some(0x180),
        115 => Some(0x1a4),
        20 => Some(0x1b4),
        116 => Some(0x1a8),
        7 => Some(0x164),
        _ => None,
    }
}

// Initialize the GPIO pins as pins
pub fn set_gpio_status(pin: u8, control_mod: &Control, state: u32) -> Option<()> {
    let index = get_pin_addr(pin)?;
    control_mod.set(index as usize, state);
    Some(())
}
pub fn get_gpio_status(pin: u8, control_mod: &Control) -> Option<u32> {
    let index = get_pin_addr(pin)?;
    control_mod.get(index as usize)
}
// Initialize the Uart
