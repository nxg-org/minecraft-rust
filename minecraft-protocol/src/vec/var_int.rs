pub trait WriteMCVarInt {
    fn write_var_int(&mut self, var_int: i32) -> &mut Self;
    fn prefix_var_int(&mut self, var_int: i32) -> &mut Self;
}

impl WriteMCVarInt for Vec<u8> {
    fn write_var_int(&mut self, mut var_int: i32) -> &mut Self {
        for _ in 0..5 {
            self.push((var_int | 0x80) as u8);
            var_int = (var_int as u32 >> 7) as i32;
            if var_int == 0 {
                break;
            }
        }
        let len = self.len();
        self[len - 1] &= 0x7f;
        self
    }
    fn prefix_var_int(&mut self, var_int: i32) -> &mut Self {
        let mut tmp = vec![];
        tmp.write_var_int(var_int);
        let len = self.len();
        let tmp_len = tmp.len();
        self.resize(tmp_len + len, 0);
        self.copy_within(..len, tmp_len);
        self[..tmp_len].copy_from_slice(&tmp[..]);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use rand::Rng;

    use crate::cursor::prelude::ReadVarInt;

    use super::*;

    #[test]
    fn write_var_int() {
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("iteration {}", i);
            let x = rng.gen();
            let mut v = vec![];
            v.write_var_int(x);
            assert_eq!(Cursor::new(v).read_var_int().unwrap(), x);
        }
    }
}
