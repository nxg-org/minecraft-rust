use std::io::Cursor;

use super::{BufferReadError, BufferWriteError};

pub trait ReadBoolean {
    fn read_bool(&mut self) -> Result<bool, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadBoolean for Cursor<T> {
    fn read_bool(&mut self) -> Result<bool, BufferReadError> {
        let val = match self.remaining_slice()[0] {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(BufferReadError {
                pos: self.position(),
            }),
        };
        self.set_position(self.position() + 1);
        val
    }
}

pub trait WriteBoolean {
    fn write_bool(&mut self, boolean: bool) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteBoolean for Cursor<T> {
    fn write_bool(&mut self, boolean: bool) -> Result<(), BufferWriteError> {
        let pos = self.position();
        let buf = self.get_mut().as_mut();
        buf[pos as usize] = match boolean {
            true => 0x01,
            false => 0x00,
        };
        self.set_position(pos + 1);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn write_bool() {
        let data = &mut [0; 2];
        let mut cursor = Cursor::new(data);
        cursor.write_bool(true).unwrap();
        cursor.write_bool(false).unwrap();
        assert_eq!(cursor.get_ref().as_ref(), [1, 0]);
    }
}
