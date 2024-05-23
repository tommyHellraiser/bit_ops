
use crate::data::traits::BitwiseOps;
use crate::{implement_bitwise_ops, implement_from_traits, implement_methods};

type Bits8Type = u8;
type Bits16Type = u16;
type Bits32Type = u32;
type Bits64Type = u64;
type Bits128Type = u128;

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub enum Endianness {
    #[default]
    LittleEndian,
    BigEndian
}

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bits8(Bits8Type, Endianness);

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bits16(Bits16Type, Endianness);

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bits32(Bits32Type, Endianness);

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bits64(Bits64Type, Endianness);

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bits128(Bits128Type, Endianness);


//  ---- Implement methods for bits structs
implement_methods!(Bits8, Bits8Type, 8);
implement_methods!(Bits16, Bits16Type, 16);
implement_methods!(Bits32, Bits32Type, 32);
implement_methods!(Bits64, Bits64Type, 64);
implement_methods!(Bits128, Bits128Type, 128);


//  ---- Implement Bitwise ops trait ---- 
implement_bitwise_ops!(Bits8, Bits8Type, 8);
implement_bitwise_ops!(Bits16, Bits16Type, 16);
implement_bitwise_ops!(Bits32, Bits32Type, 32);
implement_bitwise_ops!(Bits64, Bits64Type, 64);
implement_bitwise_ops!(Bits128, Bits128Type, 128);


//  ---- Implement From traits
//  Implement From any for Bits8
implement_from_traits!(Bits8, Bits8Type, (Bits16, Bits32, Bits64, Bits128));
//  Implement From any for Bits16
implement_from_traits!(Bits16, Bits16Type, (Bits8, Bits32, Bits64, Bits128));
//  Implement From any for Bits32
implement_from_traits!(Bits32, Bits32Type, (Bits8, Bits16, Bits64, Bits128));
// //  Implement From any for Bits64
implement_from_traits!(Bits64, Bits64Type, (Bits8, Bits16, Bits32, Bits128));
// //  Implement From any for Bits128
implement_from_traits!(Bits128, Bits128Type, (Bits8, Bits16, Bits32, Bits64));
