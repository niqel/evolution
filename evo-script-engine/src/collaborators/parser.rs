use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::data::ast::expressions::{
    BinaryOperator, EnumConstruction, Expression, ExpressionKind, FieldInitializer, FunctionCall,
    LiteralKind, Pipeline, PipelineStage, UnaryOperator,
};
use crate::data::ast::foundational::{
    Identifier, ImportDeclaration, QualifiedName, TypedBinding, Visibility,
};
use crate::data::ast::functions::{
    BodyStatement, FunctionBody, FunctionDefinition, LetBinding, OperationStatement, Parameter,
};
use crate::data::ast::local_types::{
    EnumDefinition, EnumVariant, FieldDefinition, StructDefinition,
};
use crate::data::ast::program::{Declaration, Program};
use crate::data::ast::when::{PatternField, WhenCorrespondence, WhenExpression, WhenPattern};
use crate::data::failures::{CompileFailure, CompileFailureKind, SyntaxFailure};
use crate::data::lexical::{SourceSpan, Token, TokenKind, TokenSequence};

pub type Parse = for<'source> fn(
    &TokenSequence<'source>,
    &'source str,
) -> Result<Program<'source>, CompileFailure>;

pub fn parse_tokens<'source>(
    tokens: &TokenSequence<'source>,
    source: &'source str,
) -> Result<Program<'source>, CompileFailure> {
    let mut parser = Parser::new(tokens, source);
    parser.parse_program()
}

pub const PARSE_TOKENS: Parse = parse_tokens;

struct Parser<'a, 'source> {
    tokens: &'a [Token<'source>],
    cursor: usize,
    source: &'source str,
    in_when_subject: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OpKind {
    Comparison,
    LogicalAnd,
    LogicalOr,
    Other,
}

struct ExprInfo<'source> {
    expr: Expression<'source>,
    is_grouped: bool,
    op_kind: Option<OpKind>,
}

impl<'a, 'source> Parser<'a, 'source> {
    fn new(tokens: &'a [Token<'source>], source: &'source str) -> Self {
        Self {
            tokens,
            cursor: 0,
            source,
            in_when_subject: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Option<Token<'source>> {
        self.tokens.get(self.cursor).copied()
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind)
    }

    fn peek_nth(&self, n: usize) -> Option<Token<'source>> {
        self.tokens.get(self.cursor + n).copied()
    }

    fn peek_nth_kind(&self, n: usize) -> Option<TokenKind> {
        self.peek_nth(n).map(|t| t.kind)
    }

    fn advance(&mut self) -> Option<Token<'source>> {
        if !self.is_at_end() {
            let token = self.tokens[self.cursor];
            self.cursor += 1;
            Some(token)
        } else {
            None
        }
    }

    fn current_span(&self) -> SourceSpan {
        if let Some(token) = self.peek() {
            token.span
        } else {
            SourceSpan {
                start: self.source.len(),
                end: self.source.len(),
            }
        }
    }

    fn failure<T>(&self, kind: SyntaxFailure, span: SourceSpan) -> Result<T, CompileFailure> {
        Err(CompileFailure {
            kind: CompileFailureKind::Syntax(kind),
            source_span: span,
        })
    }

    // --- Contextual Expectations for Declarations ---
    fn expect_declaration_token(
        &mut self,
        expected: TokenKind,
    ) -> Result<Token<'source>, CompileFailure> {
        if let Some(token) = self.peek() {
            if token.kind == expected {
                Ok(self.advance().unwrap())
            } else {
                self.failure(SyntaxFailure::MalformedDeclaration, token.span)
            }
        } else {
            self.failure(SyntaxFailure::MalformedDeclaration, self.current_span())
        }
    }

    fn expect_declaration_identifier(&mut self) -> Result<Identifier<'source>, CompileFailure> {
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::Identifier {
                let tok = self.advance().unwrap();
                Ok(Identifier {
                    lexeme: tok.lexeme,
                    span: tok.span,
                })
            } else {
                self.failure(SyntaxFailure::MalformedDeclaration, token.span)
            }
        } else {
            self.failure(SyntaxFailure::MalformedDeclaration, self.current_span())
        }
    }

    fn parse_declaration_qualified_name(
        &mut self,
    ) -> Result<QualifiedName<'source>, CompileFailure> {
        let qualifier = self.expect_declaration_identifier()?;
        self.expect_declaration_token(TokenKind::Qualification)?;
        let name = self.expect_declaration_identifier()?;
        Ok(QualifiedName { qualifier, name })
    }

    fn parse_declaration_typed_binding(&mut self) -> Result<TypedBinding<'source>, CompileFailure> {
        let type_name = self.expect_declaration_identifier()?;
        let name = self.expect_declaration_identifier()?;
        Ok(TypedBinding { type_name, name })
    }

    // --- Contextual Expectations for Expressions ---
    fn expect_expression_token(
        &mut self,
        expected: TokenKind,
    ) -> Result<Token<'source>, CompileFailure> {
        if let Some(token) = self.peek() {
            if token.kind == expected {
                Ok(self.advance().unwrap())
            } else {
                self.failure(SyntaxFailure::MalformedExpression, token.span)
            }
        } else {
            self.failure(SyntaxFailure::MalformedExpression, self.current_span())
        }
    }

    fn expect_expression_identifier(&mut self) -> Result<Identifier<'source>, CompileFailure> {
        if let Some(token) = self.peek() {
            if token.kind == TokenKind::Identifier {
                let tok = self.advance().unwrap();
                Ok(Identifier {
                    lexeme: tok.lexeme,
                    span: tok.span,
                })
            } else {
                self.failure(SyntaxFailure::MalformedExpression, token.span)
            }
        } else {
            self.failure(SyntaxFailure::MalformedExpression, self.current_span())
        }
    }

    fn parse_expression_qualified_name(
        &mut self,
    ) -> Result<QualifiedName<'source>, CompileFailure> {
        let qualifier = self.expect_expression_identifier()?;
        self.expect_expression_token(TokenKind::Qualification)?;
        let name = self.expect_expression_identifier()?;
        Ok(QualifiedName { qualifier, name })
    }

    // --- Program Parsing ---
    fn parse_program(&mut self) -> Result<Program<'source>, CompileFailure> {
        let mut imports = Vec::new();
        let mut declarations = Vec::new();
        let mut public_function_count = 0;

        // 1. Imports at the beginning
        while let Some(tok) = self.peek() {
            if tok.kind == TokenKind::Import {
                self.advance();
                let symbol = self.parse_declaration_qualified_name()?;
                let alias = if self.peek_kind() == Some(TokenKind::As) {
                    self.advance();
                    Some(self.expect_declaration_identifier()?)
                } else {
                    None
                };
                self.expect_declaration_token(TokenKind::Semicolon)?;
                imports.push(ImportDeclaration { symbol, alias });
            } else {
                break;
            }
        }

        // 2. Declarations
        while !self.is_at_end() {
            let next_kind = self.peek_kind().unwrap();
            match next_kind {
                TokenKind::Import => {
                    let import_span = self.peek().unwrap().span;
                    return self.failure(SyntaxFailure::InvalidImportPlacement, import_span);
                }
                TokenKind::Struct => {
                    let struct_def = self.parse_struct_definition()?;
                    declarations.push(Declaration::Struct(struct_def));
                }
                TokenKind::Enum => {
                    let enum_def = self.parse_enum_definition()?;
                    declarations.push(Declaration::Enum(enum_def));
                }
                TokenKind::Public | TokenKind::Private => {
                    let vis_tok = self.peek().unwrap();
                    let vis_span = vis_tok.span;
                    let func_def = self.parse_function_definition()?;
                    match func_def.visibility {
                        Visibility::Public => {
                            public_function_count += 1;
                            if public_function_count > 1 {
                                return self
                                    .failure(SyntaxFailure::MultiplePublicFunctions, vis_span);
                            }
                        }
                        Visibility::Private => {}
                    }
                    declarations.push(Declaration::Function(func_def));
                }
                _ => {
                    let span = self.peek().unwrap().span;
                    return self.failure(SyntaxFailure::MalformedDeclaration, span);
                }
            }
        }

        if public_function_count == 0 {
            let eof_span = SourceSpan {
                start: self.source.len(),
                end: self.source.len(),
            };
            return self.failure(SyntaxFailure::MissingPublicFunction, eof_span);
        }

        Ok(Program {
            imports,
            declarations,
        })
    }

    // --- Type Definitions ---
    fn parse_struct_definition(&mut self) -> Result<StructDefinition<'source>, CompileFailure> {
        self.expect_declaration_token(TokenKind::Struct)?;
        let name = self.expect_declaration_identifier()?;
        self.expect_declaration_token(TokenKind::LeftBrace)?;

        let mut fields = Vec::new();
        while self.peek_kind() != Some(TokenKind::RightBrace) {
            if self.is_at_end() {
                return self.failure(SyntaxFailure::MalformedDeclaration, self.current_span());
            }
            let type_name = self.expect_declaration_identifier()?;
            let field_name = self.expect_declaration_identifier()?;
            self.expect_declaration_token(TokenKind::Semicolon)?;
            fields.push(FieldDefinition {
                type_name,
                name: field_name,
            });
        }
        self.expect_declaration_token(TokenKind::RightBrace)?;

        Ok(StructDefinition { name, fields })
    }

    fn parse_enum_definition(&mut self) -> Result<EnumDefinition<'source>, CompileFailure> {
        self.expect_declaration_token(TokenKind::Enum)?;
        let name = self.expect_declaration_identifier()?;
        self.expect_declaration_token(TokenKind::LeftBrace)?;

        if self.peek_kind() == Some(TokenKind::RightBrace) {
            let close_span = self.peek().unwrap().span;
            return self.failure(SyntaxFailure::EmptyEnum, close_span);
        }

        let mut variants = Vec::new();
        while self.peek_kind() != Some(TokenKind::RightBrace) {
            if self.is_at_end() {
                return self.failure(SyntaxFailure::MalformedDeclaration, self.current_span());
            }
            let var_name = self.expect_declaration_identifier()?;
            if self.peek_kind() == Some(TokenKind::LeftParenthesis) {
                self.advance();
                let type_name = self.expect_declaration_identifier()?;
                self.expect_declaration_token(TokenKind::RightParenthesis)?;
                variants.push(EnumVariant::Associated {
                    name: var_name,
                    type_name,
                });
            } else if self.peek_kind() == Some(TokenKind::LeftBrace) {
                self.advance();
                let mut fields = Vec::new();
                while self.peek_kind() != Some(TokenKind::RightBrace) {
                    if self.is_at_end() {
                        return self
                            .failure(SyntaxFailure::MalformedDeclaration, self.current_span());
                    }
                    let type_name = self.expect_declaration_identifier()?;
                    let field_name = self.expect_declaration_identifier()?;
                    self.expect_declaration_token(TokenKind::Semicolon)?;
                    fields.push(FieldDefinition {
                        type_name,
                        name: field_name,
                    });
                }
                self.expect_declaration_token(TokenKind::RightBrace)?;
                variants.push(EnumVariant::Structured {
                    name: var_name,
                    fields,
                });
            } else {
                variants.push(EnumVariant::Simple { name: var_name });
            }

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect_declaration_token(TokenKind::RightBrace)?;

        Ok(EnumDefinition { name, variants })
    }

    // --- Function Definition ---
    fn parse_function_definition(&mut self) -> Result<FunctionDefinition<'source>, CompileFailure> {
        let visibility = match self.peek_kind() {
            Some(TokenKind::Public) => {
                self.advance();
                Visibility::Public
            }
            Some(TokenKind::Private) => {
                self.advance();
                Visibility::Private
            }
            _ => {
                return self.failure(SyntaxFailure::MalformedDeclaration, self.current_span());
            }
        };

        self.expect_declaration_token(TokenKind::Fn)?;
        let name = self.expect_declaration_identifier()?;
        self.expect_declaration_token(TokenKind::LeftParenthesis)?;

        let mut parameters = Vec::new();
        while self.peek_kind() != Some(TokenKind::RightParenthesis) {
            if self.is_at_end() {
                return self.failure(SyntaxFailure::MalformedDeclaration, self.current_span());
            }

            if self.peek_nth_kind(1) == Some(TokenKind::Qualification) {
                let signature = self.parse_declaration_qualified_name()?;
                let param_name = self.expect_declaration_identifier()?;
                parameters.push(Parameter::SignatureDependency {
                    signature,
                    name: param_name,
                });
            } else {
                let binding = self.parse_declaration_typed_binding()?;
                parameters.push(Parameter::Value(binding));
            }

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(TokenKind::RightParenthesis) {
                return self.failure(SyntaxFailure::MalformedDeclaration, self.current_span());
            }
        }
        self.expect_declaration_token(TokenKind::RightParenthesis)?;

        self.expect_declaration_token(TokenKind::ReturnType)?;
        let result_type = self.expect_declaration_identifier()?;

        let satisfaction = if self.peek_kind() == Some(TokenKind::Colon) {
            self.advance();
            Some(self.parse_declaration_qualified_name()?)
        } else {
            None
        };

        self.expect_declaration_token(TokenKind::LeftBrace)?;
        let body = self.parse_function_body()?;
        self.expect_declaration_token(TokenKind::RightBrace)?;

        Ok(FunctionDefinition {
            visibility,
            name,
            parameters,
            result_type,
            satisfaction,
            body,
        })
    }

    fn parse_function_body(&mut self) -> Result<FunctionBody<'source>, CompileFailure> {
        let mut statements = Vec::new();

        while self.peek_kind() != Some(TokenKind::Return)
            && self.peek_kind() != Some(TokenKind::RightBrace)
            && !self.is_at_end()
        {
            if self.peek_kind() == Some(TokenKind::Let) {
                self.advance();
                let binding = self.parse_declaration_typed_binding()?;
                self.expect_declaration_token(TokenKind::Association)?;
                let value_info = self.parse_expression_level0()?;
                self.expect_declaration_token(TokenKind::Semicolon)?;
                statements.push(BodyStatement::Let(LetBinding {
                    binding,
                    value: value_info.expr,
                }));
            } else {
                let expr_info = self.parse_expression_level0()?;
                self.expect_declaration_token(TokenKind::Semicolon)?;
                match expr_info.expr.kind {
                    ExpressionKind::FunctionCall(call) => {
                        statements.push(BodyStatement::Operation(
                            OperationStatement::FunctionCall(call),
                        ));
                    }
                    ExpressionKind::Pipeline(pipeline) => {
                        statements.push(BodyStatement::Operation(OperationStatement::Pipeline(
                            pipeline,
                        )));
                    }
                    _ => {
                        return self.failure(
                            SyntaxFailure::InvalidOperationStatement,
                            expr_info.expr.span,
                        );
                    }
                }
            }
        }

        if self.peek_kind() == Some(TokenKind::Return) {
            self.advance();
            let result_info = self.parse_expression_level0()?;
            self.expect_declaration_token(TokenKind::Semicolon)?;

            if self.peek_kind() != Some(TokenKind::RightBrace) {
                let invalid_span = self.current_span();
                return self.failure(SyntaxFailure::InvalidReturnPlacement, invalid_span);
            }

            Ok(FunctionBody {
                statements,
                result: result_info.expr,
            })
        } else {
            let span = self.current_span();
            self.failure(SyntaxFailure::MissingFinalReturn, span)
        }
    }

    // --- Expression Parsing (Precedence Hierarchy) ---

    // Level 0: Pipeline (|>)
    fn parse_expression_level0(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let source_info = self.parse_expression_level1_logical_or()?;

        if self.peek_kind() != Some(TokenKind::Pipeline) {
            return Ok(source_info);
        }

        let mut stages = Vec::new();
        let start_span = source_info.expr.span;
        let mut end_span = start_span;

        while self.peek_kind() == Some(TokenKind::Pipeline) {
            self.advance(); // consume |>

            let callee = self.expect_expression_identifier()?;
            end_span = callee.span;

            if self.peek_kind() == Some(TokenKind::LeftParenthesis) {
                self.advance(); // consume (

                // Multi-arg pipeline stage: MUST have `this` as the first argument!
                if self.peek_kind() == Some(TokenKind::RightParenthesis) {
                    // value |> op() -> invalid this usage
                    let span = SourceSpan {
                        start: callee.span.start,
                        end: self.current_span().end,
                    };
                    return self.failure(SyntaxFailure::InvalidThisUsage, span);
                }

                if self.peek_kind() != Some(TokenKind::This) {
                    let span = self.current_span();
                    return self.failure(SyntaxFailure::InvalidThisUsage, span);
                }

                let this_span = self.peek().unwrap().span;
                self.advance(); // consume `this`

                if self.peek_kind() == Some(TokenKind::RightParenthesis) {
                    // value |> op(this) -> invalid this usage (requires >= 1 additional arg)
                    return self.failure(SyntaxFailure::InvalidThisUsage, this_span);
                }

                if self.peek_kind() != Some(TokenKind::Comma) {
                    let span = self.current_span();
                    return self.failure(SyntaxFailure::InvalidThisUsage, span);
                }
                self.advance(); // consume comma

                let mut additional_arguments = Vec::new();
                while self.peek_kind() != Some(TokenKind::RightParenthesis) {
                    if self.is_at_end() {
                        return self
                            .failure(SyntaxFailure::MalformedExpression, self.current_span());
                    }
                    let arg_info = self.parse_expression_level0()?;
                    additional_arguments.push(arg_info.expr);

                    if self.peek_kind() == Some(TokenKind::Comma) {
                        self.advance();
                    } else if self.peek_kind() != Some(TokenKind::RightParenthesis) {
                        return self
                            .failure(SyntaxFailure::MalformedExpression, self.current_span());
                    }
                }
                let close_tok = self.expect_expression_token(TokenKind::RightParenthesis)?;
                end_span = close_tok.span;

                stages.push(PipelineStage {
                    callee,
                    additional_arguments,
                });
            } else {
                // 1-argument stage
                stages.push(PipelineStage {
                    callee,
                    additional_arguments: Vec::new(),
                });
            }
        }

        let full_span = SourceSpan {
            start: start_span.start,
            end: end_span.end,
        };

        Ok(ExprInfo {
            expr: Expression {
                kind: ExpressionKind::Pipeline(Pipeline {
                    source: Box::new(source_info.expr),
                    stages,
                }),
                span: full_span,
            },
            is_grouped: false,
            op_kind: Some(OpKind::Other),
        })
    }

    // Level 1: Logical OR (||)
    fn parse_expression_level1_logical_or(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let mut left = self.parse_expression_level2_logical_and()?;

        while self.peek_kind() == Some(TokenKind::Or) {
            self.advance();

            // Explicit grouping rule: unparenthesized Comparison or LogicalAnd as operand of LogicalOr is invalid
            if !left.is_grouped
                && (left.op_kind == Some(OpKind::Comparison)
                    || left.op_kind == Some(OpKind::LogicalAnd))
            {
                return self.failure(SyntaxFailure::MalformedExpression, left.expr.span);
            }

            let right = self.parse_expression_level2_logical_and()?;

            if !right.is_grouped
                && (right.op_kind == Some(OpKind::Comparison)
                    || right.op_kind == Some(OpKind::LogicalAnd))
            {
                return self.failure(SyntaxFailure::MalformedExpression, right.expr.span);
            }

            let span = SourceSpan {
                start: left.expr.span.start,
                end: right.expr.span.end,
            };

            left = ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::Binary {
                        left: Box::new(left.expr),
                        operator: BinaryOperator::Or,
                        right: Box::new(right.expr),
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::LogicalOr),
            };
        }

        Ok(left)
    }

    // Level 2: Logical AND (&&)
    fn parse_expression_level2_logical_and(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let mut left = self.parse_expression_level3_comparison()?;

        while self.peek_kind() == Some(TokenKind::And) {
            self.advance();

            // Explicit grouping rule: unparenthesized Comparison or LogicalOr as operand of LogicalAnd is invalid
            if !left.is_grouped
                && (left.op_kind == Some(OpKind::Comparison)
                    || left.op_kind == Some(OpKind::LogicalOr))
            {
                return self.failure(SyntaxFailure::MalformedExpression, left.expr.span);
            }

            let right = self.parse_expression_level3_comparison()?;

            if !right.is_grouped
                && (right.op_kind == Some(OpKind::Comparison)
                    || right.op_kind == Some(OpKind::LogicalOr))
            {
                return self.failure(SyntaxFailure::MalformedExpression, right.expr.span);
            }

            let span = SourceSpan {
                start: left.expr.span.start,
                end: right.expr.span.end,
            };

            left = ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::Binary {
                        left: Box::new(left.expr),
                        operator: BinaryOperator::And,
                        right: Box::new(right.expr),
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::LogicalAnd),
            };
        }

        Ok(left)
    }

    // Level 3: Comparison (<, <=, >, >=, ==, !=) (Non-chainable)
    fn parse_expression_level3_comparison(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let left = self.parse_expression_level4_additive()?;

        let op = match self.peek_kind() {
            Some(TokenKind::Less) => Some(BinaryOperator::Less),
            Some(TokenKind::LessEqual) => Some(BinaryOperator::LessEqual),
            Some(TokenKind::Greater) => Some(BinaryOperator::Greater),
            Some(TokenKind::GreaterEqual) => Some(BinaryOperator::GreaterEqual),
            Some(TokenKind::Equal) => Some(BinaryOperator::Equal),
            Some(TokenKind::NotEqual) => Some(BinaryOperator::NotEqual),
            _ => None,
        };

        if let Some(binary_op) = op {
            self.advance(); // consume comparison operator

            if !left.is_grouped && left.op_kind == Some(OpKind::Comparison) {
                return self.failure(SyntaxFailure::MalformedExpression, left.expr.span);
            }

            let right = self.parse_expression_level4_additive()?;

            if !right.is_grouped && right.op_kind == Some(OpKind::Comparison) {
                return self.failure(SyntaxFailure::MalformedExpression, right.expr.span);
            }

            // Check if chained comparison follows immediately (e.g. a < b < c)
            let chained_op = match self.peek_kind() {
                Some(TokenKind::Less)
                | Some(TokenKind::LessEqual)
                | Some(TokenKind::Greater)
                | Some(TokenKind::GreaterEqual)
                | Some(TokenKind::Equal)
                | Some(TokenKind::NotEqual) => true,
                _ => false,
            };
            if chained_op {
                let span = self.current_span();
                return self.failure(SyntaxFailure::MalformedExpression, span);
            }

            let span = SourceSpan {
                start: left.expr.span.start,
                end: right.expr.span.end,
            };

            Ok(ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::Binary {
                        left: Box::new(left.expr),
                        operator: binary_op,
                        right: Box::new(right.expr),
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::Comparison),
            })
        } else {
            Ok(left)
        }
    }

    // Level 4: Additive (+, -)
    fn parse_expression_level4_additive(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let mut left = self.parse_expression_level5_multiplicative()?;

        while let Some(op_kind) = self.peek_kind() {
            let binary_op = match op_kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance();

            let right = self.parse_expression_level5_multiplicative()?;
            let span = SourceSpan {
                start: left.expr.span.start,
                end: right.expr.span.end,
            };

            left = ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::Binary {
                        left: Box::new(left.expr),
                        operator: binary_op,
                        right: Box::new(right.expr),
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::Other),
            };
        }

        Ok(left)
    }

    // Level 5: Multiplicative (*, /, %)
    fn parse_expression_level5_multiplicative(
        &mut self,
    ) -> Result<ExprInfo<'source>, CompileFailure> {
        let mut left = self.parse_expression_level6_unary()?;

        while let Some(op_kind) = self.peek_kind() {
            let binary_op = match op_kind {
                TokenKind::Multiply => BinaryOperator::Multiply,
                TokenKind::Divide => BinaryOperator::Divide,
                TokenKind::Remainder => BinaryOperator::Remainder,
                _ => break,
            };
            self.advance();

            let right = self.parse_expression_level6_unary()?;
            let span = SourceSpan {
                start: left.expr.span.start,
                end: right.expr.span.end,
            };

            left = ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::Binary {
                        left: Box::new(left.expr),
                        operator: binary_op,
                        right: Box::new(right.expr),
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::Other),
            };
        }

        Ok(left)
    }

    // Level 6: Unary (!, -)
    fn parse_expression_level6_unary(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        if let Some(tok) = self.peek() {
            if tok.kind == TokenKind::Not {
                let op_tok = self.advance().unwrap();
                let operand = self.parse_expression_level6_unary()?;
                let span = SourceSpan {
                    start: op_tok.span.start,
                    end: operand.expr.span.end,
                };
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Unary {
                            operator: UnaryOperator::Not,
                            operand: Box::new(operand.expr),
                        },
                        span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            } else if tok.kind == TokenKind::Minus {
                let op_tok = self.advance().unwrap();
                let operand = self.parse_expression_level6_unary()?;
                let span = SourceSpan {
                    start: op_tok.span.start,
                    end: operand.expr.span.end,
                };
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Unary {
                            operator: UnaryOperator::Negate,
                            operand: Box::new(operand.expr),
                        },
                        span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            } else if tok.kind == TokenKind::Plus {
                // No unary + in Evo-Script
                return self.failure(SyntaxFailure::MalformedExpression, tok.span);
            }
        }

        self.parse_expression_level7_postfix()
    }

    // Level 7: Postfix (. field)
    fn parse_expression_level7_postfix(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let mut receiver = self.parse_expression_level8_primary()?;

        while self.peek_kind() == Some(TokenKind::FieldAccess) {
            self.advance(); // consume '.'
            let field = self.expect_expression_identifier()?;
            let span = SourceSpan {
                start: receiver.expr.span.start,
                end: field.span.end,
            };
            receiver = ExprInfo {
                expr: Expression {
                    kind: ExpressionKind::FieldAccess {
                        receiver: Box::new(receiver.expr),
                        field,
                    },
                    span,
                },
                is_grouped: false,
                op_kind: Some(OpKind::Other),
            };
        }

        Ok(receiver)
    }

    // Level 8: Primary
    fn parse_expression_level8_primary(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        if self.is_at_end() {
            let eof_span = SourceSpan {
                start: self.source.len(),
                end: self.source.len(),
            };
            return self.failure(SyntaxFailure::MalformedExpression, eof_span);
        }

        let tok = self.peek().unwrap();

        // 1. Literals
        match tok.kind {
            TokenKind::IntegerLiteral => {
                let tok = self.advance().unwrap();
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Integer,
                            lexeme: tok.lexeme,
                        },
                        span: tok.span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            }
            TokenKind::FloatingLiteral => {
                let tok = self.advance().unwrap();
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Floating,
                            lexeme: tok.lexeme,
                        },
                        span: tok.span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            }
            TokenKind::StringLiteral => {
                let tok = self.advance().unwrap();
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::String,
                            lexeme: tok.lexeme,
                        },
                        span: tok.span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            }
            TokenKind::BooleanLiteral => {
                let tok = self.advance().unwrap();
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Boolean,
                            lexeme: tok.lexeme,
                        },
                        span: tok.span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            }
            TokenKind::This => {
                let tok = self.advance().unwrap();
                return self.failure(SyntaxFailure::InvalidThisUsage, tok.span);
            }
            TokenKind::LeftParenthesis => {
                let open_tok = self.advance().unwrap();
                let inner = self.parse_expression_level0()?;
                let close_tok = self.expect_expression_token(TokenKind::RightParenthesis)?;
                let full_span = SourceSpan {
                    start: open_tok.span.start,
                    end: close_tok.span.end,
                };
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: inner.expr.kind,
                        span: full_span,
                    },
                    is_grouped: true,
                    op_kind: None,
                });
            }
            TokenKind::When => {
                return self.parse_when_expression();
            }
            TokenKind::Identifier => {
                let id_tok = self.advance().unwrap();
                let id = Identifier {
                    lexeme: id_tok.lexeme,
                    span: id_tok.span,
                };
                let id_span = id.span;

                // Check FunctionCall: id(...)
                if self.peek_kind() == Some(TokenKind::LeftParenthesis) {
                    self.advance();
                    let mut arguments = Vec::new();
                    while self.peek_kind() != Some(TokenKind::RightParenthesis) {
                        if self.is_at_end() {
                            return self
                                .failure(SyntaxFailure::MalformedExpression, self.current_span());
                        }
                        let arg = self.parse_expression_level0()?;
                        arguments.push(arg.expr);

                        if self.peek_kind() == Some(TokenKind::Comma) {
                            self.advance();
                        } else if self.peek_kind() != Some(TokenKind::RightParenthesis) {
                            return self
                                .failure(SyntaxFailure::MalformedExpression, self.current_span());
                        }
                    }
                    let close_tok = self.expect_expression_token(TokenKind::RightParenthesis)?;
                    let span = SourceSpan {
                        start: id_span.start,
                        end: close_tok.span.end,
                    };
                    return Ok(ExprInfo {
                        expr: Expression {
                            kind: ExpressionKind::FunctionCall(FunctionCall {
                                callee: id,
                                arguments,
                            }),
                            span,
                        },
                        is_grouped: false,
                        op_kind: Some(OpKind::Other),
                    });
                }

                // Check EnumConstruction: Qualifier::Variant ...
                if self.peek_kind() == Some(TokenKind::Qualification) {
                    self.advance(); // consume ::
                    let var_id = self.expect_expression_identifier()?;
                    let variant = QualifiedName {
                        qualifier: id,
                        name: var_id,
                    };

                    if self.peek_kind() == Some(TokenKind::LeftParenthesis) {
                        self.advance();
                        let value_info = self.parse_expression_level0()?;
                        let close_tok =
                            self.expect_expression_token(TokenKind::RightParenthesis)?;
                        let span = SourceSpan {
                            start: variant.qualifier.span.start,
                            end: close_tok.span.end,
                        };
                        return Ok(ExprInfo {
                            expr: Expression {
                                kind: ExpressionKind::EnumConstruction(
                                    EnumConstruction::Associated {
                                        variant,
                                        value: Box::new(value_info.expr),
                                    },
                                ),
                                span,
                            },
                            is_grouped: false,
                            op_kind: Some(OpKind::Other),
                        });
                    } else if self.peek_kind() == Some(TokenKind::LeftBrace) {
                        // Disambiguate Structured EnumConstruction vs surrounding `when subject { ... }`
                        let is_structured_enum_construction = if !self.in_when_subject {
                            true
                        } else if self.peek_nth_kind(1) == Some(TokenKind::RightBrace) {
                            true
                        } else if self.peek_nth_kind(1) == Some(TokenKind::Identifier)
                            && self.peek_nth_kind(2) == Some(TokenKind::Colon)
                        {
                            true
                        } else {
                            false
                        };

                        if is_structured_enum_construction {
                            self.advance(); // consume {
                            let mut fields = Vec::new();
                            while self.peek_kind() != Some(TokenKind::RightBrace) {
                                if self.is_at_end() {
                                    return self.failure(
                                        SyntaxFailure::MalformedExpression,
                                        self.current_span(),
                                    );
                                }
                                let field_name = self.expect_expression_identifier()?;
                                self.expect_expression_token(TokenKind::Colon)?;
                                let field_val = self.parse_expression_level0()?;
                                fields.push(FieldInitializer {
                                    name: field_name,
                                    value: field_val.expr,
                                });
                                if self.peek_kind() == Some(TokenKind::Comma) {
                                    self.advance();
                                }
                            }
                            let close_tok = self.expect_expression_token(TokenKind::RightBrace)?;
                            let span = SourceSpan {
                                start: variant.qualifier.span.start,
                                end: close_tok.span.end,
                            };
                            return Ok(ExprInfo {
                                expr: Expression {
                                    kind: ExpressionKind::EnumConstruction(
                                        EnumConstruction::Structured { variant, fields },
                                    ),
                                    span,
                                },
                                is_grouped: false,
                                op_kind: Some(OpKind::Other),
                            });
                        } else {
                            // Simple EnumConstruction followed by `{` of `when`
                            let span = SourceSpan {
                                start: variant.qualifier.span.start,
                                end: variant.name.span.end,
                            };
                            return Ok(ExprInfo {
                                expr: Expression {
                                    kind: ExpressionKind::EnumConstruction(
                                        EnumConstruction::Simple { variant },
                                    ),
                                    span,
                                },
                                is_grouped: false,
                                op_kind: Some(OpKind::Other),
                            });
                        }
                    } else {
                        let span = SourceSpan {
                            start: variant.qualifier.span.start,
                            end: variant.name.span.end,
                        };
                        return Ok(ExprInfo {
                            expr: Expression {
                                kind: ExpressionKind::EnumConstruction(EnumConstruction::Simple {
                                    variant,
                                }),
                                span,
                            },
                            is_grouped: false,
                            op_kind: Some(OpKind::Other),
                        });
                    }
                }

                // Check StructConstruction: Identifier { field: val, ... }
                if self.peek_kind() == Some(TokenKind::LeftBrace) {
                    // Disambiguate StructConstruction vs `when subject { ... }`
                    let is_struct_construction = if !self.in_when_subject {
                        true
                    } else if self.peek_nth_kind(1) == Some(TokenKind::RightBrace) {
                        true
                    } else if self.peek_nth_kind(1) == Some(TokenKind::Identifier)
                        && self.peek_nth_kind(2) == Some(TokenKind::Colon)
                    {
                        true
                    } else {
                        false
                    };

                    if is_struct_construction {
                        self.advance(); // consume {
                        let mut fields = Vec::new();
                        while self.peek_kind() != Some(TokenKind::RightBrace) {
                            if self.is_at_end() {
                                return self.failure(
                                    SyntaxFailure::MalformedExpression,
                                    self.current_span(),
                                );
                            }
                            let field_name = self.expect_expression_identifier()?;
                            self.expect_expression_token(TokenKind::Colon)?;
                            let field_val = self.parse_expression_level0()?;
                            fields.push(FieldInitializer {
                                name: field_name,
                                value: field_val.expr,
                            });
                            if self.peek_kind() == Some(TokenKind::Comma) {
                                self.advance();
                            }
                        }
                        let close_tok = self.expect_expression_token(TokenKind::RightBrace)?;
                        let span = SourceSpan {
                            start: id_span.start,
                            end: close_tok.span.end,
                        };
                        return Ok(ExprInfo {
                            expr: Expression {
                                kind: ExpressionKind::StructConstruction {
                                    type_name: id,
                                    fields,
                                },
                                span,
                            },
                            is_grouped: false,
                            op_kind: Some(OpKind::Other),
                        });
                    }
                }

                // Plain Identifier
                return Ok(ExprInfo {
                    expr: Expression {
                        kind: ExpressionKind::Identifier(id),
                        span: id_span,
                    },
                    is_grouped: false,
                    op_kind: Some(OpKind::Other),
                });
            }
            _ => {
                return self.failure(SyntaxFailure::MalformedExpression, tok.span);
            }
        }
    }

    // --- When Expression Parsing ---
    fn parse_when_expression(&mut self) -> Result<ExprInfo<'source>, CompileFailure> {
        let when_tok = self.expect_expression_token(TokenKind::When)?;

        let prev_in_when_subject = self.in_when_subject;
        self.in_when_subject = true;
        let subject_info = self.parse_expression_level0();
        self.in_when_subject = prev_in_when_subject;
        let subject_info = subject_info?;

        self.expect_expression_token(TokenKind::LeftBrace)?;

        if self.peek_kind() == Some(TokenKind::RightBrace) {
            let span = self.peek().unwrap().span;
            return self.failure(SyntaxFailure::MalformedExpression, span);
        }

        let mut correspondences = Vec::new();

        while self.peek_kind() != Some(TokenKind::RightBrace) {
            if self.is_at_end() {
                return self.failure(SyntaxFailure::MalformedExpression, self.current_span());
            }

            let variant = self.parse_expression_qualified_name()?;
            let pattern = if self.peek_kind() == Some(TokenKind::LeftParenthesis) {
                self.advance();
                let type_name = self.expect_expression_identifier()?;
                let name = self.expect_expression_identifier()?;
                self.expect_expression_token(TokenKind::RightParenthesis)?;
                WhenPattern::Associated {
                    variant,
                    binding: TypedBinding { type_name, name },
                }
            } else if self.peek_kind() == Some(TokenKind::LeftBrace) {
                self.advance();
                let mut fields = Vec::new();
                while self.peek_kind() != Some(TokenKind::RightBrace) {
                    if self.is_at_end() {
                        return self
                            .failure(SyntaxFailure::MalformedExpression, self.current_span());
                    }
                    let field = self.expect_expression_identifier()?;
                    self.expect_expression_token(TokenKind::Colon)?;
                    let type_name = self.expect_expression_identifier()?;
                    let name = self.expect_expression_identifier()?;
                    self.expect_expression_token(TokenKind::Semicolon)?;
                    fields.push(PatternField {
                        field,
                        binding: TypedBinding { type_name, name },
                    });
                }
                self.expect_expression_token(TokenKind::RightBrace)?;
                WhenPattern::Structured { variant, fields }
            } else {
                WhenPattern::Simple { variant }
            };

            self.expect_expression_token(TokenKind::Correspondence)?;

            if self.peek_kind() == Some(TokenKind::Return) {
                let return_span = self.peek().unwrap().span;
                return self.failure(SyntaxFailure::InvalidReturnPlacement, return_span);
            }

            let result_info = self.parse_expression_level0()?;
            correspondences.push(WhenCorrespondence {
                pattern,
                result: result_info.expr,
            });

            if self.peek_kind() == Some(TokenKind::Comma) {
                self.advance();
            }
        }

        let close_tok = self.expect_expression_token(TokenKind::RightBrace)?;
        let span = SourceSpan {
            start: when_tok.span.start,
            end: close_tok.span.end,
        };

        Ok(ExprInfo {
            expr: Expression {
                kind: ExpressionKind::When(WhenExpression {
                    subject: Box::new(subject_info.expr),
                    correspondences,
                }),
                span,
            },
            is_grouped: false,
            op_kind: Some(OpKind::Other),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unwrap_ok<'source>(res: Result<Program<'source>, CompileFailure>) -> Program<'source> {
        match res {
            Ok(p) => p,
            Err(_) => panic!("expected Ok"),
        }
    }

    fn unwrap_err<'source>(res: Result<Program<'source>, CompileFailure>) -> CompileFailure {
        match res {
            Ok(_) => panic!("expected Err, got Ok"),
            Err(e) => e,
        }
    }

    fn make_token<'source>(
        kind: TokenKind,
        lexeme: &'source str,
        start: usize,
        end: usize,
    ) -> Token<'source> {
        Token {
            kind,
            lexeme,
            span: SourceSpan { start, end },
        }
    }

    #[test]
    fn parse_binding_and_type_check() {
        let parse: Parse = parse_tokens;
        let bound: Parse = PARSE_TOKENS;

        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "main", 10, 14),
            make_token(TokenKind::LeftParenthesis, "(", 14, 15),
            make_token(TokenKind::RightParenthesis, ")", 15, 16),
            make_token(TokenKind::ReturnType, "->", 17, 19),
            make_token(TokenKind::Identifier, "int", 20, 23),
            make_token(TokenKind::LeftBrace, "{", 24, 25),
            make_token(TokenKind::Return, "return", 26, 32),
            make_token(TokenKind::IntegerLiteral, "0", 33, 34),
            make_token(TokenKind::Semicolon, ";", 34, 35),
            make_token(TokenKind::RightBrace, "}", 36, 37),
        ];

        let p1 = unwrap_ok(parse(&tokens, "public fn main() -> int { return 0; }"));
        assert_eq!(p1.declarations.len(), 1);

        let p2 = unwrap_ok(bound(&tokens, "public fn main() -> int { return 0; }"));
        assert_eq!(p2.declarations.len(), 1);
    }

    #[test]
    fn parse_lifetime_independence() {
        let source = "public fn run() -> int { return 42; }";
        let program = {
            let inner_tokens = vec![
                make_token(TokenKind::Public, "public", 0, 6),
                make_token(TokenKind::Fn, "fn", 7, 9),
                make_token(TokenKind::Identifier, "run", 10, 13),
                make_token(TokenKind::LeftParenthesis, "(", 13, 14),
                make_token(TokenKind::RightParenthesis, ")", 14, 15),
                make_token(TokenKind::ReturnType, "->", 16, 18),
                make_token(TokenKind::Identifier, "int", 19, 22),
                make_token(TokenKind::LeftBrace, "{", 23, 24),
                make_token(TokenKind::Return, "return", 25, 31),
                make_token(TokenKind::IntegerLiteral, "42", 32, 34),
                make_token(TokenKind::Semicolon, ";", 34, 35),
                make_token(TokenKind::RightBrace, "}", 36, 37),
            ];
            unwrap_ok(parse_tokens(&inner_tokens, source))
            // inner_tokens dropped here
        };

        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.name.lexeme, "run");
                match &f.body.result.kind {
                    ExpressionKind::Literal { lexeme, .. } => assert_eq!(*lexeme, "42"),
                    _ => panic!("expected Literal"),
                }
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_eof_provenance_missing_public_function() {
        let source = "   // comment final";
        let tokens: Vec<Token> = vec![];
        let err = unwrap_err(parse_tokens(&tokens, source));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MissingPublicFunction) => {
                assert_eq!(
                    err.source_span,
                    SourceSpan {
                        start: source.len(),
                        end: source.len(),
                    }
                );
            }
            _ => panic!("expected MissingPublicFunction"),
        }
    }

    #[test]
    fn parse_program_imports_and_invalid_placement() {
        let source = "import M::A; import M::B as Aliased; public fn run() -> int { return 0; }";
        let tokens = vec![
            make_token(TokenKind::Import, "import", 0, 6),
            make_token(TokenKind::Identifier, "M", 7, 8),
            make_token(TokenKind::Qualification, "::", 8, 10),
            make_token(TokenKind::Identifier, "A", 10, 11),
            make_token(TokenKind::Semicolon, ";", 11, 12),
            make_token(TokenKind::Import, "import", 13, 19),
            make_token(TokenKind::Identifier, "M", 20, 21),
            make_token(TokenKind::Qualification, "::", 21, 23),
            make_token(TokenKind::Identifier, "B", 23, 24),
            make_token(TokenKind::As, "as", 25, 27),
            make_token(TokenKind::Identifier, "Aliased", 28, 35),
            make_token(TokenKind::Semicolon, ";", 35, 36),
            make_token(TokenKind::Public, "public", 37, 43),
            make_token(TokenKind::Fn, "fn", 44, 46),
            make_token(TokenKind::Identifier, "run", 47, 50),
            make_token(TokenKind::LeftParenthesis, "(", 50, 51),
            make_token(TokenKind::RightParenthesis, ")", 51, 52),
            make_token(TokenKind::ReturnType, "->", 53, 55),
            make_token(TokenKind::Identifier, "int", 56, 59),
            make_token(TokenKind::LeftBrace, "{", 60, 61),
            make_token(TokenKind::Return, "return", 62, 68),
            make_token(TokenKind::IntegerLiteral, "0", 69, 70),
            make_token(TokenKind::Semicolon, ";", 70, 71),
            make_token(TokenKind::RightBrace, "}", 72, 73),
        ];
        let p = unwrap_ok(parse_tokens(&tokens, source));
        assert_eq!(p.imports.len(), 2);
        assert_eq!(p.imports[0].symbol.qualifier.lexeme, "M");
        assert_eq!(p.imports[0].symbol.name.lexeme, "A");
        assert!(p.imports[0].alias.is_none());
        assert_eq!(p.imports[1].alias.as_ref().unwrap().lexeme, "Aliased");

        // Invalid import placement after declaration
        let bad_tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::IntegerLiteral, "0", 32, 33),
            make_token(TokenKind::Semicolon, ";", 33, 34),
            make_token(TokenKind::RightBrace, "}", 35, 36),
            make_token(TokenKind::Import, "import", 37, 43),
            make_token(TokenKind::Identifier, "M", 44, 45),
            make_token(TokenKind::Qualification, "::", 45, 47),
            make_token(TokenKind::Identifier, "A", 47, 48),
            make_token(TokenKind::Semicolon, ";", 48, 49),
        ];
        let err = unwrap_err(parse_tokens(&bad_tokens, "bad"));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidImportPlacement) => {}
            _ => panic!("expected InvalidImportPlacement"),
        }
    }

    #[test]
    fn parse_public_function_cardinality_and_malformed_public() {
        // True MultiplePublicFunctions: 2 structurally valid public functions
        let tokens_two_public = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f1", 10, 12),
            make_token(TokenKind::LeftParenthesis, "(", 12, 13),
            make_token(TokenKind::RightParenthesis, ")", 13, 14),
            make_token(TokenKind::ReturnType, "->", 15, 17),
            make_token(TokenKind::Identifier, "int", 18, 21),
            make_token(TokenKind::LeftBrace, "{", 22, 23),
            make_token(TokenKind::Return, "return", 24, 30),
            make_token(TokenKind::IntegerLiteral, "1", 31, 32),
            make_token(TokenKind::Semicolon, ";", 32, 33),
            make_token(TokenKind::RightBrace, "}", 34, 35),
            make_token(TokenKind::Public, "public", 36, 42),
            make_token(TokenKind::Fn, "fn", 43, 45),
            make_token(TokenKind::Identifier, "f2", 46, 48),
            make_token(TokenKind::LeftParenthesis, "(", 48, 49),
            make_token(TokenKind::RightParenthesis, ")", 49, 50),
            make_token(TokenKind::ReturnType, "->", 51, 53),
            make_token(TokenKind::Identifier, "int", 54, 57),
            make_token(TokenKind::LeftBrace, "{", 58, 59),
            make_token(TokenKind::Return, "return", 60, 66),
            make_token(TokenKind::IntegerLiteral, "2", 67, 68),
            make_token(TokenKind::Semicolon, ";", 68, 69),
            make_token(TokenKind::RightBrace, "}", 70, 71),
        ];
        let err1 = unwrap_err(parse_tokens(&tokens_two_public, "source"));
        match err1.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MultiplePublicFunctions) => {
                assert_eq!(err1.source_span, SourceSpan { start: 36, end: 42 });
            }
            _ => panic!("expected MultiplePublicFunctions"),
        }

        // Malformed declaration beginning with public after a valid public function
        let tokens_malformed_public = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "valid", 10, 15),
            make_token(TokenKind::LeftParenthesis, "(", 15, 16),
            make_token(TokenKind::RightParenthesis, ")", 16, 17),
            make_token(TokenKind::ReturnType, "->", 18, 20),
            make_token(TokenKind::Identifier, "int", 21, 24),
            make_token(TokenKind::LeftBrace, "{", 25, 26),
            make_token(TokenKind::Return, "return", 27, 33),
            make_token(TokenKind::IntegerLiteral, "1", 34, 35),
            make_token(TokenKind::Semicolon, ";", 35, 36),
            make_token(TokenKind::RightBrace, "}", 37, 38),
            make_token(TokenKind::Public, "public", 39, 45),
        ];
        let err2 = unwrap_err(parse_tokens(&tokens_malformed_public, "source"));
        match err2.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedDeclaration) => {}
            _ => panic!("expected MalformedDeclaration for trailing public"),
        }

        // Public struct Broken {} -> MalformedDeclaration
        let tokens_public_struct = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "valid", 10, 15),
            make_token(TokenKind::LeftParenthesis, "(", 15, 16),
            make_token(TokenKind::RightParenthesis, ")", 16, 17),
            make_token(TokenKind::ReturnType, "->", 18, 20),
            make_token(TokenKind::Identifier, "int", 21, 24),
            make_token(TokenKind::LeftBrace, "{", 25, 26),
            make_token(TokenKind::Return, "return", 27, 33),
            make_token(TokenKind::IntegerLiteral, "1", 34, 35),
            make_token(TokenKind::Semicolon, ";", 35, 36),
            make_token(TokenKind::RightBrace, "}", 37, 38),
            make_token(TokenKind::Public, "public", 39, 45),
            make_token(TokenKind::Struct, "struct", 46, 52),
            make_token(TokenKind::Identifier, "Broken", 53, 59),
            make_token(TokenKind::LeftBrace, "{", 60, 61),
            make_token(TokenKind::RightBrace, "}", 61, 62),
        ];
        let err3 = unwrap_err(parse_tokens(&tokens_public_struct, "source"));
        match err3.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedDeclaration) => {}
            _ => panic!("expected MalformedDeclaration for public struct"),
        }
    }

    #[test]
    fn parse_type_definitions_and_empty_enum() {
        let source = "struct Empty {} struct User { int32 id; string name; } enum Status { Active Assoc(int32) Struct { int32 val; } } public fn run() -> int { return 0; }";
        let tokens = vec![
            make_token(TokenKind::Struct, "struct", 0, 6),
            make_token(TokenKind::Identifier, "Empty", 7, 12),
            make_token(TokenKind::LeftBrace, "{", 13, 14),
            make_token(TokenKind::RightBrace, "}", 14, 15),
            make_token(TokenKind::Struct, "struct", 16, 22),
            make_token(TokenKind::Identifier, "User", 23, 27),
            make_token(TokenKind::LeftBrace, "{", 28, 29),
            make_token(TokenKind::Identifier, "int32", 30, 35),
            make_token(TokenKind::Identifier, "id", 36, 38),
            make_token(TokenKind::Semicolon, ";", 38, 39),
            make_token(TokenKind::Identifier, "string", 40, 46),
            make_token(TokenKind::Identifier, "name", 47, 51),
            make_token(TokenKind::Semicolon, ";", 51, 52),
            make_token(TokenKind::RightBrace, "}", 53, 54),
            make_token(TokenKind::Enum, "enum", 55, 59),
            make_token(TokenKind::Identifier, "Status", 60, 66),
            make_token(TokenKind::LeftBrace, "{", 67, 68),
            make_token(TokenKind::Identifier, "Active", 69, 75),
            make_token(TokenKind::Identifier, "Assoc", 76, 81),
            make_token(TokenKind::LeftParenthesis, "(", 81, 82),
            make_token(TokenKind::Identifier, "int32", 82, 87),
            make_token(TokenKind::RightParenthesis, ")", 87, 88),
            make_token(TokenKind::Identifier, "Struct", 89, 95),
            make_token(TokenKind::LeftBrace, "{", 96, 97),
            make_token(TokenKind::Identifier, "int32", 98, 103),
            make_token(TokenKind::Identifier, "val", 104, 107),
            make_token(TokenKind::Semicolon, ";", 107, 108),
            make_token(TokenKind::RightBrace, "}", 109, 110),
            make_token(TokenKind::RightBrace, "}", 111, 112),
            make_token(TokenKind::Public, "public", 113, 119),
            make_token(TokenKind::Fn, "fn", 120, 122),
            make_token(TokenKind::Identifier, "run", 123, 126),
            make_token(TokenKind::LeftParenthesis, "(", 126, 127),
            make_token(TokenKind::RightParenthesis, ")", 127, 128),
            make_token(TokenKind::ReturnType, "->", 129, 131),
            make_token(TokenKind::Identifier, "int", 132, 135),
            make_token(TokenKind::LeftBrace, "{", 136, 137),
            make_token(TokenKind::Return, "return", 138, 144),
            make_token(TokenKind::IntegerLiteral, "0", 145, 146),
            make_token(TokenKind::Semicolon, ";", 146, 147),
            make_token(TokenKind::RightBrace, "}", 148, 149),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        assert_eq!(p.declarations.len(), 4);

        // Empty Enum test
        let empty_enum_tokens = vec![
            make_token(TokenKind::Enum, "enum", 0, 4),
            make_token(TokenKind::Identifier, "Impossible", 5, 15),
            make_token(TokenKind::LeftBrace, "{", 16, 17),
            make_token(TokenKind::RightBrace, "}", 17, 18),
        ];
        let err = unwrap_err(parse_tokens(&empty_enum_tokens, "enum Impossible {}"));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::EmptyEnum) => {
                assert_eq!(err.source_span, SourceSpan { start: 17, end: 18 });
            }
            _ => panic!("expected EmptyEnum"),
        }
    }

    #[test]
    fn parse_function_parameters_satisfaction_and_body() {
        let source = "public fn compute(int32 x, Math::Adder adder) -> int32 : Core::Runner { let int32 y = 10; call(y); return y; }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "compute", 10, 17),
            make_token(TokenKind::LeftParenthesis, "(", 17, 18),
            make_token(TokenKind::Identifier, "int32", 18, 23),
            make_token(TokenKind::Identifier, "x", 24, 25),
            make_token(TokenKind::Comma, ",", 25, 26),
            make_token(TokenKind::Identifier, "Math", 27, 31),
            make_token(TokenKind::Qualification, "::", 31, 33),
            make_token(TokenKind::Identifier, "Adder", 33, 38),
            make_token(TokenKind::Identifier, "adder", 39, 44),
            make_token(TokenKind::RightParenthesis, ")", 44, 45),
            make_token(TokenKind::ReturnType, "->", 46, 48),
            make_token(TokenKind::Identifier, "int32", 49, 54),
            make_token(TokenKind::Colon, ":", 55, 56),
            make_token(TokenKind::Identifier, "Core", 57, 61),
            make_token(TokenKind::Qualification, "::", 61, 63),
            make_token(TokenKind::Identifier, "Runner", 63, 69),
            make_token(TokenKind::LeftBrace, "{", 70, 71),
            make_token(TokenKind::Let, "let", 72, 75),
            make_token(TokenKind::Identifier, "int32", 76, 81),
            make_token(TokenKind::Identifier, "y", 82, 83),
            make_token(TokenKind::Association, "=", 84, 85),
            make_token(TokenKind::IntegerLiteral, "10", 86, 88),
            make_token(TokenKind::Semicolon, ";", 88, 89),
            make_token(TokenKind::Identifier, "call", 90, 94),
            make_token(TokenKind::LeftParenthesis, "(", 94, 95),
            make_token(TokenKind::Identifier, "y", 95, 96),
            make_token(TokenKind::RightParenthesis, ")", 96, 97),
            make_token(TokenKind::Semicolon, ";", 97, 98),
            make_token(TokenKind::Return, "return", 99, 105),
            make_token(TokenKind::Identifier, "y", 106, 107),
            make_token(TokenKind::Semicolon, ";", 107, 108),
            make_token(TokenKind::RightBrace, "}", 109, 110),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.parameters.len(), 2);
                assert!(f.satisfaction.is_some());
                assert_eq!(f.body.statements.len(), 2);
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_return_invariants() {
        // Missing final return
        let tokens_missing = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::RightBrace, "}", 24, 25),
        ];
        let err1 = unwrap_err(parse_tokens(&tokens_missing, "source"));
        match err1.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MissingFinalReturn) => {}
            _ => panic!("expected MissingFinalReturn"),
        }

        // Invalid return placement (statement after return)
        let tokens_after = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::IntegerLiteral, "0", 32, 33),
            make_token(TokenKind::Semicolon, ";", 33, 34),
            make_token(TokenKind::Identifier, "call", 35, 39),
            make_token(TokenKind::LeftParenthesis, "(", 39, 40),
            make_token(TokenKind::RightParenthesis, ")", 40, 41),
            make_token(TokenKind::Semicolon, ";", 41, 42),
            make_token(TokenKind::RightBrace, "}", 43, 44),
        ];
        let err2 = unwrap_err(parse_tokens(&tokens_after, "source"));
        match err2.kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidReturnPlacement) => {}
            _ => panic!("expected InvalidReturnPlacement"),
        }
    }

    #[test]
    fn parse_operation_statement_invalids() {
        // 10 + 20; as statement -> InvalidOperationStatement
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::IntegerLiteral, "10", 25, 27),
            make_token(TokenKind::Plus, "+", 28, 29),
            make_token(TokenKind::IntegerLiteral, "20", 30, 32),
            make_token(TokenKind::Semicolon, ";", 32, 33),
            make_token(TokenKind::Return, "return", 34, 40),
            make_token(TokenKind::IntegerLiteral, "0", 41, 42),
            make_token(TokenKind::Semicolon, ";", 42, 43),
            make_token(TokenKind::RightBrace, "}", 44, 45),
        ];
        let err = unwrap_err(parse_tokens(&tokens, "source"));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidOperationStatement) => {}
            _ => panic!("expected InvalidOperationStatement"),
        }
    }

    #[test]
    fn parse_precedence_and_grouping() {
        // a + b * c
        let source = "public fn run() -> int { return a + b * c; }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "a", 32, 33),
            make_token(TokenKind::Plus, "+", 34, 35),
            make_token(TokenKind::Identifier, "b", 36, 37),
            make_token(TokenKind::Multiply, "*", 38, 39),
            make_token(TokenKind::Identifier, "c", 40, 41),
            make_token(TokenKind::Semicolon, ";", 41, 42),
            make_token(TokenKind::RightBrace, "}", 43, 44),
        ];
        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::Binary {
                    left,
                    operator,
                    right,
                } => {
                    match operator {
                        BinaryOperator::Add => {}
                        _ => panic!("expected Add"),
                    }
                    match &left.kind {
                        ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "a"),
                        _ => panic!("expected id a"),
                    }
                    match &right.kind {
                        ExpressionKind::Binary {
                            operator: right_op, ..
                        } => match right_op {
                            BinaryOperator::Multiply => {}
                            _ => panic!("expected multiply"),
                        },
                        _ => panic!("expected binary right"),
                    }
                }
                _ => panic!("expected Binary Add"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_grouping_span() {
        // ((worker))
        let source = "public fn run() -> int { return ((worker)); }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::LeftParenthesis, "(", 32, 33),
            make_token(TokenKind::LeftParenthesis, "(", 33, 34),
            make_token(TokenKind::Identifier, "worker", 34, 40),
            make_token(TokenKind::RightParenthesis, ")", 40, 41),
            make_token(TokenKind::RightParenthesis, ")", 41, 42),
            make_token(TokenKind::Semicolon, ";", 42, 43),
            make_token(TokenKind::RightBrace, "}", 44, 45),
        ];
        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.body.result.span, SourceSpan { start: 32, end: 42 });
                match &f.body.result.kind {
                    ExpressionKind::Identifier(id) => {
                        assert_eq!(id.lexeme, "worker");
                        assert_eq!(id.span, SourceSpan { start: 34, end: 40 });
                    }
                    _ => panic!("expected Identifier"),
                }
            }
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_comparison_non_chainable_and_explicit_logical_grouping() {
        // a < b < c -> MalformedExpression
        let tokens_chain = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "bool", 19, 23),
            make_token(TokenKind::LeftBrace, "{", 24, 25),
            make_token(TokenKind::Return, "return", 26, 32),
            make_token(TokenKind::Identifier, "a", 33, 34),
            make_token(TokenKind::Less, "<", 35, 36),
            make_token(TokenKind::Identifier, "b", 37, 38),
            make_token(TokenKind::Less, "<", 39, 40),
            make_token(TokenKind::Identifier, "c", 41, 42),
            make_token(TokenKind::Semicolon, ";", 42, 43),
            make_token(TokenKind::RightBrace, "}", 44, 45),
        ];
        let err1 = unwrap_err(parse_tokens(&tokens_chain, "source"));
        match err1.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for chained comparison"),
        }

        // a == b && c -> MalformedExpression (unparenthesized comparison operand)
        let tokens_unp_comp = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "bool", 19, 23),
            make_token(TokenKind::LeftBrace, "{", 24, 25),
            make_token(TokenKind::Return, "return", 26, 32),
            make_token(TokenKind::Identifier, "a", 33, 34),
            make_token(TokenKind::Equal, "==", 35, 37),
            make_token(TokenKind::Identifier, "b", 38, 39),
            make_token(TokenKind::And, "&&", 40, 42),
            make_token(TokenKind::Identifier, "c", 43, 44),
            make_token(TokenKind::Semicolon, ";", 44, 45),
            make_token(TokenKind::RightBrace, "}", 46, 47),
        ];
        let err2 = unwrap_err(parse_tokens(&tokens_unp_comp, "source"));
        match err2.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for unparenthesized comparison in &&"),
        }

        // a && b || c -> MalformedExpression (mixed && and || without parens)
        let tokens_mixed = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "bool", 19, 23),
            make_token(TokenKind::LeftBrace, "{", 24, 25),
            make_token(TokenKind::Return, "return", 26, 32),
            make_token(TokenKind::Identifier, "a", 33, 34),
            make_token(TokenKind::And, "&&", 35, 37),
            make_token(TokenKind::Identifier, "b", 38, 39),
            make_token(TokenKind::Or, "||", 40, 42),
            make_token(TokenKind::Identifier, "c", 43, 44),
            make_token(TokenKind::Semicolon, ";", 44, 45),
            make_token(TokenKind::RightBrace, "}", 46, 47),
        ];
        let err3 = unwrap_err(parse_tokens(&tokens_mixed, "source"));
        match err3.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for mixed && and ||"),
        }
    }

    #[test]
    fn parse_expression_failure_classification_not_declaration() {
        // 1. Missing ')' in grouping: return (value;
        let tok1 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::LeftParenthesis, "(", 32, 33),
            make_token(TokenKind::Identifier, "value", 33, 38),
            make_token(TokenKind::Semicolon, ";", 38, 39),
            make_token(TokenKind::RightBrace, "}", 40, 41),
        ];
        let err1 = unwrap_err(parse_tokens(
            &tok1,
            "public fn run() -> int { return (value; }",
        ));
        match err1.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for unclosed grouping paren"),
        }

        // 2. Missing ')' in FunctionCall: return call(value;
        let tok2 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "call", 32, 36),
            make_token(TokenKind::LeftParenthesis, "(", 36, 37),
            make_token(TokenKind::Identifier, "value", 37, 42),
            make_token(TokenKind::Semicolon, ";", 42, 43),
            make_token(TokenKind::RightBrace, "}", 44, 45),
        ];
        let err2 = unwrap_err(parse_tokens(
            &tok2,
            "public fn run() -> int { return call(value; }",
        ));
        match err2.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for unclosed call paren"),
        }

        // 3. Malformed EnumConstruction: return Enum::(value);
        let tok3 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "Enum", 32, 36),
            make_token(TokenKind::Qualification, "::", 36, 38),
            make_token(TokenKind::LeftParenthesis, "(", 38, 39),
            make_token(TokenKind::Identifier, "value", 39, 44),
            make_token(TokenKind::RightParenthesis, ")", 44, 45),
            make_token(TokenKind::Semicolon, ";", 45, 46),
            make_token(TokenKind::RightBrace, "}", 47, 48),
        ];
        let err3 = unwrap_err(parse_tokens(
            &tok3,
            "public fn run() -> int { return Enum::(value); }",
        ));
        match err3.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for malformed enum construction"),
        }

        // 4. Malformed StructConstruction: return Struct { field value }; (missing colon)
        let tok4 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "Struct", 32, 38),
            make_token(TokenKind::LeftBrace, "{", 39, 40),
            make_token(TokenKind::Identifier, "field", 41, 46),
            make_token(TokenKind::Identifier, "value", 47, 52),
            make_token(TokenKind::RightBrace, "}", 53, 54),
            make_token(TokenKind::Semicolon, ";", 54, 55),
            make_token(TokenKind::RightBrace, "}", 56, 57),
        ];
        let err4 = unwrap_err(parse_tokens(
            &tok4,
            "public fn run() -> int { return Struct { field value }; }",
        ));
        match err4.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for malformed struct construction"),
        }

        // 5. Malformed When pattern: return when value { Enum:: => 1 };
        let tok5 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::When, "when", 32, 36),
            make_token(TokenKind::Identifier, "value", 37, 42),
            make_token(TokenKind::LeftBrace, "{", 43, 44),
            make_token(TokenKind::Identifier, "Enum", 45, 49),
            make_token(TokenKind::Qualification, "::", 49, 51),
            make_token(TokenKind::Correspondence, "=>", 52, 54),
            make_token(TokenKind::IntegerLiteral, "1", 55, 56),
            make_token(TokenKind::RightBrace, "}", 57, 58),
            make_token(TokenKind::Semicolon, ";", 58, 59),
            make_token(TokenKind::RightBrace, "}", 60, 61),
        ];
        let err5 = unwrap_err(parse_tokens(
            &tok5,
            "public fn run() -> int { return when value { Enum:: => 1 }; }",
        ));
        match err5.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression for malformed when pattern"),
        }
    }

    #[test]
    fn parse_pipeline_and_this_rules() {
        // Valid pipeline with this
        let source = "public fn run() -> int { return val |> add(this, 10) |> to_string; }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "val", 32, 35),
            make_token(TokenKind::Pipeline, "|>", 36, 38),
            make_token(TokenKind::Identifier, "add", 39, 42),
            make_token(TokenKind::LeftParenthesis, "(", 42, 43),
            make_token(TokenKind::This, "this", 43, 47),
            make_token(TokenKind::Comma, ",", 47, 48),
            make_token(TokenKind::IntegerLiteral, "10", 49, 51),
            make_token(TokenKind::RightParenthesis, ")", 51, 52),
            make_token(TokenKind::Pipeline, "|>", 53, 55),
            make_token(TokenKind::Identifier, "to_string", 56, 65),
            make_token(TokenKind::Semicolon, ";", 65, 66),
            make_token(TokenKind::RightBrace, "}", 67, 68),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::Pipeline(pipe) => {
                    assert_eq!(pipe.stages.len(), 2);
                    assert_eq!(pipe.stages[0].callee.lexeme, "add");
                    assert_eq!(pipe.stages[0].additional_arguments.len(), 1);
                    assert_eq!(pipe.stages[1].callee.lexeme, "to_string");
                    assert_eq!(pipe.stages[1].additional_arguments.len(), 0);
                }
                _ => panic!("expected Pipeline"),
            },
            _ => panic!("expected Function"),
        }

        // InvalidThisUsage: return this;
        let tokens_this = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::This, "this", 32, 36),
            make_token(TokenKind::Semicolon, ";", 36, 37),
            make_token(TokenKind::RightBrace, "}", 38, 39),
        ];
        let err = unwrap_err(parse_tokens(&tokens_this, "source"));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidThisUsage) => {}
            _ => panic!("expected InvalidThisUsage"),
        }
    }

    #[test]
    fn parse_nested_pipeline() {
        // name |> concat(this, " ", surname |> clean)
        let source =
            "public fn run() -> string { return name |> concat(this, \" \", surname |> clean); }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "string", 19, 25),
            make_token(TokenKind::LeftBrace, "{", 26, 27),
            make_token(TokenKind::Return, "return", 28, 34),
            make_token(TokenKind::Identifier, "name", 35, 39),
            make_token(TokenKind::Pipeline, "|>", 40, 42),
            make_token(TokenKind::Identifier, "concat", 43, 49),
            make_token(TokenKind::LeftParenthesis, "(", 49, 50),
            make_token(TokenKind::This, "this", 50, 54),
            make_token(TokenKind::Comma, ",", 54, 55),
            make_token(TokenKind::StringLiteral, "\" \"", 56, 59),
            make_token(TokenKind::Comma, ",", 59, 60),
            make_token(TokenKind::Identifier, "surname", 61, 68),
            make_token(TokenKind::Pipeline, "|>", 69, 71),
            make_token(TokenKind::Identifier, "clean", 72, 77),
            make_token(TokenKind::RightParenthesis, ")", 77, 78),
            make_token(TokenKind::Semicolon, ";", 78, 79),
            make_token(TokenKind::RightBrace, "}", 80, 81),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::Pipeline(pipe) => {
                    assert_eq!(pipe.stages.len(), 1);
                    assert_eq!(pipe.stages[0].additional_arguments.len(), 2);
                    match &pipe.stages[0].additional_arguments[1].kind {
                        ExpressionKind::Pipeline(inner_pipe) => {
                            assert_eq!(inner_pipe.stages.len(), 1);
                            assert_eq!(inner_pipe.stages[0].callee.lexeme, "clean");
                        }
                        _ => panic!("expected nested Pipeline"),
                    }
                }
                _ => panic!("expected outer Pipeline"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_field_access_chain_and_constructions() {
        let source = "public fn run() -> int { return country.state.name; }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "country", 32, 39),
            make_token(TokenKind::FieldAccess, ".", 39, 40),
            make_token(TokenKind::Identifier, "state", 40, 45),
            make_token(TokenKind::FieldAccess, ".", 45, 46),
            make_token(TokenKind::Identifier, "name", 46, 50),
            make_token(TokenKind::Semicolon, ";", 50, 51),
            make_token(TokenKind::RightBrace, "}", 52, 53),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::FieldAccess { receiver, field } => {
                    assert_eq!(field.lexeme, "name");
                    match &receiver.kind {
                        ExpressionKind::FieldAccess {
                            receiver: r2,
                            field: f2,
                        } => {
                            assert_eq!(f2.lexeme, "state");
                            match &r2.kind {
                                ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "country"),
                                _ => panic!("expected country id"),
                            }
                        }
                        _ => panic!("expected receiver field access"),
                    }
                }
                _ => panic!("expected FieldAccess"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_struct_construction_and_when_identifier_subject() {
        // Valid StructConstruction: return Worker { id: 10 };
        let source_struct = "public fn run() -> int { return Worker { id: 10 }; }";
        let tokens_struct = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "Worker", 32, 38),
            make_token(TokenKind::LeftBrace, "{", 39, 40),
            make_token(TokenKind::Identifier, "id", 41, 43),
            make_token(TokenKind::Colon, ":", 43, 44),
            make_token(TokenKind::IntegerLiteral, "10", 45, 47),
            make_token(TokenKind::RightBrace, "}", 47, 48),
            make_token(TokenKind::Semicolon, ";", 48, 49),
            make_token(TokenKind::RightBrace, "}", 50, 51),
        ];
        let p_struct = unwrap_ok(parse_tokens(&tokens_struct, source_struct));
        match &p_struct.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::StructConstruction { type_name, fields } => {
                    assert_eq!(type_name.lexeme, "Worker");
                    assert_eq!(fields.len(), 1);
                    assert_eq!(fields[0].name.lexeme, "id");
                }
                _ => panic!("expected StructConstruction"),
            },
            _ => panic!("expected Function"),
        }

        // When with Identifier subject: public fn run(State state) -> int { return when state { State::Ready => 1 }; }
        let source_when_id =
            "public fn run(State state) -> int { return when state { State::Ready => 1 }; }";
        let tokens_when_id = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::Identifier, "State", 14, 19),
            make_token(TokenKind::Identifier, "state", 20, 25),
            make_token(TokenKind::RightParenthesis, ")", 25, 26),
            make_token(TokenKind::ReturnType, "->", 27, 29),
            make_token(TokenKind::Identifier, "int", 30, 33),
            make_token(TokenKind::LeftBrace, "{", 34, 35),
            make_token(TokenKind::Return, "return", 36, 42),
            make_token(TokenKind::When, "when", 43, 47),
            make_token(TokenKind::Identifier, "state", 48, 53),
            make_token(TokenKind::LeftBrace, "{", 54, 55),
            make_token(TokenKind::Identifier, "State", 56, 61),
            make_token(TokenKind::Qualification, "::", 61, 63),
            make_token(TokenKind::Identifier, "Ready", 63, 68),
            make_token(TokenKind::Correspondence, "=>", 69, 71),
            make_token(TokenKind::IntegerLiteral, "1", 72, 73),
            make_token(TokenKind::RightBrace, "}", 74, 75),
            make_token(TokenKind::Semicolon, ";", 75, 76),
            make_token(TokenKind::RightBrace, "}", 77, 78),
        ];
        let p_when_id = unwrap_ok(parse_tokens(&tokens_when_id, source_when_id));
        match &p_when_id.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::When(w) => match &w.subject.kind {
                    ExpressionKind::Identifier(id) => {
                        assert_eq!(id.lexeme, "state");
                    }
                    _ => panic!("expected Identifier as when subject"),
                },
                _ => panic!("expected When expression"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_when_simple_enum_construction_as_subject_and_structured_regression() {
        // enum State { Ready } public fn run() -> int { return when State::Ready { State::Ready => 1 }; }
        let source = "enum State { Ready } public fn run() -> int { return when State::Ready { State::Ready => 1 }; }";
        let tokens = vec![
            make_token(TokenKind::Enum, "enum", 0, 4),
            make_token(TokenKind::Identifier, "State", 5, 10),
            make_token(TokenKind::LeftBrace, "{", 11, 12),
            make_token(TokenKind::Identifier, "Ready", 13, 18),
            make_token(TokenKind::RightBrace, "}", 19, 20),
            make_token(TokenKind::Public, "public", 21, 27),
            make_token(TokenKind::Fn, "fn", 28, 30),
            make_token(TokenKind::Identifier, "run", 31, 34),
            make_token(TokenKind::LeftParenthesis, "(", 34, 35),
            make_token(TokenKind::RightParenthesis, ")", 35, 36),
            make_token(TokenKind::ReturnType, "->", 37, 39),
            make_token(TokenKind::Identifier, "int", 40, 43),
            make_token(TokenKind::LeftBrace, "{", 44, 45),
            make_token(TokenKind::Return, "return", 46, 52),
            make_token(TokenKind::When, "when", 53, 57),
            make_token(TokenKind::Identifier, "State", 58, 63),
            make_token(TokenKind::Qualification, "::", 63, 65),
            make_token(TokenKind::Identifier, "Ready", 65, 70),
            make_token(TokenKind::LeftBrace, "{", 71, 72),
            make_token(TokenKind::Identifier, "State", 73, 78),
            make_token(TokenKind::Qualification, "::", 78, 80),
            make_token(TokenKind::Identifier, "Ready", 80, 85),
            make_token(TokenKind::Correspondence, "=>", 86, 88),
            make_token(TokenKind::IntegerLiteral, "1", 89, 90),
            make_token(TokenKind::RightBrace, "}", 91, 92),
            make_token(TokenKind::Semicolon, ";", 92, 93),
            make_token(TokenKind::RightBrace, "}", 94, 95),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        assert_eq!(p.declarations.len(), 2);
        match &p.declarations[1] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::When(w) => {
                    match &w.subject.kind {
                        ExpressionKind::EnumConstruction(EnumConstruction::Simple { variant }) => {
                            assert_eq!(variant.qualifier.lexeme, "State");
                            assert_eq!(variant.name.lexeme, "Ready");
                        }
                        _ => panic!("expected EnumConstruction::Simple as when subject"),
                    }
                    assert_eq!(w.correspondences.len(), 1);
                    match &w.correspondences[0].pattern {
                        WhenPattern::Simple { variant } => {
                            assert_eq!(variant.qualifier.lexeme, "State");
                            assert_eq!(variant.name.lexeme, "Ready");
                        }
                        _ => panic!("expected Simple pattern in correspondence"),
                    }
                }
                _ => panic!("expected When expression"),
            },
            _ => panic!("expected Function"),
        }

        // Regression: Structured EnumConstruction
        let source_struct = "public fn run() -> int { return State::Structured { val: 100 }; }";
        let tokens_struct = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Identifier, "State", 32, 37),
            make_token(TokenKind::Qualification, "::", 37, 39),
            make_token(TokenKind::Identifier, "Structured", 39, 49),
            make_token(TokenKind::LeftBrace, "{", 50, 51),
            make_token(TokenKind::Identifier, "val", 52, 55),
            make_token(TokenKind::Colon, ":", 55, 56),
            make_token(TokenKind::IntegerLiteral, "100", 57, 60),
            make_token(TokenKind::RightBrace, "}", 61, 62),
            make_token(TokenKind::Semicolon, ";", 62, 63),
            make_token(TokenKind::RightBrace, "}", 64, 65),
        ];
        let p_struct = unwrap_ok(parse_tokens(&tokens_struct, source_struct));
        match &p_struct.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::EnumConstruction(EnumConstruction::Structured {
                    variant,
                    fields,
                }) => {
                    assert_eq!(variant.qualifier.lexeme, "State");
                    assert_eq!(variant.name.lexeme, "Structured");
                    assert_eq!(fields.len(), 1);
                    assert_eq!(fields[0].name.lexeme, "val");
                }
                _ => panic!("expected EnumConstruction::Structured"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_when_expression_variants() {
        let source = "public fn run() -> int { return when status { Status::Active => 1, Status::Assoc(int32 x) => x, Status::Struct { val: int32 v; } => v }; }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::When, "when", 32, 36),
            make_token(TokenKind::Identifier, "status", 37, 43),
            make_token(TokenKind::LeftBrace, "{", 44, 45),
            make_token(TokenKind::Identifier, "Status", 46, 52),
            make_token(TokenKind::Qualification, "::", 52, 54),
            make_token(TokenKind::Identifier, "Active", 54, 60),
            make_token(TokenKind::Correspondence, "=>", 61, 63),
            make_token(TokenKind::IntegerLiteral, "1", 64, 65),
            make_token(TokenKind::Comma, ",", 65, 66),
            make_token(TokenKind::Identifier, "Status", 67, 73),
            make_token(TokenKind::Qualification, "::", 73, 75),
            make_token(TokenKind::Identifier, "Assoc", 75, 80),
            make_token(TokenKind::LeftParenthesis, "(", 80, 81),
            make_token(TokenKind::Identifier, "int32", 81, 86),
            make_token(TokenKind::Identifier, "x", 87, 88),
            make_token(TokenKind::RightParenthesis, ")", 88, 89),
            make_token(TokenKind::Correspondence, "=>", 90, 92),
            make_token(TokenKind::Identifier, "x", 93, 94),
            make_token(TokenKind::Comma, ",", 94, 95),
            make_token(TokenKind::Identifier, "Status", 96, 102),
            make_token(TokenKind::Qualification, "::", 102, 104),
            make_token(TokenKind::Identifier, "Struct", 104, 110),
            make_token(TokenKind::LeftBrace, "{", 111, 112),
            make_token(TokenKind::Identifier, "val", 113, 116),
            make_token(TokenKind::Colon, ":", 116, 117),
            make_token(TokenKind::Identifier, "int32", 118, 123),
            make_token(TokenKind::Identifier, "v", 124, 125),
            make_token(TokenKind::Semicolon, ";", 125, 126),
            make_token(TokenKind::RightBrace, "}", 127, 128),
            make_token(TokenKind::Correspondence, "=>", 129, 131),
            make_token(TokenKind::Identifier, "v", 132, 133),
            make_token(TokenKind::RightBrace, "}", 134, 135),
            make_token(TokenKind::Semicolon, ";", 135, 136),
            make_token(TokenKind::RightBrace, "}", 137, 138),
        ];

        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::When(w) => {
                    assert_eq!(w.correspondences.len(), 3);
                }
                _ => panic!("expected When"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_all_10_syntax_failures_coverage() {
        // 1. MalformedDeclaration
        let tok1 = vec![make_token(TokenKind::Let, "let", 0, 3)];
        match unwrap_err(parse_tokens(&tok1, "let")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedDeclaration) => {}
            _ => panic!("expected MalformedDeclaration"),
        }

        // 2. MalformedExpression
        let tok2 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::Return, "return", 23, 29),
            make_token(TokenKind::Plus, "+", 30, 31),
            make_token(TokenKind::IntegerLiteral, "1", 32, 33),
            make_token(TokenKind::Semicolon, ";", 33, 34),
            make_token(TokenKind::RightBrace, "}", 35, 36),
        ];
        match unwrap_err(parse_tokens(&tok2, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression) => {}
            _ => panic!("expected MalformedExpression"),
        }

        // 3. InvalidImportPlacement
        let tok3 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::Return, "return", 23, 29),
            make_token(TokenKind::IntegerLiteral, "0", 30, 31),
            make_token(TokenKind::Semicolon, ";", 31, 32),
            make_token(TokenKind::RightBrace, "}", 33, 34),
            make_token(TokenKind::Import, "import", 35, 41),
        ];
        match unwrap_err(parse_tokens(&tok3, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidImportPlacement) => {}
            _ => panic!("expected InvalidImportPlacement"),
        }

        // 4. MissingFinalReturn
        let tok4 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::RightBrace, "}", 22, 23),
        ];
        match unwrap_err(parse_tokens(&tok4, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::MissingFinalReturn) => {}
            _ => panic!("expected MissingFinalReturn"),
        }

        // 5. InvalidReturnPlacement
        let tok5 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::Return, "return", 23, 29),
            make_token(TokenKind::IntegerLiteral, "0", 30, 31),
            make_token(TokenKind::Semicolon, ";", 31, 32),
            make_token(TokenKind::Return, "return", 33, 39),
            make_token(TokenKind::IntegerLiteral, "1", 40, 41),
            make_token(TokenKind::Semicolon, ";", 41, 42),
            make_token(TokenKind::RightBrace, "}", 43, 44),
        ];
        match unwrap_err(parse_tokens(&tok5, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidReturnPlacement) => {}
            _ => panic!("expected InvalidReturnPlacement"),
        }

        // 6. MissingPublicFunction
        let tok6: Vec<Token> = vec![];
        match unwrap_err(parse_tokens(&tok6, "")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::MissingPublicFunction) => {}
            _ => panic!("expected MissingPublicFunction"),
        }

        // 7. MultiplePublicFunctions
        let tok7 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f1", 10, 12),
            make_token(TokenKind::LeftParenthesis, "(", 12, 13),
            make_token(TokenKind::RightParenthesis, ")", 13, 14),
            make_token(TokenKind::ReturnType, "->", 15, 17),
            make_token(TokenKind::Identifier, "int", 18, 21),
            make_token(TokenKind::LeftBrace, "{", 22, 23),
            make_token(TokenKind::Return, "return", 24, 30),
            make_token(TokenKind::IntegerLiteral, "0", 31, 32),
            make_token(TokenKind::Semicolon, ";", 32, 33),
            make_token(TokenKind::RightBrace, "}", 34, 35),
            make_token(TokenKind::Public, "public", 36, 42),
            make_token(TokenKind::Fn, "fn", 43, 45),
            make_token(TokenKind::Identifier, "f2", 46, 48),
            make_token(TokenKind::LeftParenthesis, "(", 48, 49),
            make_token(TokenKind::RightParenthesis, ")", 49, 50),
            make_token(TokenKind::ReturnType, "->", 51, 53),
            make_token(TokenKind::Identifier, "int", 54, 57),
            make_token(TokenKind::LeftBrace, "{", 58, 59),
            make_token(TokenKind::Return, "return", 60, 66),
            make_token(TokenKind::IntegerLiteral, "0", 67, 68),
            make_token(TokenKind::Semicolon, ";", 68, 69),
            make_token(TokenKind::RightBrace, "}", 70, 71),
        ];
        match unwrap_err(parse_tokens(&tok7, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::MultiplePublicFunctions) => {}
            _ => panic!("expected MultiplePublicFunctions"),
        }

        // 8. EmptyEnum
        let tok8 = vec![
            make_token(TokenKind::Enum, "enum", 0, 4),
            make_token(TokenKind::Identifier, "E", 5, 6),
            make_token(TokenKind::LeftBrace, "{", 7, 8),
            make_token(TokenKind::RightBrace, "}", 8, 9),
        ];
        match unwrap_err(parse_tokens(&tok8, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::EmptyEnum) => {}
            _ => panic!("expected EmptyEnum"),
        }

        // 9. InvalidOperationStatement
        let tok9 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::BooleanLiteral, "true", 23, 27),
            make_token(TokenKind::Semicolon, ";", 27, 28),
            make_token(TokenKind::Return, "return", 29, 35),
            make_token(TokenKind::IntegerLiteral, "0", 36, 37),
            make_token(TokenKind::Semicolon, ";", 37, 38),
            make_token(TokenKind::RightBrace, "}", 39, 40),
        ];
        match unwrap_err(parse_tokens(&tok9, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidOperationStatement) => {}
            _ => panic!("expected InvalidOperationStatement"),
        }

        // 10. InvalidThisUsage
        let tok10 = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "f", 10, 11),
            make_token(TokenKind::LeftParenthesis, "(", 11, 12),
            make_token(TokenKind::RightParenthesis, ")", 12, 13),
            make_token(TokenKind::ReturnType, "->", 14, 16),
            make_token(TokenKind::Identifier, "int", 17, 20),
            make_token(TokenKind::LeftBrace, "{", 21, 22),
            make_token(TokenKind::Return, "return", 23, 29),
            make_token(TokenKind::This, "this", 30, 34),
            make_token(TokenKind::Semicolon, ";", 34, 35),
            make_token(TokenKind::RightBrace, "}", 36, 37),
        ];
        match unwrap_err(parse_tokens(&tok10, "source")).kind {
            CompileFailureKind::Syntax(SyntaxFailure::InvalidThisUsage) => {}
            _ => panic!("expected InvalidThisUsage"),
        }
    }

    #[test]
    fn parse_eof_provenance_empty_source() {
        let source = "";
        let tokens: Vec<Token> = vec![];
        let err = unwrap_err(parse_tokens(&tokens, source));
        match err.kind {
            CompileFailureKind::Syntax(SyntaxFailure::MissingPublicFunction) => {
                assert_eq!(err.source_span, SourceSpan { start: 0, end: 0 });
            }
            _ => panic!("expected MissingPublicFunction"),
        }
    }

    #[test]
    fn parse_unary_operators() {
        let source = "public fn run() -> bool { return !flag; }";
        let tokens_not = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "bool", 19, 23),
            make_token(TokenKind::LeftBrace, "{", 24, 25),
            make_token(TokenKind::Return, "return", 26, 32),
            make_token(TokenKind::Not, "!", 33, 34),
            make_token(TokenKind::Identifier, "flag", 34, 38),
            make_token(TokenKind::Semicolon, ";", 38, 39),
            make_token(TokenKind::RightBrace, "}", 40, 41),
        ];
        let p_not = unwrap_ok(parse_tokens(&tokens_not, source));
        match &p_not.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::Unary { operator, operand } => {
                    assert!(matches!(operator, UnaryOperator::Not));
                    match &operand.kind {
                        ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "flag"),
                        _ => panic!("expected Identifier operand"),
                    }
                }
                _ => panic!("expected Unary Not"),
            },
            _ => panic!("expected Function"),
        }

        let source_neg = "public fn run() -> int { return -10; }";
        let tokens_neg = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "int", 19, 22),
            make_token(TokenKind::LeftBrace, "{", 23, 24),
            make_token(TokenKind::Return, "return", 25, 31),
            make_token(TokenKind::Minus, "-", 32, 33),
            make_token(TokenKind::IntegerLiteral, "10", 33, 35),
            make_token(TokenKind::Semicolon, ";", 35, 36),
            make_token(TokenKind::RightBrace, "}", 37, 38),
        ];
        let p_neg = unwrap_ok(parse_tokens(&tokens_neg, source_neg));
        match &p_neg.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::Unary { operator, operand } => {
                    assert!(matches!(operator, UnaryOperator::Negate));
                    match &operand.kind {
                        ExpressionKind::Literal { lexeme, kind } => {
                            assert_eq!(*lexeme, "10");
                            assert!(matches!(kind, LiteralKind::Integer));
                        }
                        _ => panic!("expected IntegerLiteral operand"),
                    }
                }
                _ => panic!("expected Unary Negate"),
            },
            _ => panic!("expected Function"),
        }
    }

    #[test]
    fn parse_associated_enum_construction() {
        let source = "public fn run() -> Status { return Status::Assoc(42); }";
        let tokens = vec![
            make_token(TokenKind::Public, "public", 0, 6),
            make_token(TokenKind::Fn, "fn", 7, 9),
            make_token(TokenKind::Identifier, "run", 10, 13),
            make_token(TokenKind::LeftParenthesis, "(", 13, 14),
            make_token(TokenKind::RightParenthesis, ")", 14, 15),
            make_token(TokenKind::ReturnType, "->", 16, 18),
            make_token(TokenKind::Identifier, "Status", 19, 25),
            make_token(TokenKind::LeftBrace, "{", 26, 27),
            make_token(TokenKind::Return, "return", 28, 34),
            make_token(TokenKind::Identifier, "Status", 35, 41),
            make_token(TokenKind::Qualification, "::", 41, 43),
            make_token(TokenKind::Identifier, "Assoc", 43, 48),
            make_token(TokenKind::LeftParenthesis, "(", 48, 49),
            make_token(TokenKind::IntegerLiteral, "42", 49, 51),
            make_token(TokenKind::RightParenthesis, ")", 51, 52),
            make_token(TokenKind::Semicolon, ";", 52, 53),
            make_token(TokenKind::RightBrace, "}", 54, 55),
        ];
        let p = unwrap_ok(parse_tokens(&tokens, source));
        match &p.declarations[0] {
            Declaration::Function(f) => match &f.body.result.kind {
                ExpressionKind::EnumConstruction(EnumConstruction::Associated {
                    variant,
                    value,
                }) => {
                    assert_eq!(variant.qualifier.lexeme, "Status");
                    assert_eq!(variant.name.lexeme, "Assoc");
                    match &value.kind {
                        ExpressionKind::Literal { lexeme, kind } => {
                            assert_eq!(*lexeme, "42");
                            assert!(matches!(kind, LiteralKind::Integer));
                        }
                        _ => panic!("expected Literal value"),
                    }
                }
                _ => panic!("expected EnumConstruction::Associated"),
            },
            _ => panic!("expected Function"),
        }
    }
}
