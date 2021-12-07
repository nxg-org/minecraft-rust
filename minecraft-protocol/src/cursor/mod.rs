use std::fmt::Display;

pub mod boolean;
pub mod byte;
pub mod double;
pub mod float;
pub mod int;
pub mod long;
pub mod short;
pub mod unsigned_byte;
pub mod unsigned_short;
pub mod var_int;
pub mod var_long;
pub mod string;

#[derive(Debug)]
pub struct BufferReadError {
    pos: u64,
}

impl Display for BufferReadError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!(
            "Error reading from buffer at byte position 0x{:x}",
            self.pos
        );
        Ok(())
    }
}

impl std::error::Error for BufferReadError {}

#[derive(Debug)]
pub struct BufferWriteError {
    pos: u64,
}

impl Display for BufferWriteError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Error writing to buffer at byte position 0x{:x}", self.pos);
        Ok(())
    }
}

impl std::error::Error for BufferWriteError {}

pub mod prelude {
    pub use super::boolean::{ReadBoolean, WriteBoolean};
    pub use super::byte::{ReadByte, WriteByte};
    pub use super::double::{ReadDouble, WriteDouble};
    pub use super::float::{ReadFloat, WriteFloat};
    pub use super::int::{ReadInt, WriteInt};
    pub use super::long::{ReadLong, WriteLong};
    pub use super::short::{ReadShort, WriteShort};
    pub use super::unsigned_byte::{ReadUnsignedByte, WriteUnsignedByte};
    pub use super::unsigned_short::{ReadUnsignedShort, WriteUnsignedShort};
    pub use super::var_int::{ReadVarInt, WriteVarInt};
    pub use super::var_long::{ReadVarLong, WriteVarLong};
    pub use super::string::{ReadString, WriteString};
}
