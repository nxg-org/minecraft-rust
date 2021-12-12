pub trait WriteMCVarLong {
    fn write_var_long(&mut self, var_long: i64) -> &mut Self;
    fn prefix_var_long(&mut self, var_long: i64) -> &mut Self;
}

impl WriteMCVarLong for Vec<u8> {
    fn write_var_long(&mut self, mut var_long: i64) -> &mut Self {
        for _ in 0..10 {
            self.push((var_long | 0x80) as u8);
            var_long = (var_long as u64 >> 7) as i64;
            if var_long == 0 {
                break;
            }
        }
        let len = self.len();
        self[len - 1] &= 0x7f;
        self
    }
    fn prefix_var_long(&mut self, var_long: i64) -> &mut Self {
        let mut tmp = vec![];
        tmp.write_var_long(var_long);
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

    use crate::cursor::prelude::ReadVarLong;

    use super::*;

    #[test]
    fn write_var_long() {
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("iteration {}", i);
            let x = rng.gen();
            let mut v = vec![];
            v.write_var_long(x);
            assert_eq!(Cursor::new(v).read_var_long().unwrap(), x);
        }
    }

    #[test]
    fn prefix_var_long() {
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            println!("iteration {}", i);
            let x = rng.gen();
            let mut v = vec![];
            v.write_var_long(x);
            assert_eq!(Cursor::new(v).read_var_long().unwrap(), x);
        }
    }
}
