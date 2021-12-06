use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadFloat {
    fn read_float(&mut self) -> Result<f32, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadFloat for Cursor<T> {
    fn read_float(&mut self) -> Result<f32, BufferReadError> {
        let buf = self.remaining_slice();
        let val = f32::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 4);
        Ok(val)
    }
}

pub trait WriteFloat {
    fn write_float(&mut self, float: f32) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteFloat for Cursor<T> {
    fn write_float(&mut self, float: f32) -> Result<(), BufferWriteError> {
        let pos = self.position() as usize;
        let buf = &mut self.get_mut().as_mut()[pos..pos + 4];
        buf.copy_from_slice(&float.to_be_bytes());
        self.set_position(self.position() + 4);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_float() {
        let test_data = vec![0x40, 0x49, 0x0e, 0x56];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_float().unwrap(), 3.1415);
    }

    #[test]
    fn write_float() {
        let mut test_data = [0; 4];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_float(3.1415).unwrap();
        assert_eq!(test_data, [0x40, 0x49, 0x0e, 0x56])
    }
}
