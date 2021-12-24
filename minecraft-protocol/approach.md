# the goal

the goal is to go from the json protocol spec files to
rust code that can parse and write packets

# the approach

## 1. json => rust json representation

first the json data has to be serialized into rust native types

```rs
pub mod json {
    /**
     * ProtoDef Namespaces
     * https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/protocol.md#protocol
     * directly taken from the protocol.json files found in minecraft-data
     */
    #[derive(Serialize)]
    pub struct Namespace{
        #[serde(default)] // just use an empty hashmap
                          // if types wasn't provided
        types: HashMap<String, serde_json::Value>,
        #[serde(flatten)]
        sub: HashMap<String, Namespace>
    }
}

// this will get the root minecraft protocol spec's namespace
// and print it to console
fn example(){
    println!("{:#?}", json::Namespace::from_version("1.12.2"));
}
```

## 2. rust json representation => rust pds representation

next, this heavily on serde_json::Value reliant tree
structure has to be turned into something similar to
protodef's pds format, which is still wip at the time
of writing

````rs
pub mod pds {

    /**
     * ProtoDefRoot represents the root of a protodef
     * specification in a pds file format. Every Type
     * appearing will here be indexed by a path of
     * namespaces it is contained in
     */
    pub type ProtoDefRoot = HashMap<PathBuf, TypeRoot>;

    /**
     * TypeRoot represents any kind of Type value in the
     * protocol, be it as a native declaration in the top
     * level namespace's types or a reference to those in
     * the definition of a packet or even a container or
     * switch on some other type.
     */
    pub enum TypeRoot {
        /**
         * The Reference kind would be represented in
         * a json protocol spec by a json string, that
         * to be resolved has to be looked up in the
         * parent namespaces. If it cannot be found then
         * the protocol spec file is written badly
         *
         * spec example:
         *
         * ```json
         * {
         *   "types": {
         *     "u64": "native" /* <= a native reference,
         *            which will have to be treated
         *            differently by the compiler */
         *   },
         *   "subnamespace": {
         *     "types": {
         *       "some_type": "u64" // <= the typical type
         *                          // of reference
         *     }
         *   }
         * }
         * ```
         */
        Reference(String),
        /**
         * The Container kind contains multiple fields
         * of types in a particular order. The order of
         * the fields is important because it is also the
         * order the fields have to be read from or
         * written to a buffer
         *
         * spec example:
         *
         * ```json
         * {
         *   "types": {
         *     "u64": "native",
         *     "u8": "native"
         *   },
         *   "subnamespace": {
         *     "types": {
         *       "a_packet": [
         *         "container",
         *         [
         *           {
         *             "name": "first_field",
         *             "type": "u64"
         *           }
         *         ]
         *       ]
         *     }
         *   }
         * }
         * ```
         */
        Container(Vec<Field>),
        /**
         * The Call kind is similar to the Container in
         * syntax, having the type to call in the first
         * index and the "arguments" which to pass to
         * that call inside of the second index. it is
         * assumed you can only pass objects to the
         * second index.
         *
         * there are many builtin calls like switch,
         * option, array, buffer, ...
         * documented here:
         * https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/datatypes.md
         *
         * spec example:
         *
         * ```json
         * {
         *   "types": {
         *     "u64": "native",
         *     "u8": "native",
         *     /* any type referenced has
         *        to be declared as well, if
         *        switch wasn't declared as native
         *        here, the compiler should also error.
         *        the exact explanation of what switch is, here:
         *        https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/datatypes/conditional.md#
         *      */
         *     "switch": "native"
         *   },
         *   "subnamespace": {
         *     "types": {
         *       "a_packet": [
         *         "container",
         *         [
         *           {
         *             "name": "some_field",
         *             "type": "u8"
         *           },
         *           {
         *             "name": "special_field",
         *             "type": [
         *               "switch",
         *               {
         *                 "compareTo": "some_field",
         *                 "fields": {
         *                   "0": "u8",
         *                   "1": "u64"
         *                 },
         *                 "default": "void"
         *               }
         *             ]
         *           }
         *         ]
         *       ]
         *     }
         *   }
         * }
         * ```
         */
        Call(Box<TypeRoot>, HashMap<String, serde_json::Value>),
    }
}
````

## 3. rust pds representation => rust code generating closures

to be able to generate code from these types, we will have to provide
an implementation for "native" types first. The ability to provide custom native types should also be given. so first there should be a
definition on what a native type should be able to do.
a native type should be able to

- take as input the name its field will have
- output rust native parsing code
- output rust native writing code
- define which rust native type it represents

```rs
pub mod gen {

}
```

HashMap<String, Fn(&mut ProtoDef, field_path) ->  >

IndexMap<Path, Type>

struct Type {
    parsing_code: |field_name| proc_macro2::TokenStream,
    writing_code: |field_name| proc_macro2::TokenStream,
    type: proc_macro2::Ident
}

fn parse_packet_that(buf: Bytes){
    let r#type = buf.get_var_int();
    match r#type {

}

1.
field is not anonymous => whatever type returns is going to be scoped
as "/type"


=> insert("/type", i32)
2. 


pub type Native = Box<dyn Fn(&mut ProtoDefRoot, PathBuf, &mut IndexMap<PathBuf, Type>, PathBuf, options: HashMap<String, serde_json::Value>) -> Option<Native>>;

NATIVES {
    "varint" = 
        |pds: &mut Protodef, pds_path: Path, index_map: &mut IndexMap, index_map_path: Path, options: HashMap<String, serde_json::Value>| -> Option<Self> {
            index_map.insert(index_map_path, Type{
                parsing_code: 
                    |field_name| {
                        quote::quote!{
                            let #field_name = buf.get_var_int();
                        },
                    },
                writing_code:
                    |field_name| {
                        quote::quote!{
                            buf.put_var_int(#field_name);
                        }
                    },
                type: proc_macro2::Ident(i32, Span::here())
            })
            None
        }
    "switch" =
        |pds: &mut Protodef, pds_path: Path, index_map: &mut IndexMap, index_map_path: Path, options: HashMap<String, serde_json::Value>| -> Option<Self> {
            let placeholder_arguments: HashMap<String, String> = Default::default();
            let enum_code = proc_macro2::TokenStream::default();
            match options.get("compareTo") {
                Some(v) => {
                    if v.starts_with("$") {
                        placeholder_arguments.insert(v - "$", "compareTo");
                    } else {
                        let field_name = v;
                        let fields = options.get("fields").unwrap().as_object().unwrap();
                        enum_code.extend(
                            quote::quote!{
                                match r#type {
                            }
                        );
                        for (field_matcher, field_type) of fields {
                            let (field_type, field_path) = lookup_type(field_type, ...);
                            enum_code.extend(
                                quote::quote!{ #field_matcher => { }
                            );

/
                        }
                    }
                },
                None => match options.get("compareToValue") {
                    Some(v) => {
/
                    },
                    None => panic!(),
                }
            }
            if placeholder_arguments.length > 0 {
                return Some(|pds: &mut Protodef, pds_path: Path, index_map: &mut IndexMap, index_map_path: Path, placeholder_options: HashMap<String, serde_json::Value>| {
                    for (alias, insertto) in placeholder_arguments {
                        options.insert(insertto, placeholder_options.get(alias).unwrap());
                    }
/
                })
            }
            None
        }
}


/*
{
  "compareTo":"type",
  "fields":{
    "0":[
      "container",
      [
        {
          "name": "displayedRecipe",
          "type": "i32"
        }
      ]
    ],
    "1": [
      "container",
      [
        {
          "name": "craftingBookOpen",
          "type": "bool"
        },
        {
          "name": "craftingFilter",
          "type": "bool"
        }
      ]
    ]
  }
}*/



##### types

how to build them from pds

1. call get type on pds_context
2. call get type on all children

{

}








HashMap<PathBuf, Type>
"/play/to_client/packet" => Type::Container([
    {
        name: "name", 
        type: Type::Call(
            Box(Type::Reference("mapper")),
            {"type": "varint","mappings":{}}
        )
    },
    {
        anon: true,
        type: Type::Call(
            Box(Type::Reference("switch")),
            {
                "compareTo": "name",
                "fields": {}
            }
        )
    }
])


pub enum Type {
    Reference(String),
    Container(Vec<Field>),
    Call(Box<Type>, serde_json::Value),
}






IndexMap<String, >






struct #aaa {

}
















