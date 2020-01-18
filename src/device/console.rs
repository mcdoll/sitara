//! Defines a standard interface for TextIO, for instance MiniUart, Uart, and Debug
pub trait Console {
    fn getc(&self) -> char;
    //fn flush(&mut self);
}

