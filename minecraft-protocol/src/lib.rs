#![feature(cursor_remaining)]

use bytes::BufMut;
pub mod cursor;
pub mod protocol_types;
// pub mod vec;

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

pub mod prelude {
    pub use super::WriteMCNativeTypes;
    pub use bytes;
    pub use tokio::{io,net};
}

// pub mod protocol {
//     pub enum PacketDirection {
//         ClientBound,
//         ServerBound,
//     }
//     impl PacketDirection {
//         pub fn opposite(&self) -> Self {
//             use PacketDirection::*;
//             match self {
//                 ClientBound => ServerBound,
//                 ServerBound => ClientBound,
//             }
//         }
//     }
//     pub enum State {
//         Handshaking,
//         Status,
//         Login,
//         Play,
//     }
//     impl State {
//         pub fn name(&self) -> &'static str {
//             use State::*;
//             match self {
//                 Handshaking => "Handshaking",
//                 Status => "Status",
//                 Login => "Login",
//                 Play => "Play",
//             }
//         }
//     }
//     pub struct Id {
//         pub id: i32,
//         pub state: State,
//         pub direction: PacketDirection,
//     }
//     //todo serialize

//     pub struct ProtocolPacketField {
//         pub name: String,
//         pub kind: String,
//     }
//     pub struct ProtocolPacketSpec {
//         pub state: String,
//         pub direction: String,
//         pub id: i32,
//         pub name: String,
//         pub body_struct: String,
//         pub fields: Vec<ProtocolPacketField>,
//     }
//     pub struct ProtocolSpec {
//         pub name: String,
//         pub packets: Vec<ProtocolPacketSpec>,
//     }
// }
