use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadLong {
    fn read_long(&mut self) -> Result<i64, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadLong for Cursor<T> {
    fn read_long(&mut self) -> Result<i64, BufferReadError> {
        let buf = self.remaining_slice();
        let val = i64::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 8);
        Ok(val)
    }
}

pub trait WriteLong {
    fn write_long(&mut self, integer: i64) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteLong for Cursor<T> {
    fn write_long(&mut self, integer: i64) -> Result<(), BufferWriteError> {
        let pos = self.position() as usize;
        let buf = &mut self.get_mut().as_mut()[pos..pos + 8];
        buf.copy_from_slice(&integer.to_be_bytes());
        self.set_position(self.position() + 8);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_long() {
        let test_data = vec![0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_long().unwrap(), 0x0a0b0c0d0e0f1011);
    }

    #[test]
    fn write_long() {
        let mut test_data = [0; 8];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_long(0x0a0b0c0d0e0f1011).unwrap();
        assert_eq!(test_data, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11])
    }
}
