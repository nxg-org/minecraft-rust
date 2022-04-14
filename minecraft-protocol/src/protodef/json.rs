use minecraft_data::FromMCDataVersionDir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ProtoDef = Namespace;

/**
 * ProtoDef Namespaces
 * <https://github.com/ProtoDef-io/ProtoDef/blob/master/doc/protocol.md#protocol>
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
    const MODULE_NAME: &'static str = "protocol";
    const FILE_NAME: &'static str = "protocol.json";
}

#[test]
fn test() {
    use minecraft_data::FromVersion;
    for v in minecraft_data::supported_versions::SUPPORTED_VERSIONS {
        println!("{:#?}", ProtoDef::from_version(v).unwrap());
    }
}
