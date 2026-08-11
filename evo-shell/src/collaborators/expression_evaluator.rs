use crate::collaborators::arithmetic;
use crate::definitions::structs::borrowed::number_binding::NumberBinding;
use crate::definitions::types::number::Number;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Arithmetic(arithmetic::Error),

    EmptyExpression,
    UnexpectedToken,
    UnexpectedEnd,
    MissingClosingParenthesis,
    UnknownIdentifier,
    InvalidNumber,
}

fn resolve_identifier(ident: &str, bindings: &[NumberBinding<'_>]) -> Result<Number, Error> {
    for b in bindings {
        if b.name == ident {
            return Ok(b.value);
        }
    }
    Err(Error::UnknownIdentifier)
}

fn parse_number_literal<'a>(s: &'a str, is_negative: bool) -> Result<(Number, &'a str), Error> {
    let bytes = s.as_bytes();
    let mut end = 0;
    let mut has_dot = false;

    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_digit() {
            end += 1;
        } else if b == b'.' && !has_dot {
            if end + 1 < bytes.len() && bytes[end + 1].is_ascii_digit() {
                has_dot = true;
                end += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if end == 0 {
        return Err(Error::UnexpectedToken);
    }

    let digits_str = &s[..end];
    let rest = &s[end..];

    if let Some(suffix_res) = parse_suffix(digits_str, rest, is_negative) {
        return suffix_res;
    }

    if has_dot {
        let val = f64::from_str(digits_str).map_err(|_| Error::InvalidNumber)?;
        let res = if is_negative { -val } else { val };
        Ok((Number::F64(res), rest))
    } else {
        let val = if is_negative {
            match u64::from_str(digits_str) {
                Ok(2147483648) => i32::MIN,
                Ok(v) if v < 2147483648 => -(v as i32),
                _ => return Err(Error::InvalidNumber),
            }
        } else {
            i32::from_str(digits_str).map_err(|_| Error::InvalidNumber)?
        };
        Ok((Number::I32(val), rest))
    }
}

fn parse_suffix<'a>(
    digits_str: &str,
    rest: &'a str,
    is_negative: bool,
) -> Option<Result<(Number, &'a str), Error>> {
    let suffixes = [
        ("i128", 4),
        ("u128", 4),
        ("i16", 3),
        ("i32", 3),
        ("i64", 3),
        ("u16", 3),
        ("u32", 3),
        ("u64", 3),
        ("f32", 3),
        ("f64", 3),
        ("i8", 2),
        ("u8", 2),
    ];
    for (suf, len) in suffixes {
        if rest.starts_with(suf) {
            let rem = &rest[len..];
            let res: Result<Number, Error> = match (suf, is_negative) {
                ("u8", true) => match u8::from_str(digits_str) {
                    Ok(v) => arithmetic::negate(Number::U8(v)).map_err(Error::Arithmetic),
                    Err(_) => Err(Error::InvalidNumber),
                },
                ("u16", true) => match u16::from_str(digits_str) {
                    Ok(v) => arithmetic::negate(Number::U16(v)).map_err(Error::Arithmetic),
                    Err(_) => Err(Error::InvalidNumber),
                },
                ("u32", true) => match u32::from_str(digits_str) {
                    Ok(v) => arithmetic::negate(Number::U32(v)).map_err(Error::Arithmetic),
                    Err(_) => Err(Error::InvalidNumber),
                },
                ("u64", true) => match u64::from_str(digits_str) {
                    Ok(v) => arithmetic::negate(Number::U64(v)).map_err(Error::Arithmetic),
                    Err(_) => Err(Error::InvalidNumber),
                },
                ("u128", true) => match u128::from_str(digits_str) {
                    Ok(v) => arithmetic::negate(Number::U128(v)).map_err(Error::Arithmetic),
                    Err(_) => Err(Error::InvalidNumber),
                },

                ("u8", false) => u8::from_str(digits_str)
                    .map(Number::U8)
                    .map_err(|_| Error::InvalidNumber),
                ("u16", false) => u16::from_str(digits_str)
                    .map(Number::U16)
                    .map_err(|_| Error::InvalidNumber),
                ("u32", false) => u32::from_str(digits_str)
                    .map(Number::U32)
                    .map_err(|_| Error::InvalidNumber),
                ("u64", false) => u64::from_str(digits_str)
                    .map(Number::U64)
                    .map_err(|_| Error::InvalidNumber),
                ("u128", false) => u128::from_str(digits_str)
                    .map(Number::U128)
                    .map_err(|_| Error::InvalidNumber),

                ("f32", false) => f32::from_str(digits_str)
                    .map(Number::F32)
                    .map_err(|_| Error::InvalidNumber),
                ("f32", true) => f32::from_str(digits_str)
                    .map(|v| Number::F32(-v))
                    .map_err(|_| Error::InvalidNumber),
                ("f64", false) => f64::from_str(digits_str)
                    .map(Number::F64)
                    .map_err(|_| Error::InvalidNumber),
                ("f64", true) => f64::from_str(digits_str)
                    .map(|v| Number::F64(-v))
                    .map_err(|_| Error::InvalidNumber),

                ("i8", false) => i8::from_str(digits_str)
                    .map(Number::I8)
                    .map_err(|_| Error::InvalidNumber),
                ("i16", false) => i16::from_str(digits_str)
                    .map(Number::I16)
                    .map_err(|_| Error::InvalidNumber),
                ("i32", false) => i32::from_str(digits_str)
                    .map(Number::I32)
                    .map_err(|_| Error::InvalidNumber),
                ("i64", false) => i64::from_str(digits_str)
                    .map(Number::I64)
                    .map_err(|_| Error::InvalidNumber),
                ("i128", false) => i128::from_str(digits_str)
                    .map(Number::I128)
                    .map_err(|_| Error::InvalidNumber),

                ("i8", true) => match u8::from_str(digits_str) {
                    Ok(128) => Ok(Number::I8(i8::MIN)),
                    Ok(v) if v < 128 => Ok(Number::I8(-(v as i8))),
                    _ => Err(Error::InvalidNumber),
                },
                ("i16", true) => match u16::from_str(digits_str) {
                    Ok(32768) => Ok(Number::I16(i16::MIN)),
                    Ok(v) if v < 32768 => Ok(Number::I16(-(v as i16))),
                    _ => Err(Error::InvalidNumber),
                },
                ("i32", true) => match u32::from_str(digits_str) {
                    Ok(2147483648) => Ok(Number::I32(i32::MIN)),
                    Ok(v) if v < 2147483648 => Ok(Number::I32(-(v as i32))),
                    _ => Err(Error::InvalidNumber),
                },
                ("i64", true) => match u64::from_str(digits_str) {
                    Ok(9223372036854775808) => Ok(Number::I64(i64::MIN)),
                    Ok(v) if v < 9223372036854775808 => Ok(Number::I64(-(v as i64))),
                    _ => Err(Error::InvalidNumber),
                },
                ("i128", true) => match u128::from_str(digits_str) {
                    Ok(170141183460469231731687303715884105728) => Ok(Number::I128(i128::MIN)),
                    Ok(v) if v < 170141183460469231731687303715884105728 => {
                        Ok(Number::I128(-(v as i128)))
                    }
                    _ => Err(Error::InvalidNumber),
                },

                _ => unreachable!(),
            };

            return match res {
                Ok(num) => Some(Ok((num, rem))),
                Err(err) => Some(Err(err)),
            };
        }
    }
    None
}

fn parse_identifier(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let first = bytes[0];
    if first.is_ascii_alphabetic() || first == b'_' {
        let mut end = 1;
        while end < bytes.len() {
            let b = bytes[end];
            if b.is_ascii_alphanumeric() || b == b'_' {
                end += 1;
            } else {
                break;
            }
        }
        Some((&s[..end], &s[end..]))
    } else {
        None
    }
}

fn parse_primary_or_unary<'a>(
    input: &'a str,
    bindings: &[NumberBinding<'_>],
) -> Result<(Number, &'a str), Error> {
    let trimmed = input.trim_start();
    if trimmed.is_empty() {
        return Err(Error::UnexpectedEnd);
    }

    if trimmed.starts_with('(') {
        let after_open = &trimmed[1..];
        let (val, rest) = parse_binary(after_open, 1, bindings)?;
        let trimmed_rest = rest.trim_start();
        if trimmed_rest.starts_with(')') {
            Ok((val, &trimmed_rest[1..]))
        } else {
            Err(Error::MissingClosingParenthesis)
        }
    } else if trimmed.starts_with('-') {
        let after_minus = &trimmed[1..].trim_start();
        if after_minus.is_empty() {
            return Err(Error::UnexpectedEnd);
        }
        if after_minus.as_bytes()[0].is_ascii_digit() {
            parse_number_literal(after_minus, true)
        } else {
            let (val, rest) = parse_primary_or_unary(after_minus, bindings)?;
            let neg = arithmetic::negate(val).map_err(Error::Arithmetic)?;
            Ok((neg, rest))
        }
    } else if trimmed.starts_with('+') {
        let after_plus = &trimmed[1..];
        parse_primary_or_unary(after_plus, bindings)
    } else if let Some((ident, rest)) = parse_identifier(trimmed) {
        let val = resolve_identifier(ident, bindings)?;
        Ok((val, rest))
    } else if trimmed.as_bytes()[0].is_ascii_digit() {
        parse_number_literal(trimmed, false)
    } else {
        Err(Error::UnexpectedToken)
    }
}

fn parse_binary<'a>(
    input: &'a str,
    min_prec: u8,
    bindings: &[NumberBinding<'_>],
) -> Result<(Number, &'a str), Error> {
    let (mut left, mut rest) = parse_primary_or_unary(input, bindings)?;

    loop {
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            break;
        }

        let bytes = trimmed.as_bytes();
        let op_char = bytes[0];

        let (prec, is_op) = match op_char {
            b'*' | b'/' | b'%' => (2, true),
            b'+' | b'-' => (1, true),
            _ => (0, false),
        };

        if !is_op || prec < min_prec {
            break;
        }

        let after_op = &trimmed[1..];
        let trimmed_after_op = after_op.trim_start();
        if trimmed_after_op.is_empty() {
            return Err(Error::UnexpectedEnd);
        }

        let (right, next_rest) = parse_binary(after_op, prec + 1, bindings)?;

        let res = match op_char {
            b'+' => arithmetic::add(left, right),
            b'-' => arithmetic::subtract(left, right),
            b'*' => arithmetic::multiply(left, right),
            b'/' => arithmetic::divide(left, right),
            b'%' => arithmetic::remainder(left, right),
            _ => unreachable!(),
        }
        .map_err(Error::Arithmetic)?;

        left = res;
        rest = next_rest;
    }

    Ok((left, rest))
}

pub fn evaluate(source: &str, bindings: &[NumberBinding<'_>]) -> Result<Number, Error> {
    let trimmed = source.trim_start();
    if trimmed.is_empty() {
        return Err(Error::EmptyExpression);
    }
    let (val, rest) = parse_binary(trimmed, 1, bindings)?;
    let final_rest = rest.trim_start();
    if !final_rest.is_empty() {
        return Err(Error::UnexpectedToken);
    }
    Ok(val)
}

pub fn evaluate_static(source: &str) -> Result<Number, Error> {
    evaluate(source, &[])
}
