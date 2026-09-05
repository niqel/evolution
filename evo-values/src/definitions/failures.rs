#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOperationFailure {
    OutOfBounds,
    EmptyPattern,
    EmptySeparator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericFailure {
    Overflow,
    DivisionByZero,
    InvalidBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseFailure {
    InvalidShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonFailure {
    DifferentFamily,
    NotComparable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFailure {
    NotExactlyRepresentable,
}
