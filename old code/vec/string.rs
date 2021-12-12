use super::var_int::WriteMCVarInt;

pub trait WriteMCString {
    fn write_str(&mut self, string: &str) -> &mut Self;
    fn prefix_str(&mut self, string: &str) -> &mut Self;
}

impl WriteMCString for Vec<u8> {
    fn write_str(&mut self, string: &str) -> &mut Self {
        self.write_var_int(string.len() as i32);
        self.extend(string.as_bytes());
        self
    }
    fn prefix_str(&mut self, string: &str) -> &mut Self {
        let mut tmp = vec![];
        tmp.write_str(string);
        let len = self.len();
        let tmp_len = tmp.len();
        self.resize(tmp_len + len, 0);
        self.copy_within(..len, tmp_len);
        self[..tmp_len].copy_from_slice(&tmp[..]);
        self
    }
}
