use minecraft_data::{FromMCDataVersionDir, FromVersion};
// use protodef::{Natives, RustType, Type, TypeResult, ProtoDefError};
use serde::Deserialize;

#[derive(Deserialize)]
struct Thing(protodef::stage0::Namespace);

impl FromMCDataVersionDir for Thing {
    const MODULE_NAME: &'static str = "protocol";
    const FILE_NAME: &'static str = "protocol.json";
}

fn main() {
    let Thing(json_ns) = FromVersion::from_version("1.12.2").unwrap();

    let thing: protodef::Result<protodef::stage1::Namespace> = TryFrom::try_from(json_ns);

    match thing {
        Ok(good) => println!("{:#?}", good),
        Err(e) => eprintln!("{:?}", e),
    }
}
