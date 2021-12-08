// use std::io::Cursor;

// use minecraft_protocol::cursor::var_long::{ReadVarLong, WriteVarLong};

// #[allow(overflowing_literals)]
// #[rustfmt::skip]
// const SAMPLEDATA: [(i64, &'static [u8]); 11] = [
//     /*                    0 */ (0x0000000000000000, &[0x00]),
//     /*                    1 */ (0x0000000000000001, &[0x01]),
//     /*                    2 */ (0x0000000000000002, &[0x02]),
//     /*                  127 */ (0x000000000000007F, &[0x7f]),
//     /*                  128 */ (0x0000000000000080, &[0x80, 0x01]),
//     /*                  255 */ (0x00000000000000ff, &[0xff, 0x01]),
//     /*           2147483647 */ (0x000000007fffffff, &[0xff, 0xff, 0xff, 0xff, 0x07]),
//     /*  9223372036854775807 */ (0x7fffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]),
//     /*                   -1 */ (0xffffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]),
//     /*          -2147483648 */ (0xffffffff80000000, &[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01]),
//     /* -9223372036854775808 */ (0x8000000000000000, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
// ];

// #[test]
// fn test_read_wiki_vals() {
//     for (res, buf) in SAMPLEDATA {
//         assert_eq!(Cursor::new(&buf.to_vec()).read_var_long().unwrap(), res);
//     }
// }

// #[test]
// fn test_write_wiki_vals() {
//     for (val, res) in SAMPLEDATA {
//         let mut buf = vec![0; 10];
//         let mut cursor = Cursor::new(&mut buf);
//         cursor.write_var_long(val).unwrap();
//         let index = cursor.position();
//         assert_eq!(
//             &(cursor.get_ref().as_ref() as &[u8])[0..index as usize],
//             res
//         );
//     }
// }
