#![feature(destructuring_assignment)]
use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use minecraft_data::FromVersion;
use minecraft_protocol::protodef::json;

pub type ProtoDefSpec = Tree<RawType>;

pub type FieldTree = indexmap::IndexMap<PathBuf, Field>;

pub struct Field {
    pub parsing_code: Box<dyn Fn(TreeContext<RawType>, PathBuf, String) -> proc_macro2::TokenTree>,
    pub writing_code: Box<dyn Fn(TreeContext<RawType>, PathBuf, String) -> proc_macro2::TokenTree>,
    pub code_type: proc_macro2::Ident,
}

pub struct TreeContext<'a, T>(&'a mut Tree<T>, PathBuf);

pub struct Tree<T> {
    pub store: HashMap<PathBuf, T>,
}

impl<Type: Debug> Tree<Namespace<Type>> {
    pub fn get_type<P: AsRef<Path>>(&self, name: &String, path: P) -> Option<(&Type, PathBuf)> {
        for p in path.as_ref().ancestors() {
            if let Some(ns) = self.store.get(p) {
                if let Some(t) = ns.types.get(name) {
                    return Some((t, p.to_path_buf()));
                }
            }
        }
        None
    }
}

#[derive(Default, Debug)]
pub struct Namespace<Type: Debug> {
    pub types: HashMap<String, Type>,
}

#[derive(Debug)]
pub struct RawField<Type: Debug> {
    pub name: Option<String>,
    pub r#type: Type,
}

impl From<serde_json::Value> for RawField<RawType> {
    fn from(v: serde_json::Value) -> Self {
        let obj = v.as_object().unwrap();
        Self {
            name: match obj.get("name") {
                Some(name) => match name.as_str() {
                    Some(name) => Some(name.to_owned()),
                    None => None,
                },
                None => None,
            },
            r#type: RawType::from(obj.get("type").unwrap().to_owned()),
        }
    }
}

trait NativeCodeGen: Fn(Namespace<RawType>) -> Field {}

impl TreeContext<'_, Namespace<RawType>> {
    
    fn _resolve_field(
        &mut self,
        name: String,
        _natives: HashMap<String, Box<dyn NativeCodeGen>>,
    ) -> Field {
        let raw_type = self.0.store.get(&self.1).unwrap().types.get(&name).unwrap();
        if let RawType::Container(fields) = raw_type {
            for field in fields {
                match &field.r#type {
                    RawType::Ref(type_name) => {
                        let (mut looked_up_type, mut looked_up_path) =
                            self.0.get_type(type_name, self.1.clone()).unwrap();
                        while let RawType::Ref(new_type_name) = looked_up_type {
                            if new_type_name == "native" {
                                break;
                            };
                            (looked_up_type, looked_up_path) =
                                self.0.get_type(type_name, looked_up_path).unwrap();
                        }

                        // while matches!(looked_up_type, RawType::Ref(b) if b != "native") {
                        //     (looked_up_type, looked_up_path) =
                        //         self.0.get_type(type_name, path).unwrap()
                        // }
                    }
                    RawType::Container(_) => todo!(),
                    RawType::Call(_, _) => todo!(),
                }
            }
        } else {
            panic!("tried to resolve a field which isn't container")
        };
        todo!()
    }
}

// fn get_native() {}

pub fn resolve_field(_cx: TreeContext<RawType>) {}

#[derive(Debug)]
pub enum RawType {
    Ref(String),
    Container(Vec<RawField<RawType>>),
    Call(Box<RawType>, HashMap<String, serde_json::Value>),
}

impl From<serde_json::Value> for RawType {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::String(s) => RawType::Ref(s),
            serde_json::Value::Array(arr) => match &arr[0] {
                serde_json::Value::String(s) if s == "container" && arr[1].is_array() => {
                    Self::Container(
                        arr[1]
                            .as_array()
                            .unwrap()
                            .to_owned()
                            .into_iter()
                            .map(|v| RawField::from(v))
                            .collect(),
                    )
                }
                v => RawType::Call(
                    Box::new(RawType::from(v.to_owned())),
                    arr[1]
                        .as_object()
                        .unwrap()
                        .to_owned()
                        .into_iter()
                        .collect::<HashMap<_, _>>(),
                ),
            },
            _ => panic!(),
        }
    }
}

fn main() {
    let _a = json::Namespace::from_version("1.12.2").unwrap();
    println!("{:#?}", _a);
    // let b = Namespace::default();
    // for (t_name, t_val) in &a.types {
    //     b.types.insert(t_name);
    // }

    let mut a = indexmap::IndexMap::new();
    a.insert("a", 1);
    a.insert("b", 2);
    a.insert("asdfasdfasdfasdf", 3);
    a.insert("-333", 4);
    println!("{:#?}", &a);
}
