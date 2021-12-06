pub enum ProtocolTypes {
    Boolean,
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    Long,
    Float,
    Double,
    String,
    Chat,
    Identifier,
    VarInt,
    VarLong,
    EntityMetadata, // !varies https://wiki.vg/Entity_metadata#Entity_Metadata_Format
    Slot,           // !varies https://wiki.vg/Slot_Data
    NbtTag,         // !varies https://wiki.vg/NBT
    /**
    An integer/block position:
    x (-33554432 to 33554431),
    y (    -2048 to     2047),
    z (-33554432 to 33554431)
    */
    Position,
    Angle,
    UUID,      // => u128
    OptionalX, //todo wtf
    ArrayOfX,  //todo wtf
    XEnum,     //todo wtf
    ByteArray, //todo wtf
}
