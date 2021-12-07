use std::io::Cursor;

use super::{BufferReadError, BufferWriteError};

pub trait ReadByte {
    fn read_byte(&mut self) -> Result<i8, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadByte for Cursor<T> {
    fn read_byte(&mut self) -> Result<i8, BufferReadError> {
        let val = self.remaining_slice()[0] as i8;
        self.set_position(self.position() + 1);
        Ok(val)
    }
}

pub trait WriteByte {
    fn write_byte(&mut self, byte: i8) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteByte for Cursor<T> {
    fn write_byte(&mut self, byte: i8) -> Result<(), BufferWriteError> {
        let pos = self.position();
        let buf = self.get_mut().as_mut();
        buf[pos as usize] = byte as u8;
        self.set_position(pos + 1);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_byte() {
        let data = vec![0x00, 0x01];
        let mut cursor = Cursor::new(&data);
        assert_eq!(cursor.read_byte().unwrap(), 0x00);
        assert_eq!(cursor.read_byte().unwrap(), 0x01);
    }

    #[test]
    fn write_byte() {
        let data = &mut [0; 2];
        let mut cursor = Cursor::new(data);
        cursor.write_byte(-1).unwrap();
        cursor.write_byte(127).unwrap();
        assert_eq!(cursor.get_ref().as_ref(), [0xff, 0x7f]);
    }
}
