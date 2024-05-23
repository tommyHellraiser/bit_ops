
#[macro_export]
macro_rules! implement_bitwise_ops {
    ($self_type: ty, $self_content_type: ty, $size: expr) => {
        impl BitwiseOps for $self_type {
            type ContentType = $self_content_type;
            
            fn bitwise_and(&self, other: $self_type) -> $self_type {
                Self(self.0 & other.0, Endianness::LittleEndian)
            }
            fn bitwise_or(&self, other: $self_type) -> $self_type {
                Self(self.0 | other.0, Endianness::LittleEndian)
            }
            fn bitwise_xor(&self, other: $self_type) -> $self_type {
                Self(self.0 ^ other.0, Endianness::LittleEndian)
            }
            fn bitwise_not(&self) -> $self_type {
                Self(!self.0, Endianness::LittleEndian)
            }
            fn shift_left(&mut self, amount: usize) {
                self.0 <<= amount;
            }
            fn shift_right(&mut self, amount: usize) {
                self.0 >>= amount;
            }
            fn set_bit(&mut self, bit: usize) {
                self.0 |= (1 << bit);
            }
            fn clear_bit(&mut self, bit: usize) {
                self.0 &= !(1 << bit);
            }
            fn toggle_bit(&mut self, bit: usize) {
                self.0 ^= (1 << bit);
            }
            fn is_bit_set(&self, bit: usize) -> bool {
                (self.0 & 1 << bit) == 0
            }
            fn count_set_bits(&self) -> usize {
                self.0.count_ones() as usize
            }
            fn count_unset_bits(&self) -> usize {
                self.0.count_zeros() as usize
            }
            fn leading_zeroes(&self) -> usize {
                self.0.leading_zeros() as usize
            }
            fn leading_ones(&self) -> usize {
                self.0.leading_ones() as usize
            }
            fn trailing_zeroes(&self) -> usize {
                self.0.trailing_zeros() as usize
            }
            fn trailing_ones(&self) -> usize {
                self.0.trailing_ones() as usize
            }
            fn twos_complement_of(value: $self_content_type) -> Self {
                let mask = (1 << ($size - 1)) - 1;
                Self((!value & mask).wrapping_add(1) & mask, Endianness::LittleEndian)
            }
        }
    };
}

#[macro_export]
macro_rules! implement_from_traits {
    ($destination_type: tt, $destination_content_type: ty, ($($origin_type: ty),+)) => {
        $(
            impl From<$origin_type> for $destination_type {
                fn from(value: $origin_type) -> $destination_type {
                    $destination_type::new(value.0 as $destination_content_type)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! implement_methods {
    ($bit_type: tt, $bit_content_type: ty, $amount_of_bits: expr) => {
        impl $bit_type {
            pub fn new(value: $bit_content_type) -> $bit_type {
                $bit_type(value, Endianness::LittleEndian)
            }
            pub fn get_value(&self) -> $bit_content_type {
                self.0
            }
            pub fn get_endianness(&self) -> Endianness {
                self.1
            }
            pub fn as_hex(&self) -> String {
                let digits = $amount_of_bits / 4;
                format!("0x{:0digits$x}", self.0)
            }
            pub fn as_hex_uppercase(&self) -> String {
                let digits = $amount_of_bits / 4;
                format!("0x{}", format!("{:0digits$x}", self.0).to_uppercase())
            }
            pub fn as_binary(&self) -> String {
                let digits = $amount_of_bits;
                format!("0b{:0digits$b}", self.0)
            }
        }
    };
}