use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;
use core::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Arithmetic(arithmetic::Error),
    VariableNotFound,
    InvalidSyntax,
    UnmatchedParenthesis,
    EmptyExpression,
}

pub type ResolveVariable = for<'name> fn(&'name str) -> Result<Number, Error>;

pub fn resolve_no_variables(_name: &str) -> Result<Number, Error> {
    Err(Error::VariableNotFound)
}

fn parse_number_literal<'a>(s: &'a str) -> Result<(Number, &'a str), Error> {
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
        return Err(Error::InvalidSyntax);
    }

    let num_str = &s[..end];
    let rest = &s[end..];

    if let Some(res) = parse_suffix(num_str, rest) {
        return res;
    }

    if has_dot {
        let val = f64::from_str(num_str).map_err(|_| Error::InvalidSyntax)?;
        Ok((Number::F64(val), rest))
    } else {
        let val = i32::from_str(num_str).map_err(|_| Error::InvalidSyntax)?;
        Ok((Number::I32(val), rest))
    }
}

fn parse_suffix<'a>(num_str: &str, rest: &'a str) -> Option<Result<(Number, &'a str), Error>> {
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
            let res = match suf {
                "i8" => i8::from_str(num_str).ok().map(Number::I8),
                "i16" => i16::from_str(num_str).ok().map(Number::I16),
                "i32" => i32::from_str(num_str).ok().map(Number::I32),
                "i64" => i64::from_str(num_str).ok().map(Number::I64),
                "i128" => i128::from_str(num_str).ok().map(Number::I128),
                "u8" => u8::from_str(num_str).ok().map(Number::U8),
                "u16" => u16::from_str(num_str).ok().map(Number::U16),
                "u32" => u32::from_str(num_str).ok().map(Number::U32),
                "u64" => u64::from_str(num_str).ok().map(Number::U64),
                "u128" => u128::from_str(num_str).ok().map(Number::U128),
                "f32" => f32::from_str(num_str).ok().map(Number::F32),
                "f64" => f64::from_str(num_str).ok().map(Number::F64),
                _ => return None,
            };
            return match res {
                Some(num) => Some(Ok((num, rem))),
                None => Some(Err(Error::InvalidSyntax)),
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
    resolve_var: ResolveVariable,
) -> Result<(Number, &'a str), Error> {
    let trimmed = input.trim_start();
    if trimmed.is_empty() {
        return Err(Error::EmptyExpression);
    }

    if trimmed.starts_with('(') {
        let after_open = &trimmed[1..];
        let (val, rest) = parse_binary(after_open, 1, resolve_var)?;
        let trimmed_rest = rest.trim_start();
        if trimmed_rest.starts_with(')') {
            Ok((val, &trimmed_rest[1..]))
        } else {
            Err(Error::UnmatchedParenthesis)
        }
    } else if trimmed.starts_with('-') {
        let after_minus = &trimmed[1..];
        let (val, rest) = parse_primary_or_unary(after_minus, resolve_var)?;
        let neg = arithmetic::negate(val).map_err(Error::Arithmetic)?;
        Ok((neg, rest))
    } else if trimmed.starts_with('+') {
        let after_plus = &trimmed[1..];
        parse_primary_or_unary(after_plus, resolve_var)
    } else if let Some((ident, rest)) = parse_identifier(trimmed) {
        let val = resolve_var(ident)?;
        Ok((val, rest))
    } else if trimmed.as_bytes()[0].is_ascii_digit() {
        parse_number_literal(trimmed)
    } else {
        Err(Error::InvalidSyntax)
    }
}

fn parse_binary<'a>(
    input: &'a str,
    min_prec: u8,
    resolve_var: ResolveVariable,
) -> Result<(Number, &'a str), Error> {
    let (mut left, mut rest) = parse_primary_or_unary(input, resolve_var)?;

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
        let (right, next_rest) = parse_binary(after_op, prec + 1, resolve_var)?;

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

pub fn evaluate(source: &str, resolve_var: ResolveVariable) -> Result<Number, Error> {
    let trimmed = source.trim_start();
    if trimmed.is_empty() {
        return Err(Error::EmptyExpression);
    }
    let (val, rest) = parse_binary(trimmed, 1, resolve_var)?;
    let final_rest = rest.trim_start();
    if !final_rest.is_empty() {
        if final_rest.starts_with(')') {
            return Err(Error::UnmatchedParenthesis);
        }
        return Err(Error::InvalidSyntax);
    }
    Ok(val)
}

pub fn evaluate_static(source: &str) -> Result<Number, Error> {
    evaluate(source, resolve_no_variables)
}
