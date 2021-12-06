use std::io::Cursor;

use super::{BufferReadError, BufferWriteError};

pub trait ReadVarInt {
    fn read_var_int(&mut self) -> Result<i32, BufferReadError>;
}

impl<T: AsRef<[u8]>> ReadVarInt for Cursor<T> {
    fn read_var_int(&mut self) -> Result<i32, BufferReadError> {
        let buf = self.remaining_slice();
        let mut val = 0;
        for i in 0..5 {
            let cur_val = buf[i];
            val += ((cur_val & 0x7f) as u32) << (i * 7);
            if (cur_val & 0x80) == 0x00 {
                self.set_position(self.position() + i as u64);
                break;
            }
            if i == 4 {
                return Err(BufferReadError {
                    pos: self.position() + i as u64,
                });
            }
        }
        Ok(val as i32)
    }
}

pub trait WriteVarInt {
    fn write_var_int(&mut self, integer: i32) -> Result<(), BufferWriteError>;
}

impl<T: AsMut<[u8]>> WriteVarInt for Cursor<T> {
    fn write_var_int(&mut self, mut integer: i32) -> Result<(), BufferWriteError> {
        let mut pos = self.position() as usize;
        let buf = self.get_mut().as_mut();
        for _ in 0..5 {
            buf[pos] = (integer | 0x80) as u8;
            integer = (integer as u32 >> 7) as i32;
            pos += 1;
            if integer == 0 {
                break;
            }
        }
        buf[pos - 1] &= 0x7f;
        self.set_position(pos as u64);
        Ok(())
    }
}

pub(crate) mod test {
    #![allow(unused_imports)]
    use super::*;
    use rand::{Rng, RngCore};

    #[test]
    fn read_var_int() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("\niteration {}", i);
            let x = rng.gen::<i32>();

            let (buf, byte_index) = {
                let rand_offset = rng.gen::<u8>() as usize;
                let mut buf = vec![0; 1000];
                buf.fill_with(|| rng.gen::<u8>());
                let mut val = x;
                let mut i = 0;
                while val != 0 {
                    println!("{:x}", val);
                    buf[i + rand_offset] = (val | 0x80) as u8;
                    val = ((val as u32) >> 7) as i32;
                    i += 1;
                }
                println!("{:x}", val);
                buf[i - 1 + rand_offset] &= 0x7f;
                (buf, rand_offset)
            };
            let mut cursor = Cursor::new(&buf);
            cursor.set_position(byte_index as u64);
            let y = cursor.read_var_int().unwrap();
            println!("{:x} == {:x}", &y, &x);
            assert_eq!(y, x);
        }
    }

    #[test]
    fn fail_read_var_int() {
        let buf = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
        if Cursor::new(&buf).read_var_int().is_ok() {
            panic!("didn't panic when it should have panicked because of a too big value");
        }
    }

    #[test]
    fn write_var_int() {
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("iteration {}", i);
            let x = rng.gen::<i32>();
            let rand_offset = rng.gen::<u8>() as usize;
            let mut buf = vec![0; 1000];
            rng.fill_bytes(&mut buf);
            let mut cursor = Cursor::new(&mut buf);
            cursor.set_position(rand_offset as u64);
            cursor.write_var_int(x).unwrap();
            cursor.set_position(rand_offset as u64);
            assert_eq!(cursor.read_var_int().unwrap(), x);
        }
    }
}
