use std::{collections::HashMap, path::PathBuf};

use indexmap::IndexMap;
use proc_macro2::Span;

pub type CodeGenFn = Box<dyn Fn(String) -> proc_macro2::TokenStream>;

pub type TypeStore = HashMap<proc_macro2::Ident, Type>;

pub struct Type {
    pub parsing_code: CodeGenFn,
    pub writing_code: CodeGenFn,
    pub r#type: proc_macro2::Ident,
    // type_def: proc_macro2::TokenStream,
}

pub type Fields = IndexMap<String, proc_macro2::Ident>;

lazy_static::lazy_static!(
    pub static ref BUILTIN_NATIVES: HashMap<&'static str, TypeGenFn<'static>> = {
        let mut ret: HashMap<&'static str, TypeGenFn> = HashMap::default();

        macro_rules! get_natives {
            ($($native_name:expr => $native_fun_name:ident $buf_getter:ident $buf_putter:ident $rstype:expr)+) => {
                $(fn $native_fun_name<'a>(
                    _: &'a mut super::pds::ProtoDef,
                    _: PathBuf,
                    index_map: &'a mut indexmap::IndexMap<PathBuf, Type>,
                    index_map_path: PathBuf,
                    _: HashMap<String, serde_json::Value>,
                ) -> Option<TypeGenFn<'a>> {
                    index_map.insert(
                        index_map_path,
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

pub struct TypeGenFn<'a>(
    Box<
        dyn FnMut<
                (
                    &'a mut super::pds::ProtoDef,
                    PathBuf,
                    &'a mut indexmap::IndexMap<PathBuf, Type>,
                    PathBuf,
                    HashMap<String, serde_json::Value>,
                ),
                Output = Option<TypeGenFn<'a>>,
            > + Send
            + Sync
            + 'static,
    >,
);

fn native_switch<'a>(
    pds: &'a mut super::pds::ProtoDef,
    pds_path: PathBuf,
    index_map: &'a mut indexmap::IndexMap<PathBuf, Type>,
    index_map_path: PathBuf,
    mut options: HashMap<String, serde_json::Value>,
) -> Option<TypeGenFn<'a>> {
    let mut placeholder_arguments: HashMap<String, String> = Default::default();
    let mut enum_code = proc_macro2::TokenStream::default();
    match options.get("compareTo") {
        Some(v) => {
            if let Some(s) = v.as_str() {
                if s.starts_with("$") {
                    placeholder_arguments
                        .insert((&s[1..]).to_string(), "compareTo".to_string().to_string());
                } else {
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
            }
        }
        None => match options.get("compareToValue") {
            Some(v) => {}
            None => panic!(),
        },
    }
    if placeholder_arguments.len() > 0 {
        Some(TypeGenFn(Box::new(
            move |pds: &mut HashMap<PathBuf, super::pds::Type>,
                  pds_path: PathBuf,
                  index_map: &mut IndexMap<PathBuf, Type>,
                  index_map_path: PathBuf,
                  placeholder_options: HashMap<String, serde_json::Value>| {
                for (alias, insertto) in &placeholder_arguments {
                    options.insert(
                        insertto.clone(),
                        placeholder_options.get(&alias.to_owned()).unwrap().clone(),
                    );
                }
                None
            },
        )))
    } else {
        None
    }
}
