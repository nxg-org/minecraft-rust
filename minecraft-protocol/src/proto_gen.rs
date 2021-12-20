// use serde::*;
// use serde_json::*;
// use std::collections::HashMap;

// mod json {
//     use minecraft_data::prelude::MINECRAFT_DATA_DIR;

//     use super::*;
//     #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
//     pub struct Protocol {
//         pub types: HashMap<String, Value>,
//         #[serde(flatten)]
//         pub states: HashMap<String, ProtocolState>,
//     }
//     #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
//     pub struct ProtocolState {
//         pub to_client: DirectionPacketTypes,
//         pub to_server: DirectionPacketTypes,
//     }
//     #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
//     pub struct DirectionPacketTypes {
//         pub types: HashMap<String, Value>,
//     }

//     impl minecraft_data::FromMCDataVersionDir for Protocol
//     where
//         Self: Sized,
//     {
//         fn from_version_paths(paths: &HashMap<String, String>) -> Option<Self> {
//             let mut path = std::path::PathBuf::from(paths.get("protocol").unwrap());
//             path.push("protocol.json");
//             Some(
//                 serde_json::from_str(
//                     MINECRAFT_DATA_DIR
//                         .get_file(path)
//                         .unwrap()
//                         .contents_utf8()
//                         .unwrap(),
//                 )
//                 .unwrap(),
//             )
//         }
//     }
// }

// #[derive(Clone, PartialEq, Debug, Default)]
// pub enum Encoding {
//     #[default]
//     Utf8,
//     Utf16,
//     Ascii,
//     Latin,
//     __UNFINISHEDLIST,
// }

// pub type ContainerFieldIdent<'a> = &'a str;

// #[derive(Clone, PartialEq, Debug)]
// pub struct BitfieldSection<'a> {
//     pub name: ContainerFieldIdent<'a>,
//     pub size: usize,
//     pub signed: bool,
// }

// #[derive(Clone, PartialEq, Debug)]
// pub enum Type {
//     Native(String),
//     Container(Vec<ContainerField<'a>>),
//     Mapper {
//         r#type: Box<Type>,
//         mappings: HashMap<usize, serde_json::Value>,
//     },
//     Switch {
//         compare_to: ContainerFieldIdent<'a>,
//         fields: HashMap<usize, Type>,
//         default: Option<Box<Type>>,
//     },
//     Buffer {
//         count_type: Box<Type>,
//     },
//     FieldBuffer {
//         count_field: ContainerFieldIdent<'a>,
//     },
//     FixedBuffer {
//         count: usize,
//     },
//     RestBuffer,
//     PrefixedString {
//         count_type: Box<Type>,
//         encoding: Encoding,
//     },
//     FieldString {
//         count_field: ContainerFieldIdent<'a>,
//         encoding: Encoding,
//     },
//     FixedString {
//         count: usize,
//         encoding: Encoding,
//     },
//     Bitfield(Vec<BitfieldSection<'a>>),
//     Array {
//         r#type: Box<Type>,
//         count_type: Box<Type>,
//     },
//     FieldArray {
//         r#type: Box<Type>,
//         count_field: ContainerFieldIdent<'a>,
//     },
//     FixedArray {
//         r#type: Box<Type>,
//         count: usize,
//     },
//     Count {
//         r#type: Box<Type>,
//         count_for: ContainerFieldIdent<'a>,
//     },
//     Option(Box<Type>),
//     EntityMetadataLoop {
//         end_val: usize,
//         r#type: Box<Type>,
//     },
// }

// #[derive(Clone, PartialEq, Debug)]
// pub struct ContainerField<'a> {
//     pub name: Option<String>,
//     pub r#type: Type,
// }

// impl<'a> Type {
//     pub fn from_json_value(val: Value, lookup_table: &TypeTable) -> Self {
//         match val {
//             Value::String(_) => todo!(),
//             Value::Array(_) => todo!(),
//             _ => panic!("badly formatted protocol json"),
//         }
//     }
// }

// #[derive(Default)]
// pub struct TypeTable<'a> {
//     pub native: Vec<String>,
//     pub aliases: HashMap<String, Type>,
// }

// #[test]
// fn test_proto_gen() {
//     use minecraft_data::prelude::*;
//     let versions: &[&str] = &["1.12.2", "1.17.1", "1.8.9"];
//     for version in versions {
//         let prot_json = json::Protocol::from_version(version).unwrap();
//         let mut type_table = TypeTable::default();
//         for (name, val) in prot_json.types {
//             match val {
//                 Value::String(s) if s == "native" => {
//                     type_table.native.push(name.clone());
//                 }
//                 Value::Array(_) => {
//                     type_table
//                         .aliases
//                         .insert(name, Type::from_json_value(val, &type_table));
//                 }
//                 _ => panic!("badly formatted protocol json"),
//             }
//         }
//         for (state_name, state) in prot_json.states {
//             for (packet_name, data_type) in state.to_client.types {}
//         }
//     }
// }

// //todo everything below this is bullshit

// pub struct PacketDisconnect {
//     reason: String,
// }

// pub struct PacketEncryptionBegin<'a> {
//     server_id: String,
//     public_key: &'a [u8],
//     verify_token: &'a [u8],
// }

// // #[derive(ToFields)]
// // pub struct Position {
// //     x: u64,
// //     y: u64,
// //     z: u64,
// // }

// // impl ToFields for Position {
// //     fn add_to_hashmap(&self, hm: HashMap<>){
// //         hm.insert("x", self.x);
// //         hm.insert("y", self.y);
// //         hm.insert("z", self.z);
// //     }
// // }

// // {
// //     x: Value,
// //     y: Value,
// //     z: Value,
// // }

// pub struct PositionPacket {
//     x: f32,
//     y: f32,
//     z: f32,
// }

// /*
//  * namespace: {
//  *   types?: {[type_alias]: type}
//  *   [namespace_name]: namespace
//  * }
//  *
//  * type: "native" | alias | impl
//  */
// // pub struct

// pub struct ProtocolStepFive<'a> {
//     to_client: Type,
//     to_server: Type,
// }

// //let packet = mcp::Position{
// //     x
// // }

// // fn teststtststs() {
// //     let ns: Namespace = todo!();
// //     let states = &ns.sub;

// //     for state in states {
// //         let state_client_packets = state.1.sub.get("toClient").unwrap();
// //     }
// // }

// // ! DA PLAN
// // !
// // ! 1. parse json into struct (json::Protocol)
// // ! 2. iterate Protocol struct into Namespace tree
// // ! 3. get the packet types (to_client and to_server) as the two only impls
// // ! 4. write parsing code
// // ! 5. todo!()

// pub struct Namespace<'a> {
//     pub parent: Option<Box<Namespace<'a>>>,
//     pub sub: HashMap<String, Namespace<'a>>,
//     pub types: HashMap<String, Type>,
// }

// /**
//  *
//  * 1. json => json::Protocol
//  * 2.
//  * 3.
//  * 4.
//  *
//  *
//  */
// fn tset() {}

// pub fn not_todo() {
//     todo!()
// }

// /**
//  * Steps to accomplish above:
//  * ==============================
//  * 1. Serialize all packets from minecraft-protocol into usable, version-specific structs (partially, mostly done?) (contains json, not done.)
//  *  1a. Right now, json is stuck using serde_json::Value. This requires a currently unknown fix.
//  * 2.
//  * 2. Have a top level "manager" for each type of packet, hard-coded. References the specified version's struct data.
//  *  2a. This is done via &str specification rn.
//  *  2b. Inside manager, reference
//  */
// fn trestssss() {}

// // impl<'a> Type {
// //     pub fn from_json_val(val: &'a serde_json::Value) -> Self {
// //         match val {
// //             Value::String(s) => {Self::Ref(&s)},
// //             Value::Array(v) => {
// //                 let (tag, content) = (v[0].as_str().unwrap(), &v[1]);
// //                 match tag {
// //                     "container" => {
// //                         let arr = content.as_array().unwrap();
// //                         let mut a = Vec::new();
// //                         for elem in arr {
// //                             let obj = elem.as_object().unwrap();
// //                             a.push(ContainerField{
// //                                 name: if let Some(name) = obj.get("name") {Some(name.as_str().unwrap().into())} else {None},
// //                                 r#type: Type::from_json_val(obj.get("type").unwrap()),
// //                             })
// //                         }
// //                         Type::Container(a)
// //                     },
// //                     "mapper" => {
// //                         let obj = content.as_object().unwrap();
// //                         let mut mappings = HashMap::new();
// //                         obj.get("mappings").unwrap().as_object().unwrap().into_iter().for_each(|(ident, val)|{
// //                             mappings.insert(ident.clone(), Type::from_json_val(val));
// //                         });
// //                         Type::Mapper{
// //                             r#type: obj.get("type").unwrap().as_str().unwrap(),
// //                             mappings
// //                         }
// //                     },
// //                     "switch" => {
// //                         let obj = content.as_object().unwrap();
// //                         let mut fields = HashMap::new();
// //                         for (switchval, r#type) in obj.get("fields").unwrap().as_object().unwrap().into_iter() {
// //                             fields.insert(switchval.clone(), Type::from_json_val(r#type));
// //                         };
// //                         Type::Switch{
// //                             compare_to: obj.get("compareTo").unwrap().as_str().unwrap(),
// //                             fields,
// //                             default: if let Some(val) = obj.get("default") {Some(Box::new(Type::from_json_val(val))) } else {None}
// //                         }
// //                     },
// //                     "buffer" => {
// //                         let obj = content.as_object().unwrap();
// //                         if let Some(a) = obj.get("countType") {
// //                             Type::Buffer{
// //                                 count_type: Box::new(Type::from_json_val(a))
// //                             }
// //                         } else if let Some(a) = obj.get("count") {
// //                             let s = a.as_str().unwrap();
// //                             match from_str::<usize>(s) {
// //                                 Ok(count) => Type::FixedBuffer{count},
// //                                 _ => Type::FieldBuffer{count_field: s},
// //                             }
// //                         } else {
// //                             Type::RestBuffer
// //                         }
// //                     },
// //                     "pstring" => {
// //                         let obj = content.as_object().unwrap();
// //                         let encoding = if let Some(encoding) = obj.get("encoding") {
// //                             match &encoding.as_str().unwrap().to_ascii_lowercase()[..] {
// //                                 "utf-8" | "utf8" => Encoding::Utf8,
// //                                 "utf-16" | "utf16" => Encoding::Utf16,
// //                                 "ascii" => Encoding::Ascii,
// //                                 "latin" => Encoding::Latin,
// //                                 _ => panic!("encoding not supported"),
// //                             }
// //                         } else {Encoding::Utf8};
// //                         if let Some(a) = obj.get("countType") {
// //                             Type::PrefixedString{
// //                                 count_type: Box::new(Type::from_json_val(a)),
// //                                 encoding
// //                             }
// //                         } else if let Some(a) = obj.get("count") {
// //                             let s = a.as_str().unwrap();
// //                             match from_str::<usize>(s) {
// //                                 Ok(count) => Type::FixedString{count,
// //                                     encoding},
// //                                 _ => Type::FieldString{count_field: s,
// //                                     encoding},
// //                             }
// //                         } else {
// //                             panic!("either provide count or countType")
// //                         }
// //                     },
// //                     "bitfield" => {
// //                         let arr = content.as_array().unwrap();
// //                         let mut ret = Vec::new();
// //                         for elem in arr {
// //                             let obj = elem.as_object().unwrap();
// //                             ret.push(BitfieldValue{
// //                                 name: obj.get("name").unwrap().as_str().unwrap(),
// //                                 size: obj.get("size").unwrap().as_u64().unwrap() as u8,
// //                                 signed: obj.get("signed").unwrap().as_bool().unwrap(),
// //                             });
// //                         };
// //                         Type::Bitfield(ret)
// //                     },
// //                     "array" => {
// //                         let obj = content.as_object().unwrap();
// //                         let r#type = Box::new(Type::from_json_val(obj.get("type").unwrap()));

// //                         if let Some(a) = obj.get("countType") {
// //                             Type::Array{
// //                                 r#type,
// //                                 count_type: Box::new(Type::from_json_val(a))
// //                             }
// //                         } else if let Some(a) = obj.get("count") {
// //                             match a {
// //                                 Value::Number(n) => Type::FixedArray{
// //                                     r#type,count: n.as_u64().unwrap() as usize},
// //                                 Value::String(count_field) => Type::FieldArray{
// //                                     r#type,count_field},
// //                                 _=>panic!("stupid ass wrong type definitions strike again")
// //                             }
// //                         } else {
// //                             panic!("you have to provide one of count or countType")
// //                         }
// //                     },
// //                     "count" => {
// //                         let obj = content.as_object().unwrap();
// //                         Type::Count{
// //                             r#type: Box::new(Type::from_json_val(obj.get("type").unwrap())),
// //                             count_for: obj.get("countFor").unwrap().as_str().unwrap()
// //                         }
// //                     },
// //                     "option" => {
// //                         Type::Option(Box::new(Type::from_json_val(content)))
// //                     },
// //                     "entityMetadataLoop" => {
// //                         let obj = content.as_object().unwrap();
// //                         Type::EntityMetadataLoop{
// //                             r#type: Box::new(Type::from_json_val(obj.get("type").unwrap())),
// //                             end_val: obj.get("endVal").unwrap().as_u64().unwrap() as usize
// //                         }
// //                     },
// //                     "entityMetadataItem" => {
// //                         Type::Ref("entityMetadataItem")
// //                     },
// //                     _ => panic!("{:#?}, your protocol definitions are fucking retarded and not correct, so please gladly fuck off now immediately you stupid fuck i hope you die immediately you ...........................", v),
// //                 }
// //             },
// //             _=>panic!("fuck off you retarded bitch, learn how to make your protocol definitions correctly, then come back to get your head smashed you afpjldksfplsdkjfpaksbpfjasdhfgojsdnv;lkzjf...............")
// //         }
// //     }
// // }
