use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadInt {
    fn read_int(&mut self) -> Result<i32, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadInt for Cursor<T> {
    fn read_int(&mut self) -> Result<i32, BufferReadError> {
        let buf = self.remaining_slice();
        let val = i32::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 4);
        Ok(val)
    }
}

pub trait WriteInt {
    fn write_int(&mut self, integer: i32) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteInt for Cursor<T> {
    fn write_int(&mut self, integer: i32) -> Result<(), BufferWriteError> {
        let pos = self.position() as usize;
        let buf = &mut self.get_mut().as_mut()[pos..pos+4];
        buf.copy_from_slice(&integer.to_be_bytes());
        self.set_position(self.position() + 4);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_int() {
        let test_data = vec![0x0a, 0x0b, 0x0c, 0x0d];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_int().unwrap(), 0x0a0b0c0d);
    }

    #[test]
    fn write_int() {
        let mut test_data = [0; 4];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_int(0x0a0b0c0d).unwrap();
        assert_eq!(test_data, [0x0a, 0x0b, 0x0c, 0x0d])
    }
}
