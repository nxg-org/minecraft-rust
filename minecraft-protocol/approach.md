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
