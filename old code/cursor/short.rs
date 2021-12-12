use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadShort {
    fn read_short(&mut self) -> Result<i16, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadShort for Cursor<T> {
    fn read_short(&mut self) -> Result<i16, BufferReadError> {
        let buf = self.remaining_slice();
        let val = i16::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 2);
        Ok(val)
    }
}

pub trait WriteShort {
    fn write_short(&mut self, integer: i16) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteShort for Cursor<T> {
    fn write_short(&mut self, integer: i16) -> Result<(), BufferWriteError> {
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
    fn read_short() {
        let test_data = vec![0x0a, 0x0b];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_short().unwrap(), 0x0a0b);
    }

    #[test]
    fn write_short() {
        let mut test_data = [0; 2];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_short(0x0a0b).unwrap();
        assert_eq!(test_data, [0x0a, 0x0b])
    }
}
