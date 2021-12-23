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
    pub static ref BUILTIN_NATIVES: HashMap<&'static str, TypeGenFn> = {
        let mut ret: HashMap<&'static str, TypeGenFn> = HashMap::default();

        macro_rules! get_natives {
            ($($native_name:expr => $native_fun_name:ident $buf_getter:ident $buf_putter:ident $rstype:expr)+) => {
                $(fn $native_fun_name(
                    _: &mut super::pds::ProtoDef,
                    _: PathBuf,
                    index_map: &mut indexmap::IndexMap<PathBuf, Type>,
                    index_map_path: PathBuf,
                    _: HashMap<String, serde_json::Value>,
                ) -> Option<TypeGenFn> {
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
                ret.insert($native_name, TypeGenFn(&$native_fun_name));)*
            };
        }

        get_natives!(
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

        ret
    };
);

pub struct TypeGenFn(
    &'static (dyn Fn(
        &mut super::pds::ProtoDef,
        PathBuf,
        &mut indexmap::IndexMap<PathBuf, Type>,
        PathBuf,
        HashMap<String, serde_json::Value>,
    ) -> Option<TypeGenFn>
                  + Sync),
);
