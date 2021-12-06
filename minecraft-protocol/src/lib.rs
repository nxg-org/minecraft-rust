#![feature(cursor_remaining)]
pub mod protocol_types;
pub mod buffer;

pub mod protocol {
    pub enum PacketDirection {
        ClientBound,
        ServerBound,
    }
    impl PacketDirection {
        pub fn opposite(&self) -> Self {
            use PacketDirection::*;
            match self {
                ClientBound => ServerBound,
                ServerBound => ClientBound,
            }
        }
    }
    pub enum State {
        Handshaking,
        Status,
        Login,
        Play,
    }
    impl State {
        pub fn name(&self) -> &'static str {
            use State::*;
            match self {
                Handshaking => "Handshaking",
                Status => "Status",
                Login => "Login",
                Play => "Play",
            }
        }
    }
    pub struct Id {
        pub id: i32,
        pub state: State,
        pub direction: PacketDirection,
    }
    //todo serialize

    pub struct ProtocolPacketField {
        pub name: String,
        pub kind: String,
    }
    pub struct ProtocolPacketSpec {
        pub state: String,
        pub direction: String,
        pub id: i32,
        pub name: String,
        pub body_struct: String,
        pub fields: Vec<ProtocolPacketField>,
    }
    pub struct ProtocolSpec {
        pub name: String,
        pub packets: Vec<ProtocolPacketSpec>,
    }
}
