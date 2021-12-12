use self::native::WriteMCNativeTypes;
use self::string::WriteMCString;
use self::var_int::WriteMCVarInt;
use self::var_long::WriteMCVarLong;

pub mod native;
pub mod string;
pub mod var_int;
pub mod var_long;

pub trait WriteMCTypes:
    WriteMCNativeTypes + WriteMCString + WriteMCVarInt + WriteMCVarLong
{
}
