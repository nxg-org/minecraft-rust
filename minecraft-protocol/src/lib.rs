#![feature(derive_default_enum)]

use bytes::{Buf, BufMut};
pub mod proto_gen;
pub mod protocol_types;

pub trait WriteMCNativeTypes {
    fn put_var_int(&mut self, var_int: i32);
    fn put_var_long(&mut self, var_long: i64);
    fn put_str(&mut self, string: &str);
}

impl<T: BufMut> WriteMCNativeTypes for T {
    fn put_var_int(&mut self, mut var_int: i32) {
        loop {
            let next_val = (var_int as u32 >> 7) as i32;
            if next_val == 0 {
                self.put_u8(var_int as u8);
                break;
            }
            self.put_u8(var_int as u8 | 0x80);
            var_int = next_val;
        }
    }

    fn put_var_long(&mut self, mut var_long: i64) {
        loop {
            let next_val = (var_long as u64 >> 7) as i64;
            if next_val == 0 {
                self.put_u8(var_long as u8);
                break;
            }
            self.put_u8(var_long as u8 | 0x80);
            var_long = next_val;
        }
    }

    fn put_str(&mut self, string: &str) {
        self.put_var_int(string.len() as i32);
        self.put_slice(string.as_bytes());
    }
}

pub trait ReadMCNativeTypes {
    fn get_var_int(&mut self) -> i32;
    fn get_var_long(&mut self) -> i64;
    fn get_str(&mut self) -> String;
}

impl<T: Buf> ReadMCNativeTypes for T {
    fn get_var_int(&mut self) -> i32 {
        let mut val = 0;
        for i in 0..5 {
            let cur_val = self.get_u8();
            val += ((cur_val & 0x7f) as u32) << (i * 7);
            if (cur_val & 0x80) == 0x00 {
                break;
            }
        }
        val as i32
    }

    fn get_var_long(&mut self) -> i64 {
        let mut val = 0;
        for i in 0..10 {
            let cur_val = self.get_u8();
            val += ((cur_val & 0x7f) as u64) << (i * 7);
            if (cur_val & 0x80) == 0x00 {
                break;
            }
        }
        val as i64
    }

    fn get_str(&mut self) -> String {
        let len = self.get_var_int();
        String::from_utf8(self.copy_to_bytes(len as usize).to_vec()).unwrap()
    }
}

pub mod prelude {
    pub use super::WriteMCNativeTypes;
    pub use bytes;
    pub use tokio::{io, net};
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::*;

    #[allow(overflowing_literals)]
    const VAR_INT_SAMPLEDATA: [(i32, &'static [u8]); 11] = [
        /*           0 */ (0x00000000, &[0x00]),
        /*           1 */ (0x00000001, &[0x01]),
        /*           2 */ (0x00000002, &[0x02]),
        /*         127 */ (0x0000007F, &[0x7f]),
        /*         128 */ (0x00000080, &[0x80, 0x01]),
        /*         255 */ (0x000000ff, &[0xff, 0x01]),
        /*       25565 */ (0x000063dd, &[0xdd, 0xc7, 0x01]),
        /*     2097151 */ (0x001fffff, &[0xff, 0xff, 0x7f]),
        /*  2147483647 */ (0x7fffffff, &[0xff, 0xff, 0xff, 0xff, 0x07]),
        /*          -1 */ (0xffffffff, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
        /* -2147483648 */ (0x80000000, &[0x80, 0x80, 0x80, 0x80, 0x08]),
    ];

    #[test]
    fn read_var_int() {
        for (res, buf) in VAR_INT_SAMPLEDATA {
            assert_eq!(Bytes::from(buf).get_var_int(), res);
        }
    }

    #[test]
    fn write_var_int() {
        for (num, res) in VAR_INT_SAMPLEDATA {
            let mut buf = BytesMut::new();
            buf.put_var_int(num);
            assert_eq!(&buf[..], res);
        }
    }

    #[allow(overflowing_literals)]
    #[rustfmt::skip]
    const VAR_LONG_SAMPLEDATA: [(i64, &'static [u8]); 11] = [
        /*                    0 */ (0x0000000000000000, &[0x00]),
        /*                    1 */ (0x0000000000000001, &[0x01]),
        /*                    2 */ (0x0000000000000002, &[0x02]),
        /*                  127 */ (0x000000000000007F, &[0x7f]),
        /*                  128 */ (0x0000000000000080, &[0x80, 0x01]),
        /*                  255 */ (0x00000000000000ff, &[0xff, 0x01]),
        /*           2147483647 */ (0x000000007fffffff, &[0xff, 0xff, 0xff, 0xff, 0x07]),
        /*  9223372036854775807 */ (0x7fffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]),
        /*                   -1 */ (0xffffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]),
        /*          -2147483648 */ (0xffffffff80000000, &[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]),
        /* -9223372036854775808 */ (0x8000000000000000, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
    ];

    #[test]
    fn read_var_long() {
        for (res, buf) in VAR_LONG_SAMPLEDATA {
            assert_eq!(Bytes::from(buf).get_var_long(), res);
        }
    }

    #[test]
    fn write_var_long() {
        for (num, res) in VAR_LONG_SAMPLEDATA {
            let mut buf = BytesMut::new();
            buf.put_var_long(num);
            assert_eq!(&buf[..], res);
        }
    }

    const STRING_SAMPLEDATA: [(&'static str, &'static [u8]); 3] = [
        ("welcome to the test", b"\x13welcome to the test"),
        ("this has to be a little longer string so that we can check if it works when the var_int length changes... writing tests is really fun, isn't it?", b"\x90\x01this has to be a little longer string so that we can check if it works when the var_int length changes... writing tests is really fun, isn't it?"),
        ("some weird utf8 characters: äöü {} ßœæé€²¼⅐™", b"\x33some weird utf8 characters: \xE4\xF6\xFC {} \xDF\xC5\x93\xE6\xE9\xE2\x82\xAC\xB2\xBC\xE2\x85\x90\xE2\x84\xA2")
    ];

    #[test]
    fn read_string() {
        for (res, buf) in STRING_SAMPLEDATA {
            assert_eq!(Bytes::from(buf).get_str(), res);
        }
    }

    #[test]
    fn write_string() {
        for (str, res) in STRING_SAMPLEDATA {
            let mut buf = BytesMut::new();
            buf.put_str(str);
            assert_eq!(&buf[..], res);
        }
    }
}
