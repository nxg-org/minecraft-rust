use std::io::Cursor;

use super::{BufferReadError, BufferWriteError};

pub trait ReadUnsignedByte {
    fn read_unsigned_byte(&mut self) -> Result<u8, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadUnsignedByte for Cursor<T> {
    fn read_unsigned_byte(&mut self) -> Result<u8, BufferReadError> {
        let val = self.remaining_slice()[0];
        self.set_position(self.position() + 1);
        Ok(val)
    }
}

pub trait WriteUnsignedByte {
    fn write_unsigned_byte(&mut self, byte: u8) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteUnsignedByte for Cursor<T> {
    fn write_unsigned_byte(&mut self, byte: u8) -> Result<(), BufferWriteError> {
        let pos = self.position();
        let buf = self.get_mut().as_mut();
        buf[pos as usize] = byte;
        self.set_position(pos + 1);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_unsigned_byte() {
        let data = vec![0x00, 0xff];
        let mut cursor = Cursor::new(&data);
        assert_eq!(cursor.read_unsigned_byte().unwrap(), 0x00);
        assert_eq!(cursor.read_unsigned_byte().unwrap(), 0xff);
    }

    #[test]
    fn write_unsigned_byte() {
        let data = &mut [0; 2];
        let mut cursor = Cursor::new(data);
        cursor.write_unsigned_byte(0).unwrap();
        cursor.write_unsigned_byte(255).unwrap();
        assert_eq!(cursor.get_ref().as_ref(), [0x00, 0xff]);
    }
}
