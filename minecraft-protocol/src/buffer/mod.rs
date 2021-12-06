use std::fmt::Display;

pub mod var_int;
pub mod var_long;
pub mod boolean;
pub mod byte;
pub mod unsigned_byte;
pub mod short;
pub mod unsigned_short;
pub mod int;
pub mod long;
pub mod float;
pub mod double;

#[derive(Debug)]
pub struct BufferReadError {
    pos: u64,
}

impl Display for BufferReadError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("Error reading buffer at byte position 0x{:x}", self.pos);
        Ok(())
    }
}
impl std::error::Error for BufferReadError {}

#[derive(Debug)]
pub struct BufferWriteError {
    position: u64,
}

impl Display for BufferWriteError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!(
            "Error writing to buffer at byte position 0x{:x}",
            self.position
        );
        Ok(())
    }
}
impl std::error::Error for BufferWriteError {}
