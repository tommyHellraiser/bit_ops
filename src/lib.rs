mod data;
pub use data::structs;
use crate::structs::Bit8;

pub fn test() {
    
    let value = Bit8::new(0b11111111);
    
    dbg!(&value.as_hex());
    dbg!(&value.as_hex_uppercase());
    dbg!(&value.as_binary());
}

#[cfg(test)]
mod tests {
    use crate::test;

    #[test]
    fn it_works() {
        test();
        
        assert!(false);
    }
}
