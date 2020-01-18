//! Low level access to the Sitara SoC by Texas Instruments
//!
//! This crate provide an interface to the memory-mapped devices of the SoC
//! and the memory map as layed out in the Sitara AM335x manual.
// Author: Moritz Doll
// License: MIT

#![no_std]

#![feature(stdsimd)]

pub mod device;
pub mod memory_map;
