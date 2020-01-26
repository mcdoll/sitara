//! Provides access to the memory-mapped devices of the Sitara.

// Author: Moritz Doll
// License: MIT

pub mod console;
pub mod gpio;
pub mod timer;
pub mod uart;
pub mod watchdog;

pub use console::*;
