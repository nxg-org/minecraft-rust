fn main() {}

pub mod native {
    macro_rules! num {
        ($($num:ident $ty:ident $bytes:literal $to_op:ident $from_op:ident ;)*) => {
            $(
                pub struct $ty;
                impl ::protodef::ProtoDef for $ty {
                    type Data = ::core::primitive::$num;
                    #[inline]
                    fn ser(
                        data: Self::Data,
                        buf: &mut impl ::std::io::Write,
                    ) -> ::core::result::Result<(), ::protodef::ProtoDefSerError> {
                        buf.write_all(&data.$to_op()).map_err(|_| protodef::ProtoDefSerError)
                    }
                    #[inline]
                    fn de(
                        buf: &mut impl ::std::io::Read
                    ) -> ::core::result::Result<Self::Data, ::protodef::ProtoDefDeError> {
                        let mut read = [0; $bytes];
                        buf.read_exact(&mut read)
                            .map(|_| Self::Data::$from_op(read))
                            .map_err(|_| protodef::ProtoDefDeError)
                    }
                }
            )*
        };
    }

    num! {
        u8 Bu8 1 to_be_bytes from_be_bytes;
        u16 Bu16 2 to_be_bytes from_be_bytes;
        u32 Bu32 4 to_be_bytes from_be_bytes;
        u64 Bu64 8 to_be_bytes from_be_bytes;
        u128 Bu128 16 to_be_bytes from_be_bytes;

        i8 Bi8 1 to_be_bytes from_be_bytes;
        i16 Bi16 2 to_be_bytes from_be_bytes;
        i32 Bi32 4 to_be_bytes from_be_bytes;
        i64 Bi64 8 to_be_bytes from_be_bytes;
        i128 Bi128 16 to_be_bytes from_be_bytes;

        u8 Lu8 1 to_le_bytes from_le_bytes;
        u16 Lu16 2 to_le_bytes from_le_bytes;
        u32 Lu32 4 to_le_bytes from_le_bytes;
        u64 Lu64 8 to_le_bytes from_le_bytes;
        u128 Lu128 16 to_le_bytes from_le_bytes;

        i8 Li8 1 to_le_bytes from_le_bytes;
        i16 Li16 2 to_le_bytes from_le_bytes;
        i32 Li32 4 to_le_bytes from_le_bytes;
        i64 Li64 8 to_le_bytes from_le_bytes;
        i128 Li128 16 to_le_bytes from_le_bytes;
    }

    pub type U8 = Bu8;
    pub type U16 = Bu16;
    pub type U32 = Bu32;
    pub type U64 = Bu64;
    pub type U128 = Bu128;
    pub type I8 = Bi8;
    pub type I16 = Bi16;
    pub type I32 = Bi32;
    pub type I64 = Bi64;
    pub type I128 = Bi128;

    // native PString parse declaration here...
    pub struct PString<CountType> {
        marker: std::marker::PhantomData<CountType>,
    }
    impl<CountType> ::protodef::ProtoDef for PString<CountType>
    where
        CountType: ::protodef::ProtoDef,
        <CountType as ::protodef::ProtoDef>::Data: TryFrom<usize> + Into<usize>,
    {
        type Data = ::std::string::String;

        fn ser(
            data: Self::Data,
            buf: &mut impl std::io::Write,
        ) -> core::result::Result<(), protodef::ProtoDefSerError> {
            let bytes = data.as_bytes();
            bytes
                .len()
                .try_into()
                .map_err(|_| ::protodef::ProtoDefSerError)
                .and_then(|len| {
                    <CountType as ::protodef::ProtoDef>::ser(len, buf).and(
                        buf.write_all(bytes)
                            .map_err(|_| ::protodef::ProtoDefSerError),
                    )
                })
        }

        fn de(
            buf: &mut impl std::io::Read,
        ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
            <CountType as ::protodef::ProtoDef>::de(buf).and_then(|len| {
                let mut read = vec![0; len.into()];
                buf.read_exact(&mut read)
                    .map_err(|_| ::protodef::ProtoDefDeError)
                    .and(
                        ::std::string::String::from_utf8(read)
                            .map_err(|_| ::protodef::ProtoDefDeError),
                    )
            })
        }
    }

    pub struct VarInt;
    impl ::protodef::ProtoDef for VarInt {
        type Data = i32;

        fn ser(
            data: Self::Data,
            buf: &mut impl std::io::Write,
        ) -> core::result::Result<(), protodef::ProtoDefSerError> {
            todo!()
        }

        fn de(
            buf: &mut impl std::io::Read,
        ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
            todo!()
        }
    }

    pub struct VarLong;
    impl ::protodef::ProtoDef for VarLong {
        type Data = i64;

        fn ser(
            data: Self::Data,
            buf: &mut impl std::io::Write,
        ) -> core::result::Result<(), protodef::ProtoDefSerError> {
            todo!()
        }

        fn de(
            buf: &mut impl std::io::Read,
        ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
            todo!()
        }
    }

    pub struct Void;
    impl ::protodef::ProtoDef for Void {
        type Data = ();
        #[inline]
        fn ser(
            _data: Self::Data,
            _buf: &mut impl std::io::Write,
        ) -> core::result::Result<(), protodef::ProtoDefSerError> {
            Ok(())
        }
        #[inline]
        fn de(
            _buf: &mut impl std::io::Read,
        ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
            Ok(())
        }
    }
}

pub mod versions {
    pub struct V1_12_2;
    pub mod v1_12_2 {
        //! ```json
        //! {
        //!     "types": {
        //!         "u8": "native",
        //!         "pstring": "native",
        //!         "string": [
        //!             "pstring",
        //!             {
        //!                 "countType": "u8"
        //!             }
        //!         ],
        //!         "container": "native"
        //!     },
        //!     "namespace": {
        //!         "types": {
        //!             "packet": [
        //!                 "container",
        //!                 [
        //!                     {
        //!                         "name": "id",
        //!                         "type": "u8"
        //!                     },
        //!                     {
        //!                         "name": "name",
        //!                         "type": "string"
        //!                     }
        //!                 ]
        //!             ]
        //!         }
        //!     }
        //! }
        //! ```
        pub type String = super::super::native::PString<super::super::native::U8>;
        pub mod namespace {
            pub struct Packet {
                pub id: ::core::primitive::u8,
                pub name: ::std::string::String,
            }
            #[allow(non_camel_case_types)]
            pub type Packet_id = super::super::super::native::U8;
            #[allow(non_camel_case_types)]
            pub type Packet_name = super::String;
            impl protodef::ProtoDef for Packet {
                type Data = Packet;
                fn ser(
                    Packet { id, name }: Self::Data,
                    buf: &mut impl ::std::io::Write,
                ) -> core::result::Result<(), protodef::ProtoDefSerError> {
                    Packet_id::ser(id, buf).and(Packet_name::ser(name, buf))
                }
                fn de(
                    buf: &mut impl ::std::io::Read,
                ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
                    Packet_id::de(buf)
                        .and_then(|id| Packet_name::de(buf).map(|name| (id, name)))
                        .map(|(id, name)| Self::Data { id, name })
                }
            }
        }
    }

    pub struct V1_18_1;
    pub mod v1_18_1 {
        //! ```json
        //! {
        //!     "types": {
        //!         "u32": "native",
        //!         "container": "native"
        //!     },
        //!     "namespace": {
        //!         "types": {
        //!             "packet": [
        //!                 "container",
        //!                 [
        //!                     {
        //!                         "name": "id",
        //!                         "type": "u32"
        //!                     }
        //!                 ]
        //!             ]
        //!         }
        //!     }
        //! }
        //! ```
        pub mod namespace {
            pub struct Packet {
                pub id: u32,
            }
            #[allow(non_camel_case_types)]
            pub type Packet_id = super::super::super::native::U32;
            impl ::protodef::ProtoDef for Packet {
                type Data = Packet;
                fn ser(
                    Self::Data { id }: Self::Data,
                    buf: &mut impl std::io::Write,
                ) -> core::result::Result<(), protodef::ProtoDefSerError> {
                    Packet_id::ser(id, buf)
                }
                fn de(
                    buf: &mut impl std::io::Read,
                ) -> core::result::Result<Self::Data, protodef::ProtoDefDeError> {
                    Packet_id::de(buf).map(|id| Self::Data { id })
                }
            }
        }
    }
}

pub mod enum_like {
    pub mod namespace {
        pub enum Packet {
            V1_12_2(super::super::versions::v1_12_2::namespace::Packet),
            V1_18_1(super::super::versions::v1_18_1::namespace::Packet),
        }
        impl From<super::super::versions::v1_12_2::namespace::Packet> for Packet {
            fn from(this: super::super::versions::v1_12_2::namespace::Packet) -> Self {
                Self::V1_12_2(this)
            }
        }
        impl From<super::super::versions::v1_18_1::namespace::Packet> for Packet {
            fn from(this: super::super::versions::v1_18_1::namespace::Packet) -> Self {
                Self::V1_18_1(this)
            }
        }
    }
}

pub mod merged {
    pub mod namespace {
        #[derive(Default)]
        pub struct Packet {
            pub id: ::core::primitive::u32,
            pub name: ::core::option::Option<::std::string::String>,
        }
        impl From<super::super::enum_like::namespace::Packet> for Packet {
            fn from(this: super::super::enum_like::namespace::Packet) -> Self {
                use super::super::enum_like::namespace::Packet::*;
                match this {
                    V1_12_2(this) => this.into(),
                    V1_18_1(this) => this.into(),
                }
            }
        }
        impl From<super::super::versions::v1_12_2::namespace::Packet> for Packet {
            #[inline(always)]
            fn from(
                super::super::versions::v1_12_2::namespace::Packet{ id, name }: super::super::versions::v1_12_2::namespace::Packet,
            ) -> Self {
                Packet {
                    id: id.into(),
                    name: Some(name),
                }
            }
        }
        impl From<super::super::versions::v1_18_1::namespace::Packet> for Packet {
            #[inline(always)]
            fn from(
                super::super::versions::v1_18_1::namespace::Packet{ id }: super::super::versions::v1_18_1::namespace::Packet,
            ) -> Self {
                Packet {
                    id,
                    ..Default::default()
                }
            }
        }
    }
}
/*pub struct SwitchTest0 {
    pub id: i32,
    pub data: SwitchTest0__enum_data,
}
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub type SwitchTest0_id = super::super::super::native::VarInt;
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub enum SwitchTest0__enum_data {
    C0(i32),
} */
