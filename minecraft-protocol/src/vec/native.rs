macro_rules! native_be {
    ($($w:ident $p:ident $t:ident $n:expr)+) => {
        pub trait WriteMCNativeTypes {
            native_be_def!($($w $p $t $n)+);
        }
        impl WriteMCNativeTypes for Vec<u8> {
            native_be_impl!($($w $p $t $n)+);
        }
    };
}

macro_rules! native_be_def {
    ($($w:ident $p:ident $t:ident $n:expr)+) => { $(
        fn $w(&mut self, var: $t) -> &mut Self;
        fn $p(&mut self, var: $t) -> &mut Self;
    )+ };
}

macro_rules! native_be_impl {
    ($($w:ident $p:ident $t:ident $n:expr)+) => { $(
        fn $w(&mut self, var: $t) -> &mut Self {self.extend(var.to_be_bytes());self}
        fn $p(&mut self, var: $t) -> &mut Self {
            let len = self.len();
            self.resize($n + len, 0);
            self.copy_within(..len, $n);
            self[..$n].copy_from_slice(&var.to_be_bytes()[..]);
            self
        }
    )+ };
}

native_be!(
    write_u128be prefix_u128 u128 16
    write_u64be prefix_u64 u64 8
    write_u32be prefix_u32 u32 4
    write_u16be prefix_u16 u16 2
    write_u8be prefix_u8 u8 1
    write_i128be prefix_i128 i128 16
    write_i64be prefix_i64 i64 8
    write_i32be prefix_i32 i32 4
    write_i16be prefix_i16 i16 2
    write_i8be prefix_i8 i8 1
    write_f64be prefix_f64 f64 8
    write_f32be prefix_f32 f32 4
);
