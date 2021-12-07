use std::io::Cursor;

use minecraft_protocol::cursor::var_int::{ReadVarInt, WriteVarInt};

#[allow(overflowing_literals)]
#[rustfmt::skip]
const SAMPLEDATA: [(i32, &'static [u8]); 11] = [
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
fn test_read_wiki_vals() {
    for (res, buf) in SAMPLEDATA {
        assert_eq!(Cursor::new(&buf.to_vec()).read_var_int().unwrap(), res);
    }
}

#[test]
fn test_write_wiki_vals() {
    for (val, res) in SAMPLEDATA {
        let mut buf = vec![0; 5];
        let mut cursor = Cursor::new(&mut buf);
        cursor.write_var_int(val).unwrap();
        let index = cursor.position();
        assert_eq!(
            &(cursor.get_ref().as_ref() as &[u8])[0..index as usize],
            res
        );
    }
}
