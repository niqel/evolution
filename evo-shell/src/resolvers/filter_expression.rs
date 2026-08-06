use std::ffi::OsString;

use evo_shell_engine::{
    FilesystemEntryKind, FilterComparison, FilterExpression, FilterOperand, FilterOperator,
    FilterProperty, FilterValue,
};

use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;

pub fn resolve<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<FilterExpression, ParseError<'a>> {
    let expression = parse_expression(stream, tokenize, false)?;
    if matches!(peek(stream, tokenize)?, Some(Token::RightParen)) {
        return Err(ParseError::UnexpectedClosingParenthesis);
    }
    Ok(expression)
}

fn parse_expression<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    allow_closing_paren: bool,
) -> Result<FilterExpression, ParseError<'a>> {
    let first = parse_term(stream, tokenize, allow_closing_paren)?;
    let mut expressions = vec![first];
    let mut logical_operator: Option<LogicalOperator> = None;

    loop {
        let lookahead = peek(stream, tokenize)?;
        let Some(lookahead) = lookahead else {
            break;
        };

        match lookahead {
            Token::PipelineSeparator => break,
            Token::RightParen if allow_closing_paren => break,
            Token::RightParen => return Err(ParseError::UnexpectedClosingParenthesis),
            Token::Word("and") => {
                let _ = next_token(stream, tokenize)?;
                logical_operator = select_logical_operator(logical_operator, LogicalOperator::And)?;
                let expression = parse_term(stream, tokenize, allow_closing_paren)?;
                push_logical_expression(&mut expressions, logical_operator.unwrap(), expression);
            }
            Token::Word("or") => {
                let _ = next_token(stream, tokenize)?;
                logical_operator = select_logical_operator(logical_operator, LogicalOperator::Or)?;
                let expression = parse_term(stream, tokenize, allow_closing_paren)?;
                push_logical_expression(&mut expressions, logical_operator.unwrap(), expression);
            }
            _ => return Err(ParseError::UnexpectedFilterToken(lookahead)),
        }
    }

    Ok(match logical_operator {
        None => expressions.into_iter().next().unwrap(),
        Some(LogicalOperator::And) => FilterExpression::and(expressions),
        Some(LogicalOperator::Or) => FilterExpression::or(expressions),
    })
}

fn parse_term<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    allow_closing_paren: bool,
) -> Result<FilterExpression, ParseError<'a>> {
    let token = peek(stream, tokenize)?;

    let Some(token) = token else {
        return Err(ParseError::EmptyFilterExpression);
    };

    match token {
        Token::LeftParen => parse_group(stream, tokenize),
        Token::RightParen if allow_closing_paren => Err(ParseError::UnexpectedClosingParenthesis),
        Token::RightParen => Err(ParseError::UnexpectedClosingParenthesis),
        Token::PipelineSeparator => Err(ParseError::EmptyFilterExpression),
        _ => parse_comparison(stream, tokenize),
    }
}

fn parse_group<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<FilterExpression, ParseError<'a>> {
    consume_specific(stream, tokenize, Token::LeftParen)?;

    if matches!(peek(stream, tokenize)?, Some(Token::RightParen)) {
        return Err(ParseError::EmptyFilterExpression);
    }

    let expression = parse_expression(stream, tokenize, true)?;
    consume_group_close(stream, tokenize)?;
    Ok(expression)
}

fn parse_comparison<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<FilterExpression, ParseError<'a>> {
    let property_token = next_word_token(stream, tokenize)?;
    let property = parse_property(property_token)?;

    let operator_token = next_filter_token(stream, tokenize, property_token)?;
    let operator = parse_operator(operator_token)?;
    let operator_text = token_text(operator_token);

    let comparison = match operator {
        FilterOperator::Between | FilterOperator::NotBetween => {
            let lower = parse_filter_value(stream, tokenize, &property, operator_text)?;
            consume_comma(stream, tokenize, operator_text)?;
            let upper = parse_filter_value(stream, tokenize, &property, operator_text)?;
            FilterComparison::new(property, operator, FilterOperand::range(lower, upper))
        }
        _ => {
            let operand = parse_filter_value(stream, tokenize, &property, operator_text)?;
            FilterComparison::new(property, operator, FilterOperand::single(operand))
        }
    };

    Ok(FilterExpression::comparison(comparison))
}

fn parse_property<'a>(property: &'a str) -> Result<FilterProperty, ParseError<'a>> {
    match property {
        "index" => Ok(FilterProperty::Index),
        "created" => Ok(FilterProperty::Created),
        "modified" => Ok(FilterProperty::Modified),
        "type" => Ok(FilterProperty::Type),
        "size" => Ok(FilterProperty::Size),
        "name" => Ok(FilterProperty::Name),
        _ => Err(ParseError::UnknownFilterProperty(property)),
    }
}

fn parse_operator<'a>(operator: Token<'a>) -> Result<FilterOperator, ParseError<'a>> {
    match operator {
        Token::Word("equals") => Ok(FilterOperator::Equals),
        Token::Word("not-equals") => Ok(FilterOperator::NotEquals),
        Token::Word(">") => Ok(FilterOperator::GreaterThan),
        Token::Word("<") => Ok(FilterOperator::LessThan),
        Token::Word("at-least") => Ok(FilterOperator::AtLeast),
        Token::Word("at-most") => Ok(FilterOperator::AtMost),
        Token::Word("between") => Ok(FilterOperator::Between),
        Token::Word("not-between") => Ok(FilterOperator::NotBetween),
        Token::Word("and") | Token::Word("or") => Err(ParseError::MissingFilterOperator("and")),
        Token::Word(other) => Err(ParseError::UnknownFilterOperator(other)),
        Token::String(other) => Err(ParseError::UnknownFilterOperator(other)),
        Token::LeftParen | Token::RightParen | Token::Comma | Token::PipelineSeparator => {
            Err(ParseError::MissingFilterOperator("operator"))
        }
    }
}

fn parse_filter_value<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    property: &FilterProperty,
    operator: &'a str,
) -> Result<FilterValue, ParseError<'a>> {
    let token = next_filter_token(stream, tokenize, operator)?;

    match property {
        FilterProperty::Index => parse_usize_value(token).map(FilterValue::Index),
        FilterProperty::Size => parse_size_value(token).map(FilterValue::Size),
        FilterProperty::Type => parse_kind_value(token).map(FilterValue::Type),
        FilterProperty::Name => parse_name_value(token).map(FilterValue::Name),
        FilterProperty::Created | FilterProperty::Modified => {
            Err(ParseError::UnsupportedFilterValue(property_text(property)))
        }
        FilterProperty::Unsupported(_) => {
            Err(ParseError::UnknownFilterProperty(property_text(property)))
        }
    }
}

fn parse_usize_value(token: Token<'_>) -> Result<usize, ParseError<'_>> {
    match token {
        Token::Word(value) | Token::String(value) => value
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFilterValue(value)),
        _ => Err(ParseError::InvalidFilterValue(token_text(token))),
    }
}

fn parse_size_value(token: Token<'_>) -> Result<u64, ParseError<'_>> {
    match token {
        Token::Word(value) | Token::String(value) => {
            parse_size_literal(value).map_err(|_| ParseError::InvalidFilterValue(value))
        }
        _ => Err(ParseError::InvalidFilterValue(token_text(token))),
    }
}

fn parse_kind_value(token: Token<'_>) -> Result<FilesystemEntryKind, ParseError<'_>> {
    let value = match token {
        Token::Word(value) | Token::String(value) => value,
        _ => return Err(ParseError::InvalidFilterValue(token_text(token))),
    };

    match value.to_ascii_lowercase().as_str() {
        "file" => Ok(FilesystemEntryKind::File),
        "directory" => Ok(FilesystemEntryKind::Directory),
        "symlink" => Ok(FilesystemEntryKind::Symlink),
        "other" => Ok(FilesystemEntryKind::Other),
        _ => Err(ParseError::InvalidFilterValue(value)),
    }
}

fn parse_name_value(token: Token<'_>) -> Result<OsString, ParseError<'_>> {
    match token {
        Token::Word(value) | Token::String(value) => Ok(OsString::from(value)),
        _ => Err(ParseError::InvalidFilterValue(token_text(token))),
    }
}

fn parse_size_literal(value: &str) -> Result<u64, ()> {
    let split_at = value
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(value.len());
    let (number, suffix) = value.split_at(split_at);

    if number.is_empty() {
        return Err(());
    }

    let bytes = number.parse::<u64>().map_err(|_| ())?;
    let suffix = suffix.to_ascii_lowercase();
    let multiplier = match suffix.as_str() {
        "" | "b" => 1,
        "kb" => 1024,
        "mb" => 1024_u64.saturating_mul(1024),
        "gb" => 1024_u64.saturating_mul(1024).saturating_mul(1024),
        _ => return Err(()),
    };

    bytes.checked_mul(multiplier).ok_or(())
}

fn consume_specific<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    expected: Token<'a>,
) -> Result<(), ParseError<'a>> {
    let token = next_token(stream, tokenize)?;

    if token == Some(expected) {
        return Ok(());
    }

    match token {
        Some(token) => Err(ParseError::UnexpectedFilterToken(token)),
        None => Err(ParseError::EmptyFilterExpression),
    }
}

fn consume_group_close<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<(), ParseError<'a>> {
    match next_token(stream, tokenize)? {
        Some(Token::RightParen) => Ok(()),
        Some(token) => Err(ParseError::UnexpectedFilterToken(token)),
        None => Err(ParseError::UnclosedParenthesis),
    }
}

fn consume_comma<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    operator: &'a str,
) -> Result<(), ParseError<'a>> {
    match next_token(stream, tokenize)? {
        Some(Token::Comma) => Ok(()),
        Some(Token::PipelineSeparator) | None => {
            Err(ParseError::MissingBetweenUpperBound(operator))
        }
        Some(Token::RightParen) => Err(ParseError::UnclosedParenthesis),
        Some(token) => Err(ParseError::UnexpectedFilterToken(token)),
    }
}

fn next_word_token<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<&'a str, ParseError<'a>> {
    match next_token(stream, tokenize)? {
        Some(Token::Word(word)) => Ok(word),
        Some(Token::String(word)) => Err(ParseError::UnknownFilterProperty(word)),
        Some(Token::PipelineSeparator) | None => Err(ParseError::EmptyFilterExpression),
        Some(Token::RightParen) => Err(ParseError::UnexpectedClosingParenthesis),
        Some(token) => Err(ParseError::UnexpectedFilterToken(token)),
    }
}

fn next_filter_token<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    context: &'a str,
) -> Result<Token<'a>, ParseError<'a>> {
    match next_token(stream, tokenize)? {
        Some(Token::PipelineSeparator) | None => Err(ParseError::MissingFilterValue(context)),
        Some(Token::RightParen) => Err(ParseError::UnclosedParenthesis),
        Some(token) => Ok(token),
    }
}

fn select_logical_operator<'a>(
    current: Option<LogicalOperator>,
    next: LogicalOperator,
) -> Result<Option<LogicalOperator>, ParseError<'a>> {
    match current {
        None => Ok(Some(next)),
        Some(existing) if existing == next => Ok(Some(existing)),
        Some(_) => Err(ParseError::AmbiguousLogicalExpression),
    }
}

fn push_logical_expression(
    expressions: &mut Vec<FilterExpression>,
    logical_operator: LogicalOperator,
    expression: FilterExpression,
) {
    match logical_operator {
        LogicalOperator::And => match expression {
            FilterExpression::And(mut nested) => expressions.append(&mut nested),
            expression => expressions.push(expression),
        },
        LogicalOperator::Or => match expression {
            FilterExpression::Or(mut nested) => expressions.append(&mut nested),
            expression => expressions.push(expression),
        },
    }
}

fn property_text(property: &FilterProperty) -> &'static str {
    match property {
        FilterProperty::Index => "index",
        FilterProperty::Created => "created",
        FilterProperty::Modified => "modified",
        FilterProperty::Type => "type",
        FilterProperty::Size => "size",
        FilterProperty::Name => "name",
        FilterProperty::Unsupported(_) => "unsupported",
    }
}

fn token_text<'a>(token: Token<'a>) -> &'a str {
    match token {
        Token::Word(value) | Token::String(value) => value,
        Token::PipelineSeparator => "|>",
        Token::Comma => ",",
        Token::LeftParen => "(",
        Token::RightParen => ")",
    }
}

fn next_token<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Option<Token<'a>>, ParseError<'a>> {
    tokenize(stream).map_err(ParseError::Tokenize)
}

fn peek<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Option<Token<'a>>, ParseError<'a>> {
    let position = stream.position();
    let token = next_token(stream, tokenize)?;
    stream.advance_to(position);
    Ok(token)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogicalOperator {
    And,
    Or,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::definitions::domain::entities::token_stream::TokenStream;
    use crate::tokenizer;
    use evo_shell_engine::{
        FilesystemEntryKind, FilterComparison, FilterExpression, FilterOperand, FilterOperator,
        FilterProperty, FilterValue,
    };

    fn resolve_expression(input: &str) -> Result<FilterExpression, ParseError<'_>> {
        let mut stream = TokenStream::new(input);
        resolve(&mut stream, tokenizer::tokenize)
    }

    fn comparison(
        property: FilterProperty,
        operator: FilterOperator,
        operand: FilterOperand,
    ) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(property, operator, operand))
    }

    #[test]
    fn parses_equals_expression() {
        let expression = resolve_expression(r#"name equals "file.txt""#).unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Name,
                FilterOperator::Equals,
                FilterOperand::single(FilterValue::name("file.txt"))
            )
        );
    }

    #[test]
    fn parses_not_equals_expression() {
        let expression = resolve_expression(r#"type not-equals "file""#).unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Type,
                FilterOperator::NotEquals,
                FilterOperand::single(FilterValue::kind(FilesystemEntryKind::File))
            )
        );
    }

    #[test]
    fn parses_numeric_comparison_expression() {
        let expression = resolve_expression("index < 10").unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Index,
                FilterOperator::LessThan,
                FilterOperand::single(FilterValue::index(10))
            )
        );
    }

    #[test]
    fn parses_size_comparison_expression() {
        let expression = resolve_expression("size > 1000").unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Size,
                FilterOperator::GreaterThan,
                FilterOperand::single(FilterValue::size(1000))
            )
        );
    }

    #[test]
    fn parses_between_expression() {
        let expression = resolve_expression("size between 10kb, 100kb").unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Size,
                FilterOperator::Between,
                FilterOperand::range(FilterValue::size(10 * 1024), FilterValue::size(100 * 1024))
            )
        );
    }

    #[test]
    fn parses_not_between_expression() {
        let expression = resolve_expression("size not-between 10kb, 100kb").unwrap();

        assert_eq!(
            expression,
            comparison(
                FilterProperty::Size,
                FilterOperator::NotBetween,
                FilterOperand::range(FilterValue::size(10 * 1024), FilterValue::size(100 * 1024))
            )
        );
    }

    #[test]
    fn parses_and_chain() {
        let expression =
            resolve_expression(r#"type equals "file" and size > 10kb and name equals "README.md""#)
                .unwrap();

        assert_eq!(
            expression,
            FilterExpression::and(vec![
                comparison(
                    FilterProperty::Type,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::kind(FilesystemEntryKind::File))
                ),
                comparison(
                    FilterProperty::Size,
                    FilterOperator::GreaterThan,
                    FilterOperand::single(FilterValue::size(10 * 1024))
                ),
                comparison(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("README.md"))
                ),
            ])
        );
    }

    #[test]
    fn parses_or_chain() {
        let expression = resolve_expression(
            r#"name equals "README.md" or name equals "LICENSE" or name equals "CHANGELOG.md""#,
        )
        .unwrap();

        assert_eq!(
            expression,
            FilterExpression::or(vec![
                comparison(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("README.md"))
                ),
                comparison(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("LICENSE"))
                ),
                comparison(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("CHANGELOG.md"))
                ),
            ])
        );
    }

    #[test]
    fn rejects_mixed_and_or_without_parentheses() {
        let result = resolve_expression("name equals \"a\" or name equals \"b\" and size > 10");

        assert!(matches!(
            result,
            Err(ParseError::AmbiguousLogicalExpression)
        ));
    }

    #[test]
    fn parses_parenthesized_or_then_and() {
        let expression =
            resolve_expression(r#"(name equals "a" or name equals "b") and size > 10"#).unwrap();

        assert_eq!(
            expression,
            FilterExpression::and(vec![
                FilterExpression::or(vec![
                    comparison(
                        FilterProperty::Name,
                        FilterOperator::Equals,
                        FilterOperand::single(FilterValue::name("a"))
                    ),
                    comparison(
                        FilterProperty::Name,
                        FilterOperator::Equals,
                        FilterOperand::single(FilterValue::name("b"))
                    ),
                ]),
                comparison(
                    FilterProperty::Size,
                    FilterOperator::GreaterThan,
                    FilterOperand::single(FilterValue::size(10))
                ),
            ])
        );
    }

    #[test]
    fn parses_parenthesized_and_then_or() {
        let expression =
            resolve_expression(r#"name equals "a" or (name equals "b" and size > 10)"#).unwrap();

        assert_eq!(
            expression,
            FilterExpression::or(vec![
                comparison(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("a"))
                ),
                FilterExpression::and(vec![
                    comparison(
                        FilterProperty::Name,
                        FilterOperator::Equals,
                        FilterOperand::single(FilterValue::name("b"))
                    ),
                    comparison(
                        FilterProperty::Size,
                        FilterOperator::GreaterThan,
                        FilterOperand::single(FilterValue::size(10))
                    ),
                ]),
            ])
        );
    }

    #[test]
    fn rejects_unknown_property() {
        let result = resolve_expression(r#"foo equals "bar""#);

        assert!(matches!(
            result,
            Err(ParseError::UnknownFilterProperty("foo"))
        ));
    }

    #[test]
    fn rejects_unknown_operator() {
        let result = resolve_expression(r#"name eq "bar""#);

        assert!(matches!(
            result,
            Err(ParseError::UnknownFilterOperator("eq"))
        ));
    }

    #[test]
    fn rejects_equals_symbol() {
        let result = resolve_expression(r#"name = "bar""#);

        assert!(matches!(
            result,
            Err(ParseError::UnknownFilterOperator("="))
        ));
    }

    #[test]
    fn rejects_not_equals_symbol() {
        let result = resolve_expression(r#"name != "bar""#);

        assert!(matches!(
            result,
            Err(ParseError::UnknownFilterOperator("!="))
        ));
    }

    #[test]
    fn rejects_greater_equal_symbol() {
        let result = resolve_expression(r#"size >= 10"#);

        assert!(matches!(result, Err(ParseError::InvalidFilterValue("="))));
    }

    #[test]
    fn rejects_less_equal_symbol() {
        let result = resolve_expression(r#"size <= 10"#);

        assert!(matches!(result, Err(ParseError::InvalidFilterValue("="))));
    }

    #[test]
    fn rejects_missing_expression() {
        let result = resolve_expression("");

        assert!(matches!(result, Err(ParseError::EmptyFilterExpression)));
    }

    #[test]
    fn rejects_empty_group() {
        let result = resolve_expression("()");

        assert!(matches!(result, Err(ParseError::EmptyFilterExpression)));
    }

    #[test]
    fn rejects_unclosed_group() {
        let result = resolve_expression("(name equals \"a\"");

        assert!(matches!(result, Err(ParseError::UnclosedParenthesis)));
    }

    #[test]
    fn rejects_unexpected_closing_paren() {
        let result = resolve_expression("name equals \"a\")");

        assert!(matches!(
            result,
            Err(ParseError::UnexpectedClosingParenthesis)
        ));
    }

    #[test]
    fn rejects_missing_operator_after_property() {
        let result = resolve_expression("name");

        assert!(matches!(
            result,
            Err(ParseError::MissingFilterValue("name"))
        ));
    }

    #[test]
    fn rejects_missing_value_after_operator() {
        let result = resolve_expression("name equals");

        assert!(matches!(
            result,
            Err(ParseError::MissingFilterValue("equals"))
        ));
    }

    #[test]
    fn rejects_missing_between_upper_bound() {
        let result = resolve_expression("size between 10kb");

        assert!(matches!(
            result,
            Err(ParseError::MissingBetweenUpperBound("between"))
        ));
    }

    #[test]
    fn rejects_invalid_numeric_value() {
        let result = resolve_expression("index < hello");

        assert!(matches!(
            result,
            Err(ParseError::InvalidFilterValue("hello"))
        ));
    }

    #[test]
    fn rejects_invalid_size_value() {
        let result = resolve_expression("size > not-a-size");

        assert!(matches!(
            result,
            Err(ParseError::InvalidFilterValue("not-a-size"))
        ));
    }

    #[test]
    fn rejects_created_value_lacking_textual_contract() {
        let result = resolve_expression("created equals \"2026-08-06\"");

        assert!(matches!(
            result,
            Err(ParseError::UnsupportedFilterValue("created"))
        ));
    }

    #[test]
    fn filter_expression_accepts_symbolic_comparisons() {
        let less = resolve_expression("index < 10").unwrap();
        let greater = resolve_expression("size > 10").unwrap();

        assert!(matches!(less, FilterExpression::Comparison(_)));
        assert!(matches!(greater, FilterExpression::Comparison(_)));
    }
}
