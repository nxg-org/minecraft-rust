use super::prelude::WriteVarInt;
use super::var_int::ReadVarInt;
use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadString {
    fn read_string(&mut self) -> Result<String, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadString for Cursor<T> {
    fn read_string(&mut self) -> Result<String, BufferReadError> {
        let str_len = self.read_var_int()?;
        let buf = self.remaining_slice()[0..str_len as usize].to_vec();
        let string = String::from_utf8(buf).unwrap();
        self.set_position(self.position() + str_len as u64);
        Ok(string)
    }
}

pub trait WriteString {
    fn write_string<U: AsRef<str>>(&mut self, string: U) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteString for Cursor<T> {
    fn write_string<U: AsRef<str>>(&mut self, string: U) -> Result<(), BufferWriteError> {
        let bytes = string.as_ref().as_bytes();
        let len = bytes.len();
        self.write_var_int(len as i32)?;
        let pos = self.position();
        self.get_mut().as_mut()[pos as usize..pos as usize + len].copy_from_slice(bytes);
        self.set_position(pos + len as u64);
        Ok(())
    }
}
