mod data;
pub use data::structs;
use crate::data::traits::BitwiseOps;
use crate::structs::{Bits16, Bits32, Bits8};

pub fn test() {
    
    let bit8 = Bits8::new(0b111);
    let bit16 = Bits16::new(0b1111000100100110);
    
    let result = bit16.bitwise_and(bit8.into());
    let result_32 = Bits32::from(result);

    dbg!(&result.as_hex());
    dbg!(&result.as_hex_uppercase());
    dbg!(&result.as_binary());
    
    dbg!("IMPLS for Bits32");
    dbg!(&result_32.as_hex());
    dbg!(&result_32.as_hex_uppercase());
    dbg!(&result_32.as_binary());
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        super::test();
        
        assert!(false);
    }
}
