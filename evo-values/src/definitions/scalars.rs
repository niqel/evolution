#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextPosition(pub usize);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextLength(pub usize);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftAmount(pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerExponent(pub u32);
