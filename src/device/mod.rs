//! Provides access to the memory-mapped devices of the Sitara.

// Author: Moritz Doll
// License: MIT

pub mod uart;
pub mod console;
pub mod timer;
pub mod watchdog;

pub use console::*;
