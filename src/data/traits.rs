
pub trait BitwiseOps {
    type ContentType;
    fn bitwise_and(&self, other: Self) -> Self;
    fn bitwise_or(&self, other: Self) -> Self;
    fn bitwise_xor(&self, other: Self) -> Self;
    fn bitwise_not(&self) -> Self;
    fn shift_left(&mut self, amount: usize);
    fn shift_right(&mut self, amount: usize);
    fn set_bit(&mut self, bit: usize);
    fn clear_bit(&mut self, bit: usize);
    fn toggle_bit(&mut self, bit: usize);
    fn is_bit_set(&self, bit: usize) -> bool;
    fn count_set_bits(&self) -> usize;
    fn count_unset_bits(&self) -> usize;
    fn leading_zeroes(&self) -> usize;
    fn leading_ones(&self) -> usize;
    fn trailing_zeroes(&self) -> usize;
    fn trailing_ones(&self) -> usize;
    fn twos_complement_of(value: Self::ContentType) -> Self;
    
    //As BCD?
    //Nibble swapping
    //Change endianness
    //Extracting specific bits (bit masking)
    //Rotate left
    //Rotate right
    //Isolate the rightmost 1-bit
    //Isolate the rightmost 0-bit
    //Set the rightmost 0-bit
    //Reset the rightmost 1-bit
}
