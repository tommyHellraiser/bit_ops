
#[macro_export]
macro_rules! implement_bitwise_ops {
    ($self_type: ty) => {
        impl BitwiseOps for $self_type {
            fn bitwise_and(self, other: $self_type) -> $self_type {
                Self(self.0 & other.0)
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
                $bit_type(value)
            }
            pub fn get_value(&self) -> $bit_content_type {
                self.0
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