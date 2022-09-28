#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Bool,

    Bit,
    Byte,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint,
    BigUint,

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int,
    BigInt,

    Float32,
    Float64,
    Float,
    BigNum,

    Char,
    String,
}

