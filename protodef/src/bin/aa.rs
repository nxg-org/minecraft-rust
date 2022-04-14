// pub mod enum_like {
//     pub mod namespace {
//         pub mod to_client {
//             pub enum Packet {
//                 v1_12_2(super::super::super::versions::v1_12_2::namespace::to_client::Packet),
//                 v1_18_1(super::super::super::versions::v1_18_1::namespace::to_client::Packet),
//             }
//             impl Packet {
//                 // fn parse<T>
//             }
//         }
//     }
// }
// pub mod merged {
//     pub mod namespace {
//         pub mod to_client {
//             pub struct Packet {
//                 pub field: u32,
//                 pub name: Option<String>,
//             }
//         }
//     }
// }
// pub mod versions {
//     pub struct V1_12_2;
//     pub mod v1_12_2 {
//         pub mod namespace {
//             pub mod to_client {
//                 pub struct Packet{
//                     pub field: u8,
//                     pub name: String,
//                 }
//                 impl Packet {
//                     fn parse(buf: &mut impl Read) -> Self {
//                         let field = {todo!("parsing code")};
//                         let name = {todo!("parsing code")};
//                         Packet {
//                             field,
//                             name
//                         }
//                     }
//                 }
//                 type Packet_field = u8;
//             }
//         }
//     }
//     pub struct V1_18_1;
//     pub mod v1_18_1 {
//         pub mod namespace {
//             pub mod to_client {
//                 pub struct Packet {
//                     pub field: u32,
//                 }
//                 impl Packet {
//                     fn parse(buf: &mut impl Read) -> Self {
//                         let field = {todo!("parsing code")};
//                         Packet {
//                             field
//                         }
//                     }
//                 }
//                 type Packet_field = u32;
//             }
//         }
//     }
// }
fn main(){}