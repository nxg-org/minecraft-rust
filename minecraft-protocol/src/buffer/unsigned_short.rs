use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadUnsignedShort {
    fn read_unsigned_short(&mut self) -> Result<u16, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadUnsignedShort for Cursor<T> {
    fn read_unsigned_short(&mut self) -> Result<u16, BufferReadError> {
        let buf = self.remaining_slice();
        let val = u16::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 2);
        Ok(val)
    }
}

pub trait WriteUnsignedShort {
    fn write_unsigned_short(&mut self, integer: u16) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteUnsignedShort for Cursor<T> {
    fn write_unsigned_short(&mut self, integer: u16) -> Result<(), BufferWriteError> {
        let pos = self.position() as usize;
        let buf = &mut self.get_mut().as_mut()[pos..pos + 2];
        buf.copy_from_slice(&integer.to_be_bytes());
        self.set_position(self.position() + 2);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_unsigned_short() {
        let test_data = vec![0xff, 0x0b];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_unsigned_short().unwrap(), 0xff0b);
    }

    #[test]
    fn write_unsigned_short() {
        let mut test_data = [0; 2];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_unsigned_short(0xff0b).unwrap();
        assert_eq!(test_data, [0xff, 0x0b])
    }
}
