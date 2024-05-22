
pub trait BitwiseOps {
    fn bitwise_and(self, other: Self) -> Self;
    fn bitwise_or(self, other: Self) -> Self;
    fn bitwise_xor(self, other: Self) -> Self;
    fn bitwise_not(self) -> Self;
    fn shift_left(self, amount: usize) -> Self;
    fn shift_right(self, amount: usize) -> Self;
    fn set_bit(self, bit: usize) -> Self;
    fn clear_bit(self, bit: usize) -> Self;
    fn toggle_bit(self, bit: usize) -> Self;
    fn is_bit_set(&self, bit: usize) -> bool;
    //Extracting specific bits (bit masking)
    //Count set bits (population count)
    //Count unset bits
    //Count leading zeros
    //Count trailing zeros
    //Rotate left
    //Rotate right
    //Isolate the rightmost 1-bit
    //Isolate the rightmost 0-bit
    //Set the rightmost 0-bit
    //Reset the rightmost 1-bit
    //Ones compliment
    //Twos compliment
    //Nibble swapping
    //Change endianness
}
