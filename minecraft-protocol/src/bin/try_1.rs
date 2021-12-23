#![feature(derive_default_enum)]
use serde::*;
use serde_json::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod json {
    use minecraft_data::prelude::MINECRAFT_DATA_DIR;

    use super::*;

    #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
    pub struct Namespace {
        #[serde(default)]
        pub types: HashMap<String, Value>,
        #[serde(flatten)]
        pub sub: HashMap<String, Namespace>,
    }

    impl minecraft_data::FromMCDataVersionDir for Namespace
    where
        Self: Sized,
    {
        fn from_version_paths(paths: &HashMap<String, String>) -> Option<Self> {
            let mut path = std::path::PathBuf::from(paths.get("protocol").unwrap());
            path.push("protocol.json");
            Some(
                serde_json::from_str(
                    MINECRAFT_DATA_DIR
                        .get_file(path)
                        .unwrap()
                        .contents_utf8()
                        .unwrap(),
                )
                .unwrap(),
            )
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Encoding {
    #[default]
    Utf8,
    Utf16,
    Ascii,
    Latin,
    __UNFINISHEDLIST,
}

pub type ContainerFieldIdent = String;

#[derive(Clone, PartialEq, Debug)]
pub struct BitfieldSection {
    pub name: ContainerFieldIdent,
    pub size: usize,
    pub signed: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ContainerField {
    pub name: Option<String>,
    pub r#type: Type,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Native(String),
    Container(Vec<ContainerField>),
    Mapper {
        r#type: Box<Type>,
        mappings: HashMap<usize, String>,
    },
    Switch {
        compare_to: ContainerFieldIdent,
        fields: HashMap<String, Type>,
        default: Option<Box<Type>>,
    },
    Buffer {
        count_type: Box<Type>,
    },
    FieldBuffer {
        count_field: ContainerFieldIdent,
    },
    FixedBuffer {
        count: usize,
    },
    RestBuffer,
    PrefixedString {
        count_type: Box<Type>,
        encoding: Encoding,
    },
    FieldString {
        count_field: ContainerFieldIdent,
        encoding: Encoding,
    },
    FixedString {
        count: usize,
        encoding: Encoding,
    },
    Bitfield(Vec<BitfieldSection>),
    Array {
        r#type: Box<Type>,
        count_type: Box<Type>,
    },
    FieldArray {
        r#type: Box<Type>,
        count_field: ContainerFieldIdent,
    },
    FixedArray {
        r#type: Box<Type>,
        count: usize,
    },
    Count {
        r#type: Box<Type>,
        count_for: ContainerFieldIdent,
    },
    Option(Box<Type>),
    EntityMetadataLoop {
        end_val: usize,
        r#type: Box<Type>,
    },
}

impl<'a> Type {
    pub fn from_json(val: Value, cx: &mut Namespace<'a>) -> Option<Self> {
        Some(match val {
            Value::String(s) => return cx.lookup_type(s.clone()),
            Value::Array(v) => {
                let (tag, params) = (v[0].as_str().unwrap(), &v[1]);
                match tag {
                "container" => {
                    let arr = params.as_array().unwrap();
                    let mut a = Vec::new();
                    for elem in arr {
                        let obj = elem.as_object().unwrap();
                        a.push(ContainerField{
                            name: if let Some(name) = obj.get("name") {Some(name.as_str().unwrap().into())} else {None},
                            r#type: Type::from_json(obj.get("type").unwrap().to_owned(), cx).unwrap(),
                        })
                    }
                    Type::Container(a)
                },
                "mapper" => {
                    let obj = params.as_object().unwrap();
                    let mut mappings = HashMap::new();
                    obj.get("mappings").unwrap().as_object().unwrap().into_iter().for_each(|(ident, val)|{
                        println!("{}", &ident);
                        mappings.insert(if ident.starts_with("0x") {
                            usize::from_str_radix(&ident[2..], 16)
                        } else {
                            usize::from_str_radix(&ident, 10)
                        }.unwrap(), val.as_str().unwrap().to_owned());
                    });
                    Type::Mapper{
                        r#type: Box::new(Type::from_json(obj.get("type").unwrap().to_owned(), cx).unwrap()),
                        mappings
                    }
                },
                "switch" => {
                    let obj = params.as_object().unwrap();
                    let mut fields = HashMap::new();
                    for (switchval, r#type) in obj.get("fields").unwrap().as_object().unwrap().into_iter() {
                        fields.insert(switchval.clone(), Type::from_json(r#type.to_owned(), cx).unwrap());
                    };
                    Type::Switch{
                        compare_to: obj.get("compareTo").unwrap().as_str().unwrap().to_owned(),
                        fields,
                        default: if let Some(val) = obj.get("default") {Some(Box::new(Type::from_json(val.to_owned(), cx).unwrap())) } else {None}
                    }
                },
                "buffer" => {
                    let obj = params.as_object().unwrap();
                    if let Some(a) = obj.get("countType") {
                        Type::Buffer{
                            count_type: Box::new(Type::from_json(a.to_owned(), cx).unwrap())
                        }
                    } else if let Some(a) = obj.get("count") {
                        let s = a.as_str().unwrap();
                        match from_str::<usize>(s) {
                            Ok(count) => Type::FixedBuffer{count},
                            _ => Type::FieldBuffer{count_field: s.to_owned()},
                        }
                    } else {
                        Type::RestBuffer
                    }
                },
                "pstring" => {
                    let obj = params.as_object().unwrap();
                    let encoding = if let Some(encoding) = obj.get("encoding") {
                        match &encoding.as_str().unwrap().to_ascii_lowercase()[..] {
                            "utf-8" | "utf8" => Encoding::Utf8,
                            "utf-16" | "utf16" => Encoding::Utf16,
                            "ascii" => Encoding::Ascii,
                            "latin" => Encoding::Latin,
                            _ => panic!("encoding not supported"),
                        }
                    } else {Encoding::Utf8};
                    if let Some(a) = obj.get("countType") {
                        Type::PrefixedString{
                            count_type: Box::new(Type::from_json(a.to_owned(), cx).unwrap()),
                            encoding
                        }
                    } else if let Some(a) = obj.get("count") {
                        let s = a.as_str().unwrap();
                        match from_str::<usize>(s) {
                            Ok(count) => Type::FixedString{count,
                                encoding},
                            _ => Type::FieldString{count_field: s.to_owned(),
                                encoding},
                        }
                    } else {
                        panic!("either provide count or countType")
                    }
                },
                "bitfield" => {
                    let arr = params.as_array().unwrap();
                    let mut ret = Vec::new();
                    for elem in arr {
                        let obj = elem.as_object().unwrap();
                        ret.push(BitfieldSection{
                            name: obj.get("name").unwrap().as_str().unwrap().to_owned(),
                            size: obj.get("size").unwrap().as_u64().unwrap() as usize,
                            signed: obj.get("signed").unwrap().as_bool().unwrap(),
                        });
                    };
                    Type::Bitfield(ret)
                },
                "array" => {
                    let obj = params.as_object().unwrap();
                    let r#type = Box::new(Type::from_json(obj.get("type").unwrap().to_owned(), cx).unwrap());
                    if let Some(a) = obj.get("countType") {
                        Type::Array{
                            r#type,
                            count_type: Box::new(Type::from_json(a.to_owned(), cx).unwrap())
                        }
                    } else if let Some(a) = obj.get("count") {
                        match a {
                            Value::Number(n) => Type::FixedArray{
                                r#type,count: n.as_u64().unwrap() as usize},
                            Value::String(count_field) => Type::FieldArray{
                                r#type,count_field: count_field.to_owned()},
                            _=>panic!("[REDACTED]")
                        }
                    } else {
                        panic!("you have to provide one of count or countType")
                    }
                },
                "count" => {
                    let obj = params.as_object().unwrap();
                    Type::Count{
                        r#type: Box::new(Type::from_json(obj.get("type").unwrap().to_owned(), cx).unwrap()),
                        count_for: obj.get("countFor").unwrap().as_str().unwrap().to_owned()
                    }
                },
                "option" => {
                    Type::Option(Box::new(Type::from_json(params.to_owned(), cx).unwrap()))
                },
                "entityMetadataLoop" => {
                    let obj = params.as_object().unwrap();
                    Type::EntityMetadataLoop{
                        r#type: Box::new(Type::from_json(obj.get("type").unwrap().to_owned(), cx).unwrap()),
                        end_val: obj.get("endVal").unwrap().as_u64().unwrap() as usize
                    }
                },
                "entityMetadataItem" => {
                    // Type::Ref("entityMetadataItem")
                    todo!()
                },
                _ => panic!("{:#?}, your protocol definitions are fucking retarded and not correct, [REDACTED]", v),
            }
            }
            _ => {
                panic!("[REDACTED]")
            }
        })
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Namespace<'a> {
    pub json_val: Option<json::Namespace>,
    pub parent: Option<Rc<RefCell<Namespace<'a>>>>,
    pub sub: HashMap<String, Namespace<'a>>,
    pub types: HashMap<String, Type>,
}

impl<'a> Namespace<'a> {
    pub fn from_json(
        ns: json::Namespace,
        parent: Option<Rc<RefCell<Namespace<'a>>>>,
    ) -> Rc<RefCell<Namespace<'a>>> {
        let ret = Rc::new(RefCell::new(Self {
            json_val: Some(ns.clone()),
            parent,
            ..Default::default()
        }));
        for (sub_name, sub_ns) in ns.sub {
            ret.borrow_mut().sub.insert(
                sub_name,
                Namespace::from_json(sub_ns, Some(ret.clone()))
                    .borrow_mut()
                    .clone(),
            );
        }
        ret
    }
    pub fn lookup_type(&mut self, t_name: String) -> Option<Type> {
        if let Some(v) = self.types.get(&t_name) {
            Some(v.clone())
        } else if let Some(r#type) =
            // if let Some(val) = &self.json_val {
            // if let Some(v) = val.types.get(&t_name) {
            // Some(Type::from_json(v, self).unwrap())
            // } else {
            // None
            // }
            // } else {
            // None
            // }
            match self.json_val.to_owned() {
                Some(v) => match v.types.get(&t_name) {
                    Some(a) => Type::from_json(a.to_owned(), self),
                    None => None,
                },
                None => None,
            }
        {
            Some(r#type)
        } else {
            // println!("{:?}, {}", self, t_name);
            self.parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .lookup_type(t_name)
        }
    }
}

fn main() {
    use minecraft_data::prelude::*;
    let versions: &[&str] = &["1.12.2", "1.17.1", "0.30c"];
    for version in versions {
        let prot_json = json::Namespace::from_version(version).unwrap();
        let root = Namespace::from_json(prot_json, None);

        for (_, state_ns) in &mut root.borrow_mut().sub {
            println!(
                "{:#?}",
                state_ns
                    .sub
                    .get_mut("toClient")
                    .unwrap()
                    .lookup_type("packet".to_owned())
                    .unwrap()
            );
            println!(
                "{:#?}",
                state_ns
                    .sub
                    .get_mut("toServer")
                    .unwrap()
                    .lookup_type("packet".to_owned())
                    .unwrap()
            );
        }

        // println!(
        //     "{:#?}",
        //     // root.borrow_mut().lookup_type("packet".to_owned()).unwrap()
        // );
        // prot_json.sub.get("toClient").unwrap().sub.get("types").unwrap().
    }
}
