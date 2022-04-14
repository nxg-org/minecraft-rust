use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

pub type Namespace = crate::Namespace<Types>;
pub type Types = crate::Node<Type>;

impl Types {
    pub fn resolve_reference(&self, k: &str) -> Option<(Arc<Type>, usize)> {
        self.resolve_ref(k, 0)
    }

    fn resolve_ref(&self, k: &str, lvl: usize) -> Option<(Arc<Type>, usize)> {
        self.types.get(k).map(|arc| (arc.clone(), lvl)).or_else(|| {
            self.parent
                .upgrade()
                .and_then(|parent| parent.resolve_ref(k, lvl + 1))
        })
    }
}

#[derive(Debug)]
pub enum Type {
    Native {
        name: String,
    },
    Reference {
        /// to know how many `super` to prepend
        up: usize,
        /// the actual name of the type
        name: String,
        /// a reference to the type
        resolved: std::sync::Arc<Type>,
    },
    Call {
        called: std::sync::Arc<Type>,
        args: serde_json::Value,
    },
}

impl TryFrom<crate::stage0::Namespace> for Namespace {
    type Error = crate::ProtoDefError;
    fn try_from(json: crate::stage0::Namespace) -> crate::Result<Self> {
        Namespace::new(json, Weak::new())
    }
}

impl Namespace {
    fn new(
        mut json: crate::stage0::Namespace,
        parent: Weak<crate::Node<Type>>,
    ) -> crate::Result<Self> {
        let mut types = Types {
            types: HashMap::with_capacity(json.types.len()),
            parent,
        };

        pub fn resolve_type(
            old: &mut HashMap<String, crate::stage0::Type>,
            new: &mut Types,
            k: &str,
        ) -> crate::Result<(Arc<Type>, usize)> {
            fn resolve_typ(
                old: &mut super::stage0::Types,
                new: &mut Types,
                k: &str,
                lvl: usize,
            ) -> crate::Result<(Arc<Type>, usize)> {
                old.remove_entry(k)
                    .map(|(k, v)| {
                        use crate::stage0::Type::*;
                        match v {
                            Native => Ok(Type::Native { name: k }),
                            Reference { ref_name } => {
                                resolve_type(old, new, &ref_name).map(|(resolved, up)| {
                                    new.types.insert(ref_name.clone(), resolved.clone());
                                    Type::Reference {
                                        up,
                                        resolved,
                                        name: ref_name,
                                    }
                                })
                            }
                            Call { called_type, args } => {
                                resolve_type(old, new, &called_type).map(|(called, _)| {
                                    new.types.insert(called_type, called.clone());
                                    Type::Call { called, args }
                                })
                            }
                        }
                        .map(Arc::new)
                        .map(|arc| (arc, lvl))
                    })
                    .unwrap_or_else(|| {
                        new.resolve_reference(k)
                            .ok_or(crate::ProtoDefError::Unresolvable)
                    })
            }
            resolve_typ(old, new, k, 0)
        }

        while let Some((k, _)) = json.types.iter().next() {
            // we can be sure that when it wasn't visited yet, that it's still there
            #[allow(mutable_borrow_reservation_conflict)]
            let (k, v) = unsafe {
                json.types
                    .remove_entry(&*(k as *const _))
                    .unwrap_unchecked()
            };

            use crate::stage0::Type::*;
            let typ = match v {
                Native => Type::Native { name: k.clone() },
                Reference { ref_name } => {
                    match resolve_type(&mut json.types, &mut types, &ref_name) {
                        Ok((resolved, up)) => {
                            types.types.insert(ref_name.clone(), resolved.clone());
                            Type::Reference {
                                up,
                                name: ref_name,
                                resolved,
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Call { called_type, args } => {
                    match resolve_type(&mut json.types, &mut types, &called_type) {
                        Ok((called, _)) => {
                            types.types.insert(called_type, called.clone());
                            Type::Call { called, args }
                        }
                        Err(e) => return Err(e),
                    }
                }
            };
            types.types.insert(k, Arc::new(typ));
        }

        let mut ret = Namespace {
            types: Arc::new(types),
            sub: HashMap::with_capacity(json.sub.len()),
        };

        for (k, json) in json.sub {
            match Namespace::new(json, Arc::downgrade(&ret.types)) {
                Ok(ns) => ret.sub.insert(k, ns),
                Err(e) => return Err(e),
            };
        }

        Ok(ret)
    }
}
