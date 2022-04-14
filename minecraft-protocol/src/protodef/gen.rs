use std::{collections::HashMap, path::PathBuf};

use indexmap::IndexMap;
use proc_macro2::Span;

pub type CodeGenFn = Box<dyn Fn(String) -> proc_macro2::TokenStream>;

pub type TypeStore = HashMap<proc_macro2::Ident, Type>;

pub enum RustType {
    Struct(IndexMap<String, RustType>),
    Enum(Vec<RustType>),
    Option(Box<RustType>),
    Single(proc_macro2::Ident),
}

pub struct Type {
    pub parsing_code: CodeGenFn,
    pub writing_code: CodeGenFn,
    pub r#type: proc_macro2::Ident,
    // type_def: proc_macro2::TokenStream,
}

pub type Fields = IndexMap<String, proc_macro2::Ident>;

type Cx<'a, Tree> = (&'a mut Tree, PathBuf);

lazy_static::lazy_static!(
    pub static ref BUILTIN_NATIVES: HashMap<&'static str, TypeGenFn<'static>> = {
        let mut ret: HashMap<&'static str, TypeGenFn> = HashMap::default();

        macro_rules! get_natives {
            ($($native_name:expr => $native_fun_name:ident $buf_getter:ident $buf_putter:ident $rstype:expr)+) => {
                $(fn $native_fun_name<'a>(
                    _: Cx<'a, super::pds::ProtoDef>,
                    index_map_cx: Cx<'a, indexmap::IndexMap<PathBuf, Type>>,
                    _: serde_json::Value,
                ) -> Option<TypeGenFn<'a>> {
                    index_map_cx.0.insert(
                        index_map_cx.1,
                        Type {
                            parsing_code: Box::new(|field_name| {
                                quote::quote! {
                                    let #field_name = buf.$buf_getter();
                                }
                            }),
                            writing_code: Box::new(|field_name| {
                                quote::quote! {
                                    buf.$buf_putter(#field_name);
                                }
                            }),
                            r#type: proc_macro2::Ident::new($rstype, Span::call_site()),
                        },
                    );
                    None
                }
                ret.insert($native_name, TypeGenFn(Box::new(&$native_fun_name)));)*
            };
        }

        get_natives!(
            "varlong" => native_varlong get_var_long put_var_long "i64"
            "varint" => native_varint get_var_int put_var_int "i32"
            "u8"  => native_u8 get_u8 put_u8 "u8"
            "i8"  => native_i8 get_i8 put_i8 "i8"
            "u16" => native_u16 get_u16 put_u16 "u16"
            "i16" => native_i16 get_i16 put_i16 "i16"
            "u32" => native_u32 get_u32 put_u32 "u32"
            "i32" => native_i32 get_i32 put_i32 "i32"
            "u64" => native_u64 get_u64 put_u64 "u64"
            "i64" => native_i64 get_i64 put_i64 "i64"
            "u128" => native_u128 get_u128 put_u128 "u128"
            "i128" => native_i128 get_i128 put_i128 "i128"
            "f32" => native_f32 get_f32 put_f32 "f32"
            "f64" => native_f64 get_f64 put_f64 "f64"
            "boolean" => native_bool get_bool put_bool "bool"
        );
        ret.insert("switch", TypeGenFn(Box::new(native_switch)));

        ret
    };
);

// {
//     u8: {
//         code_gen: || {},

//     }
// }

// {
//     "/a" : u8,
//     "/b" : String,
//     "/"
// }

pub struct TypeGenFn<'a>(
    Box<
        dyn FnMut<
                (
                    Cx<'a, super::pds::ProtoDef>,
                    Cx<'a, indexmap::IndexMap<PathBuf, Type>>,
                    serde_json::Value,
                ),
                Output = Option<TypeGenFn<'a>>,
            > + Send
            + Sync
            + 'static,
    >,
);

fn native_switch<'a>(
    pds_cx: Cx<'a, super::pds::ProtoDef>,
    index_map_cx: Cx<'a, indexmap::IndexMap<PathBuf, Type>>,
    mut raw_options: serde_json::Value,
) -> Option<TypeGenFn<'a>> {
    let options = raw_options.as_object().unwrap();
    let mut placeholder_arguments: HashMap<String, String> = Default::default();
    let mut add_placeholder_arg = |old: &str, dollar_ref: &String| {
        placeholder_arguments.insert(dollar_ref[1..].to_string(), old.to_owned());
    };
    let mut enum_code = proc_macro2::TokenStream::default();
    match options.get("compareTo") {
        Some(serde_json::Value::String(s)) if s.starts_with('$') => {
            add_placeholder_arg("compareTo", s)
        }
        Some(serde_json::Value::String(s)) => {
            let field_name = s;
            let fields = options.get("fields").unwrap().as_object().unwrap();
            enum_code.extend(quote::quote! {
                match r#type
            });
            for (field_matcher, field_type) in fields {
                // let (field_type, field_path) = todo!();
                // // lookup_type(field_type, ...);
                // enum_code.extend(quote::quote! { #field_matcher => { } });
            }
        }
        _ => match options.get("compareToValue") {
            Some(serde_json::Value::String(s)) if s.starts_with("$") => {
                add_placeholder_arg("compareToValue", s)
            }
            Some(v) => {}
            None => panic!(),
        },
    }
    if placeholder_arguments.len() > 0 {
        Some(TypeGenFn(Box::new(
            move |pds_cx: Cx<super::pds::ProtoDef>,
                  index_map_cx: Cx<IndexMap<PathBuf, Type>>,
                  placeholder_options: serde_json::Value| {
                for (alias, insertto) in &placeholder_arguments {
                    if let Some(obj) = raw_options.as_object_mut() {
                        obj.insert(
                            insertto.clone(),
                            placeholder_options.get(&alias.to_owned()).unwrap().clone(),
                        );
                    }
                }
                native_switch(pds_cx, index_map_cx, raw_options.to_owned())
            },
        )))
    } else {
        None
    }
}

impl Into<proc_macro2::TokenStream> for super::pds::ProtoDef {
    fn into(self) -> proc_macro2::TokenStream {
        todo!()
    }
}

fn a(pds_cx: Cx<super::pds::ProtoDef>, type_name: String) {
    let a = pds_cx.0 .0.get(&pds_cx.1).unwrap();

    match a {
        super::pds::Type::Reference(ref_name) => todo!(),
        super::pds::Type::Container(_) => todo!(),
        super::pds::Type::Call(_, _) => todo!(),
    }

    todo!()
}
