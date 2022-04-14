#![feature(hash_drain_filter)]
#![feature(iter_partition_in_place)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Write, Read};
use std::sync::{Arc, Weak};

pub mod stage0;
pub mod stage1;
pub mod stage2;

#[derive(Debug)]
pub struct Namespace<T> {
    pub types: Arc<T>,
    pub sub: HashMap<String, Namespace<T>>,
}

pub struct Node<T> {
    pub types: HashMap<String, Arc<T>>,
    /// optional reference to parent types for resolving types
    /// this is effectively a linked list of all upper type maps
    pub parent: Weak<Node<T>>,
}
impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.types).finish()
    }
}

pub type Result<T> = core::result::Result<T, ProtoDefError>;

#[non_exhaustive]
#[derive(Debug)]
pub enum ProtoDefError {
    NativeNotFound(String),
    CallWrongType(String),
    SuppliedArgumentsTakesNone(String),
    Unresolvable,
    Other,
}

pub struct ProtoDefSerError;
pub struct ProtoDefDeError;
pub trait ProtoDef {
    type Data;
    fn ser(data: Self::Data,buf: &mut impl Write) -> core::result::Result<(), ProtoDefSerError>;
    fn de(buf: &mut impl Read) -> core::result::Result<Self::Data, ProtoDefDeError>;
}
