
type Bit8Type = u8;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Bit8(Bit8Type);

impl Bit8 {
    pub fn new(value: Bit8Type) -> Self {
        Self(value)
    }
    pub fn get_value(&self) -> Bit8Type {
        self.0
    }
    pub fn as_hex(&self) -> String {
        format!("0x{:x}", self.0)
    }
    pub fn as_hex_uppercase(&self) -> String {
        format!("0x{}", format!("{:x}", self.0).to_uppercase())
    }
    pub fn as_binary(&self) -> String {
        format!("0b{:b}", self.0)
    }
}
