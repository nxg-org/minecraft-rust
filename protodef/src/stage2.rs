use std::collections::HashMap;
use std::sync::Weak;
use serde_json::Value;
use crate::ProtoDefError;

type Args = Value;

/// Output of the generating functions
///
/// Results' Meaning
/// - Err(ProtoDefError) - there was an error, maybe invalid input
/// - Ok(None) - the function is still missing type dependencies
/// - Ok(Some(RustType)) - Done! cache that shit!
type FnResult = Result<Option<RustType>, ProtoDefError>;

/// just a reference to a function
type NativeFn<'a> = fn(&'a Types, &'a Args) -> FnResult;
/// an owned struct-like dynamic function
type ExportFn<'a> = Box<dyn Fn(&'a Types, &'a Args) -> FnResult>;

/// just an optimization
pub enum BuildRustTypeFn<'a>{
	Native(NativeFn<'a>),
	Reexport(ExportFn<'a>),
}

pub type Namespace = crate::Namespace<Types>;
pub type Types = crate::Node<RustType>;

pub struct RustType{
	pub types: Weak<Types>,
	pub rst: RST,
}

pub enum RST{
	// Single{
	// 	typ: Identifier,
	// 	ser: CodeGenFn<'a>,
	// 	de: CodeGenFn<'a>,
	// },
	Option(Box<RST>),
	Struct{
		fields: HashMap<String, RST>,
	},
	Enum{
		cases: Vec<EnumCase>,
	},
}

pub struct EnumCase{
	pub name: String,
	pub typ: EnumCaseType,
}

pub enum EnumCaseType{
	Struct{
		fields: HashMap<String, RST>,
	},
	Union{
		fields: Vec<RST>,
	},
}