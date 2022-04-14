use std::path::PathBuf;

use indexmap::IndexMap;
use minecraft_protocol::protodef::*;

pub struct Type {
    pub parsing_code: CodeGenFn,
    pub writing_code: CodeGenFn,
    pub r#type: proc_macro2::Ident,
    // type_def: proc_macro2::TokenStream,
}

struct StructTreeBuilder {
    fieldtree: IndexMap<PathBuf, Type>,
}

pub struct Field {
    name: Option<String>,
    r#type: Type,
}

type a = IndexMap<PathBuf, Type>;

fn main() {
    let fields: Vec<Field> = Default::default();
    // fields.push(Field{
    //     name: Some("a"),
    //     r#type:
    // });

    let current_path = "/a/b";
    for field in fields {
        generate_type(match field {
            Field { name: Some(a), .. } => current_path + name,
            Field { name: None, .. } => current_path,
        })
    }
}
