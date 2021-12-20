#![feature(derive_default_enum, if_let_guard)]
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use minecraft_data::{prelude::SUPPORTED_VERSIONS, FromVersion};
use serde_json::*;

mod json {
    use minecraft_data::{prelude::MINECRAFT_DATA_DIR, FromMCDataVersionDir};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /**
     * ProtoDef Namespaces
     * https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/protocol.md#protocol
     * directly taken from the protocol.json files found in minecraft-data
     */
    #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
    pub struct Namespace {
        #[serde(default)]
        pub types: HashMap<String, serde_json::Value>,
        #[serde(flatten)]
        pub sub: HashMap<String, Namespace>,
    }

    impl FromMCDataVersionDir for Namespace
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

// Type aliases to distinguish certain kinds of identifying strings
pub type MapperCase = String;
pub type ContainerFieldIdent = String;

/**
 * Parts of the Container
 * anonymous ones don't have a name,
 * all have a type
 * ContainerFields are written one after the other
 */
#[derive(Clone, Debug, PartialEq)]
pub struct ContainerField {
    pub name: Option<String>,
    pub r#type: Type,
}

/**
 * A BitfieldSection reflects an integer of
 * a certain size of bits inside of a bitfield
 * Equivalent to Java's BitSet
 * https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html
 */
#[derive(Clone, PartialEq, Debug)]
pub struct BitfieldSection {
    pub name: ContainerFieldIdent,
    pub size: usize,
    pub signed: bool,
}

/**
 * More rust-y representation of the ProtoDef Namespaces
 * https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/protocol.md#protocol
 * Namespaces are collected under different paths inside of NamespaceStores
 */
#[derive(Clone, Debug, Default)]
pub struct Namespace {
    pub sub: Vec<String>,
    pub types: HashMap<String, Type>,
}

/**
 * Represents different kinds of String Encoding that are possible
 * __UNFINISHEDLIST exists because this enum is not exhaustive
 */
#[derive(Clone, PartialEq, Debug, Default)]
pub enum Encoding {
    #[default]
    Utf8,
    Utf16,
    Ascii,
    Latin,
    __UNFINISHEDLIST,
}

impl Namespace {
    fn get_type<T: AsRef<str>>(&self, name: T) -> Option<&Type> {
        self.types.get(name.as_ref())
    }
}

/**
 * The NamespaceStore represents a Protocol
 * it organizes the Namespaces by Paths
 */
#[derive(Default, Debug)]
pub struct NamespaceStore {
    pub store: HashMap<PathBuf, Namespace>,
}

/**
 * A TypeContext serves to look up Types in a Namespace
 * inside of a NamespaceStore identified by its Path
 */
#[derive(Debug)]
pub struct TypeContext<'a>(
    &'a mut NamespaceStore,
    pub PathBuf,
    HashMap<String, serde_json::Value>,
);

impl TypeContext<'_> {
    /**
     * serves to look up a type recursively down the Path
     * of the Namespace the type is in
     * does not throw an error if an invalid path is provided
     * as long as the type alias exists in the root Namespace
     */
    pub fn get_type<T: AsRef<str>>(&mut self, name: T) -> Option<Type> {
        for a in self.1.ancestors() {
            if let Some(ns) = self.0.get_namespace(a) {
                if let Some(t) = ns.get_type(&name) {
                    return Some(t.to_owned());
                }
            }
        }
        Type::new(
            &self.2.get(&name.as_ref().to_owned()).unwrap().clone(),
            self,
            Some(name),
        )
    }
    pub fn push<T: AsRef<str>>(&mut self, path_segment: T) {
        self.1.push(path_segment.as_ref());
    }
    pub fn parent(mut self) -> Option<Self> {
        self.1 = self.1.parent().unwrap().to_path_buf();
        Some(self)
    }
    pub fn add_types(&mut self) -> Option<()> {
        for (t_ident, t_val) in self.2.clone() {
            let t = Type::new(&t_val, self, Some(t_ident.to_owned())).unwrap();
            self.0
                .store
                .get_mut(&self.1.to_path_buf())
                .unwrap()
                .types
                .insert(t_ident, t);
        }
        Some(())
    }
}

impl From<json::Namespace> for NamespaceStore {
    fn from(ns: json::Namespace) -> Self {
        let mut ret = Self::default();
        ret.add_namespace(ns, PathBuf::from("/"));
        ret
    }
}

impl minecraft_data::FromMCDataVersionDir for NamespaceStore
where
    Self: Sized,
{
    fn from_version_paths(paths: &HashMap<String, String>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(json::Namespace::from_version_paths(paths).unwrap().into())
    }
}

impl NamespaceStore {
    pub fn add_namespace(&mut self, input: json::Namespace, path: PathBuf) {
        let ret = Namespace::default();
        self.store.insert(path.clone(), ret);
        TypeContext {
            0: self,
            1: path.clone(),
            2: input.types,
        }
        .add_types();
        for (sub_name, sub_ns) in input.sub {
            let mut sub_path = path.clone();
            sub_path.push(sub_name);
            self.add_namespace(sub_ns, sub_path);
        }
    }
    pub fn get_namespace<P: AsRef<Path>>(&self, path: P) -> Option<&Namespace> {
        self.store.get(path.as_ref())
    }
    pub fn get_type<P: AsRef<Path>>(&self, name: &String, path: P) -> Option<&Type> {
        for p in path.as_ref().ancestors() {
            if let Some(ns) = self.store.get(p) {
                if let Some(t) = ns.get_type(name) {
                    return Some(t);
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeRef {
    Type(Type),
    Ref(PathBuf, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Native(String),
    Container(Vec<ContainerField>),
    Mapper {
        r#type: Box<Type>,
        mappings: HashMap<usize, MapperCase>,
    },
    Switch {
        compare_to: ContainerFieldIdent,
        fields: HashMap<MapperCase, Type>,
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

impl Type {
    pub fn new<T: AsRef<str>>(
        json_val: &Value,
        cx: &mut TypeContext,
        name: Option<T>,
    ) -> Option<Self> {
        Some(match json_val {
            Value::String(s) => {
                match &s[..] {
                    "native" if let Some(name) = name => Type::Native(name.as_ref().to_owned()),
                    _ => {
                        if let Some(t) = cx.get_type(&s) {
                            t.to_owned()
                        } else {
                            return None
                        }
                    }
                }
            }
            Value::Array(tuple) => {
                let (tag, params) = (tuple[0].as_str().unwrap(), &tuple[1]);
                match tag {
                    "container" => {
                        let arr = params.as_array().unwrap();
                        let mut fields = Vec::new();
                        for elem in arr {
                            let obj = elem.as_object().unwrap();
                            fields.push(ContainerField {
                                name: if let Some(name) = obj.get("name") {
                                    Some(name.as_str().unwrap().into())
                                } else {
                                    None
                                },
                                r#type: Type::new::<T>(obj.get("type").unwrap(), cx, None)
                                    .unwrap(),
                            })
                        }
                        Type::Container(fields)
                    },
                    "mapper" => {
                        let obj = params.as_object().unwrap();
                        let mut mappings = HashMap::new();
                        for (ident, val) in obj.get("mappings").unwrap().as_object().unwrap() {
                            mappings.insert(if ident.starts_with("0x") {
                                usize::from_str_radix(&ident[2..], 16)
                            } else {
                                usize::from_str_radix(&ident, 10)
                            }.unwrap(), val.as_str().unwrap().to_owned());
                        }
                        Type::Mapper{
                            r#type: Box::new(Type::new::<T>(obj.get("type").unwrap(), cx, None).unwrap()),
                            mappings
                        }
                    },
                    "switch" => {
                        let obj = params.as_object().unwrap();
                        let mut fields = HashMap::new();
                        for (switch_ident, switch_type) in obj.get("fields").unwrap().as_object().unwrap() {
                            fields.insert(switch_ident.to_owned(), Type::new::<T>(switch_type, cx, None).unwrap());
                        };
                        Type::Switch{
                            compare_to: obj.get("compareTo").unwrap().as_str().unwrap().to_owned(),
                            fields,
                            default: if let Some(val) = obj.get("default") {
                                Some(Box::new(Type::new::<T>(val, cx, None).unwrap()))
                            } else {
                                None
                            }
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
                                count_type: Box::new(Type::new::<T>(a,cx, None).unwrap()),
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
                            panic!("you have to provide one of count or countType")
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
                        let r#type = Box::new(Type::new::<T>(obj.get("type").unwrap(), cx, None).unwrap());
                        if let Some(a) = obj.get("countType") {
                            Type::Array{
                                r#type,
                                count_type: Box::new(Type::new::<T>(a, cx, None).unwrap())
                            }
                        } else if let Some(a) = obj.get("count") {
                            match a {
                                Value::Number(n) => Type::FixedArray{
                                    r#type,count: n.as_u64().unwrap() as usize},
                                Value::String(count_field) => Type::FieldArray{
                                    r#type,count_field: count_field.to_owned()},
                                _=>{panic!("malformed protocol.json file")}
                            }
                        } else {
                            panic!("you have to provide one of count or countType")
                        }
                    },
                    "count" => {
                        let obj = params.as_object().unwrap();
                        Type::Count{
                            r#type: Box::new(Type::new::<T>(obj.get("type").unwrap(), cx, None).unwrap()),
                            count_for: obj.get("countFor").unwrap().as_str().unwrap().to_owned()
                        }
                    },
                    "option" => {
                        Type::Option(Box::new(Type::new::<T>(params, cx, None).unwrap()))
                    },
                    "entityMetadataLoop" => {
                        let obj = params.as_object().unwrap();
                        Type::EntityMetadataLoop{
                            r#type: Box::new(Type::new::<T>(obj.get("type").unwrap(), cx, None).unwrap()),
                            end_val: obj.get("endVal").unwrap().as_u64().unwrap() as usize
                        }
                    },
                    s => {
                        cx.get_type(s).unwrap()
                    },
                }
            }
            _ => {
                panic!("malformed protocol.json file")
            },
        })
    }
    // pub fn get_fields(&self) -> HashMap<String, proc_macro2::TokenTree> {
    //     match self {
    //         Type::Native(s) => todo!(),
    //         Type::Container(_) => todo!(),
    //         Type::Mapper { r#type, mappings } => todo!(),
    //         Type::Switch { compare_to, fields, default } => todo!(),
    //         Type::Buffer { count_type } => todo!(),
    //         Type::FieldBuffer { count_field } => todo!(),
    //         Type::FixedBuffer { count } => todo!(),
    //         Type::PrefixedString { count_type, encoding } => todo!(),
    //         Type::FieldString { count_field, encoding } => todo!(),
    //         Type::FixedString { count, encoding } => todo!(),
    //         Type::Bitfield(_) => todo!(),
    //         Type::Array { r#type, count_type } => todo!(),
    //         Type::FieldArray { r#type, count_field } => todo!(),
    //         Type::FixedArray { r#type, count } => todo!(),
    //         Type::Count { r#type, count_for } => todo!(),
    //         Type::Option(_) => todo!(),
    //         Type::EntityMetadataLoop { end_val, r#type } => todo!(),
    //     }
        
    //     todo!()
    // }
}

/**
 * Serves to define how a native Type is handled
 */
pub struct NativeType {}

lazy_static::lazy_static! {
    pub static ref NATIVE_TYPE_MAP: HashMap<String, &'static NativeType> = {
        let ret = HashMap::default();
        ret
    };
}

#[derive(Debug, Default)]
pub struct ProtocolStore {
    pub ns_vec: Vec<NamespaceStore>,
}

#[derive(Clone)]
pub struct UniversalField {
    variations: Vec<Type>,
    optional: bool,
}

impl From<Type> for UniversalField {
    fn from(t: Type) -> Self {
        Self {
            variations: vec![t],
            optional: false,
        }
    }
}

impl UniversalField {
    pub fn add_version_specific(&mut self, r#type: Option<&Type>) {
        match r#type {
            Some(t) => self.variations.push(t.to_owned()),
            None => self.optional = true,
        }
    }
}

#[derive(Clone)]
pub struct UniversalFields(usize, HashMap<String, UniversalField>);

impl UniversalFields {
    pub fn add_version_fields(&mut self, fields: HashMap<String, Type>) {
        if self.0 == 0 {
            self.1 = fields
                .into_iter()
                .map(|(field_ident, field_type)| (field_ident, UniversalField::from(field_type)))
                .collect::<HashMap<_, _>>();
        } else {
            let mut existing = self
                .1
                .clone()
                .into_iter()
                .map(|(s, _)| s)
                .collect::<Vec<_>>();
            fields.iter().for_each(|(s, _)| {
                if !existing.contains(&s) {
                    existing.push(s.clone())
                }
            });
            for field_ident in existing {
                if let Some(field) = self.1.get_mut(&field_ident) {
                    field.add_version_specific(fields.get(&field_ident));
                } else {
                    let mut field: UniversalField =
                        fields.get(&field_ident).unwrap().to_owned().into();
                    field.add_version_specific(None);
                    self.1.insert(field_ident, field);
                }
            }
        }
    }
}

fn main() {
    let nss_arr: Vec<_> = SUPPORTED_VERSIONS
        .iter()
        .map(|v| NamespaceStore::from_version(v).unwrap())
        .collect();
    println!("{:#?}", nss_arr);
}
