use super::{BufferReadError, BufferWriteError};
use std::io::Cursor;

pub trait ReadDouble {
    fn read_double(&mut self) -> Result<f64, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadDouble for Cursor<T> {
    fn read_double(&mut self) -> Result<f64, BufferReadError> {
        let buf = self.remaining_slice();
        let val = f64::from_be_bytes(buf.try_into().unwrap());
        self.set_position(self.position() + 8);
        Ok(val)
    }
}

pub trait WriteDouble {
    fn write_double(&mut self, double: f64) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteDouble for Cursor<T> {
    fn write_double(&mut self, double: f64) -> Result<(), BufferWriteError> {
        let pos = self.position() as usize;
        let buf = &mut self.get_mut().as_mut()[pos..pos + 8];
        buf.copy_from_slice(&double.to_be_bytes());
        self.set_position(self.position() + 8);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn read_double() {
        let test_data = vec![0x40, 0x09, 0x21, 0xca, 0xc0, 0x83, 0x12, 0x6f];
        let mut cursor = Cursor::new(&test_data);
        assert_eq!(cursor.read_double().unwrap(), 3.1415);
    }

    #[test]
    fn write_double() {
        let mut test_data = [0; 8];
        let mut cursor = Cursor::new(&mut test_data);
        cursor.write_double(3.1415).unwrap();
        assert_eq!(test_data, [0x40, 0x09, 0x21, 0xca, 0xc0, 0x83, 0x12, 0x6f])
    }
}
