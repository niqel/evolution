use alloc::boxed::Box;
use alloc::vec::Vec;

use evo_values::OwnedValue;

use crate::data::ast::expressions::{BinaryOperator, UnaryOperator};
use crate::data::compilation_dependency::TypeSymbol;
use crate::data::compiled::program::CompiledProgram;
use crate::data::lexical::SourceSpan;
use crate::data::semantic::SignatureSymbol;
use crate::data::semantic::structure::NativeType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexicalFailure {
    UnrecognizedCharacter(char),
    InvalidIdentifier,
    MalformedNumericLiteral,
    UnterminatedStringLiteral,
    InvalidStringEscape(char),
    PhysicalNewlineInStringLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxFailure {
    MalformedDeclaration,
    MalformedExpression,
    InvalidImportPlacement,
    MissingFinalReturn,
    InvalidReturnPlacement,
    MissingPublicFunction,
    MultiplePublicFunctions,
    EmptyEnum,
    InvalidOperationStatement,
    InvalidThisUsage,
}

pub(crate) enum SemanticFailure {
    Resolution(ResolutionFailure),
    Declaration(DeclarationFailure),
    TypeChecking(TypeCheckingFailure),
    Call(CallFailure),
    Composite(CompositeFailure),
    When(WhenFailure),
    SignatureMismatch {
        signature: SignatureSymbol,
        mismatch: SignatureMismatchKind,
    },
}

pub(crate) enum ResolutionFailure {
    ImportedSymbolNotFound { module: Box<str>, name: Box<str> },
    UnknownType { name: Box<str> },
    UnknownValueSymbol { name: Box<str> },
    UnknownSignature(SignatureSymbol),
}

pub(crate) enum DeclarationFailure {
    TypeNameCollision { name: Box<str> },
    DuplicateFunction { name: Box<str> },
    DuplicateField { name: Box<str> },
    DuplicateVariant { name: Box<str> },
    BindingNameCollision { name: Box<str> },
    InvalidNamingConvention { role: SemanticNameRole },
    RecursiveTypeCycle,
}

pub(crate) enum SemanticNameRole {
    Type,
    Variant,
    Field,
    Function,
    Binding,
    SignatureAlias,
    SignatureDependency,
}

pub(crate) enum SemanticTypeDescriptor {
    Native(NativeType),
    Local(Box<str>),
    Shared(TypeSymbol),
}

pub(crate) enum TypeCheckingFailure {
    BindingInitialization {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    FunctionResult {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    NumericLiteralNotRepresentable {
        expected: SemanticTypeDescriptor,
    },
    UnaryOperator {
        operator: UnaryOperator,
        operand: SemanticTypeDescriptor,
    },
    ArithmeticOperator {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },
    LogicalOperator {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },
    Comparison {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },
    InvalidConversion {
        source: SemanticTypeDescriptor,
        target: SemanticTypeDescriptor,
    },
}

pub(crate) enum SemanticArgumentKind {
    Value,
    SignatureDependency,
}

pub(crate) enum CallFailure {
    FunctionNotFound {
        name: Box<str>,
    },
    AmbiguousTarget {
        name: Box<str>,
    },
    ArityMismatch {
        expected: usize,
        actual: usize,
    },
    ArgumentKindMismatch {
        position: usize,
        expected: SemanticArgumentKind,
        actual: SemanticArgumentKind,
    },
    ArgumentTypeMismatch {
        position: usize,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    SignatureDependencyMismatch {
        position: usize,
        expected: SignatureSymbol,
        actual: SignatureSymbol,
    },
    FunctionCallCycle,
}

pub(crate) enum EnumPayloadShape {
    Simple,
    Associated,
    Structured,
}

pub(crate) enum CompositeFailure {
    ExpectedStruct {
        actual: SemanticTypeDescriptor,
    },
    ExpectedEnum {
        actual: SemanticTypeDescriptor,
    },
    FieldAccessType {
        actual: SemanticTypeDescriptor,
    },
    FieldNotFound {
        field: Box<str>,
    },
    MissingField {
        field: Box<str>,
    },
    DuplicateFieldInitializer {
        field: Box<str>,
    },
    FieldTypeMismatch {
        field: Box<str>,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    VariantNotFound {
        variant: Box<str>,
    },
    VariantPayloadShapeMismatch {
        expected: EnumPayloadShape,
        actual: EnumPayloadShape,
    },
    AssociatedPayloadTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}

pub(crate) enum WhenFailure {
    SubjectNotEnum {
        actual: SemanticTypeDescriptor,
    },
    PatternEnumMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    VariantNotFound {
        variant: Box<str>,
    },
    DuplicateVariantCorrespondence {
        variant: Box<str>,
    },
    NonExhaustive {
        missing: Vec<Box<str>>,
    },
    PayloadShapeMismatch {
        expected: EnumPayloadShape,
        actual: EnumPayloadShape,
    },
    FieldNotFound {
        field: Box<str>,
    },
    DuplicateField {
        field: Box<str>,
    },
    MissingField {
        field: Box<str>,
    },
    ExtractionTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    BranchResultTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}

pub(crate) enum SignatureMismatchKind {
    FunctionName,
    ParameterCount {
        expected: usize,
        actual: usize,
    },
    ParameterKind {
        position: usize,
        expected: SemanticArgumentKind,
        actual: SemanticArgumentKind,
    },
    ValueParameterType {
        position: usize,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
    SignatureDependency {
        position: usize,
        expected: SignatureSymbol,
        actual: SignatureSymbol,
    },
    ResultType {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}

pub(crate) struct ExternalCapabilityFailure {
    pub(crate) code: Box<str>,
}

pub(crate) type CompileOutcome = Result<CompiledProgram, CompileFailure>;

pub(crate) struct CompileFailure {
    pub(crate) kind: CompileFailureKind,
    pub(crate) source_span: SourceSpan,
}

pub(crate) enum CompileFailureKind {
    Lexical(LexicalFailure),
    Syntax(SyntaxFailure),
    Semantic(SemanticFailure),
}

pub(crate) type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;

pub(crate) struct ExecutionFailure {
    pub(crate) kind: ExecutionFailureKind,
    pub(crate) source_span: Option<SourceSpan>,
}

pub(crate) enum ExecutionFailureKind {
    Compilation(CompileFailureKind),
    Invocation(InvocationFailure),
    Evaluation(EvaluationFailure),
    External(ExternalExecutionFailure),
}

pub(crate) enum InvocationFailure {
    ArityMismatch { expected: usize, actual: usize },
    ArgumentShapeMismatch { position: usize },
}

pub(crate) enum EvaluationFailure {
    Overflow,
    DivisionByZero,
    Conversion,
    DynamicNumericType,
}

pub(crate) enum ExternalExecutionFailure {
    MissingBinding {
        signature: SignatureSymbol,
    },
    CapabilityFailure {
        signature: SignatureSymbol,
        failure: ExternalCapabilityFailure,
    },
    ResultContractMismatch {
        signature: SignatureSymbol,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn lexical_failure_inventory_has_exactly_6_variants_and_payloads() {
        let all_lexical: [LexicalFailure; 6] = [
            LexicalFailure::UnrecognizedCharacter('@'),
            LexicalFailure::InvalidIdentifier,
            LexicalFailure::MalformedNumericLiteral,
            LexicalFailure::UnterminatedStringLiteral,
            LexicalFailure::InvalidStringEscape('x'),
            LexicalFailure::PhysicalNewlineInStringLiteral,
        ];

        assert_eq!(all_lexical.len(), 6);

        // Verify payloads are preserved
        assert_eq!(all_lexical[0], LexicalFailure::UnrecognizedCharacter('@'));
        assert_ne!(all_lexical[0], LexicalFailure::UnrecognizedCharacter('#'));
        assert_eq!(all_lexical[4], LexicalFailure::InvalidStringEscape('x'));
        assert_ne!(all_lexical[4], LexicalFailure::InvalidStringEscape('u'));

        // Copy and equality semantics
        let copy = all_lexical[0];
        assert_eq!(all_lexical[0], copy);
    }

    #[test]
    fn syntax_failure_inventory_has_exactly_10_variants() {
        let all_syntax: [SyntaxFailure; 10] = [
            SyntaxFailure::MalformedDeclaration,
            SyntaxFailure::MalformedExpression,
            SyntaxFailure::InvalidImportPlacement,
            SyntaxFailure::MissingFinalReturn,
            SyntaxFailure::InvalidReturnPlacement,
            SyntaxFailure::MissingPublicFunction,
            SyntaxFailure::MultiplePublicFunctions,
            SyntaxFailure::EmptyEnum,
            SyntaxFailure::InvalidOperationStatement,
            SyntaxFailure::InvalidThisUsage,
        ];

        assert_eq!(all_syntax.len(), 10);
        assert_eq!(all_syntax[0], SyntaxFailure::MalformedDeclaration);
        assert_eq!(all_syntax[9], SyntaxFailure::InvalidThisUsage);

        // Copy and equality semantics
        let copy = all_syntax[0];
        assert_eq!(all_syntax[0], copy);
    }

    #[test]
    fn semantic_failure_root_7_variants() {
        let f_res =
            SemanticFailure::Resolution(ResolutionFailure::UnknownType { name: "Foo".into() });
        match f_res {
            SemanticFailure::Resolution(ResolutionFailure::UnknownType { name }) => {
                assert_eq!(&*name, "Foo");
            }
            _ => panic!("expected Resolution"),
        }

        let f_decl = SemanticFailure::Declaration(DeclarationFailure::RecursiveTypeCycle);
        match f_decl {
            SemanticFailure::Declaration(DeclarationFailure::RecursiveTypeCycle) => {}
            _ => panic!("expected Declaration"),
        }

        let f_tc =
            SemanticFailure::TypeChecking(TypeCheckingFailure::NumericLiteralNotRepresentable {
                expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            });
        match f_tc {
            SemanticFailure::TypeChecking(
                TypeCheckingFailure::NumericLiteralNotRepresentable {
                    expected: SemanticTypeDescriptor::Native(NativeType::Int32),
                },
            ) => {}
            _ => panic!("expected TypeChecking"),
        }

        let f_call = SemanticFailure::Call(CallFailure::FunctionCallCycle);
        match f_call {
            SemanticFailure::Call(CallFailure::FunctionCallCycle) => {}
            _ => panic!("expected Call"),
        }

        let f_comp =
            SemanticFailure::Composite(CompositeFailure::FieldNotFound { field: "x".into() });
        match f_comp {
            SemanticFailure::Composite(CompositeFailure::FieldNotFound { field }) => {
                assert_eq!(&*field, "x");
            }
            _ => panic!("expected Composite"),
        }

        let f_when = SemanticFailure::When(WhenFailure::VariantNotFound {
            variant: "V".into(),
        });
        match f_when {
            SemanticFailure::When(WhenFailure::VariantNotFound { variant }) => {
                assert_eq!(&*variant, "V");
            }
            _ => panic!("expected When"),
        }

        let f_sig = SemanticFailure::SignatureMismatch {
            signature: SignatureSymbol {
                module: "Math".to_string(),
                name: "Add".to_string(),
            },
            mismatch: SignatureMismatchKind::FunctionName,
        };
        match f_sig {
            SemanticFailure::SignatureMismatch {
                signature,
                mismatch: SignatureMismatchKind::FunctionName,
            } => {
                assert_eq!(signature.module, "Math");
                assert_eq!(signature.name, "Add");
            }
            _ => panic!("expected SignatureMismatch"),
        }
    }

    #[test]
    fn resolution_failure_4_variants() {
        let r1 = ResolutionFailure::ImportedSymbolNotFound {
            module: "Math".into(),
            name: "Sqrt".into(),
        };
        match r1 {
            ResolutionFailure::ImportedSymbolNotFound { module, name } => {
                assert_eq!(&*module, "Math");
                assert_eq!(&*name, "Sqrt");
            }
            _ => panic!("expected ImportedSymbolNotFound"),
        }

        let r2 = ResolutionFailure::UnknownType {
            name: "MissingType".into(),
        };
        match r2 {
            ResolutionFailure::UnknownType { name } => assert_eq!(&*name, "MissingType"),
            _ => panic!("expected UnknownType"),
        }

        let r3 = ResolutionFailure::UnknownValueSymbol {
            name: "missing_var".into(),
        };
        match r3 {
            ResolutionFailure::UnknownValueSymbol { name } => {
                assert_eq!(&*name, "missing_var");
            }
            _ => panic!("expected UnknownValueSymbol"),
        }

        let r4 = ResolutionFailure::UnknownSignature(SignatureSymbol {
            module: "Core".to_string(),
            name: "Run".to_string(),
        });
        match r4 {
            ResolutionFailure::UnknownSignature(sym) => {
                assert_eq!(sym.module, "Core");
                assert_eq!(sym.name, "Run");
            }
            _ => panic!("expected UnknownSignature"),
        }
    }

    #[test]
    fn declaration_failure_7_variants() {
        let d1 = DeclarationFailure::TypeNameCollision { name: "Foo".into() };
        match d1 {
            DeclarationFailure::TypeNameCollision { name } => assert_eq!(&*name, "Foo"),
            _ => panic!("expected TypeNameCollision"),
        }

        let d2 = DeclarationFailure::DuplicateFunction { name: "bar".into() };
        match d2 {
            DeclarationFailure::DuplicateFunction { name } => assert_eq!(&*name, "bar"),
            _ => panic!("expected DuplicateFunction"),
        }

        let d3 = DeclarationFailure::DuplicateField { name: "id".into() };
        match d3 {
            DeclarationFailure::DuplicateField { name } => assert_eq!(&*name, "id"),
            _ => panic!("expected DuplicateField"),
        }

        let d4 = DeclarationFailure::DuplicateVariant {
            name: "Active".into(),
        };
        match d4 {
            DeclarationFailure::DuplicateVariant { name } => assert_eq!(&*name, "Active"),
            _ => panic!("expected DuplicateVariant"),
        }

        let d5 = DeclarationFailure::BindingNameCollision { name: "x".into() };
        match d5 {
            DeclarationFailure::BindingNameCollision { name } => assert_eq!(&*name, "x"),
            _ => panic!("expected BindingNameCollision"),
        }

        let d6 = DeclarationFailure::InvalidNamingConvention {
            role: SemanticNameRole::Type,
        };
        match d6 {
            DeclarationFailure::InvalidNamingConvention { role } => match role {
                SemanticNameRole::Type => {}
                _ => panic!("expected Type role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }

        let d7 = DeclarationFailure::RecursiveTypeCycle;
        match d7 {
            DeclarationFailure::RecursiveTypeCycle => {}
            _ => panic!("expected RecursiveTypeCycle"),
        }
    }

    #[test]
    fn semantic_name_role_7_variants() {
        let roles = [
            SemanticNameRole::Type,
            SemanticNameRole::Variant,
            SemanticNameRole::Field,
            SemanticNameRole::Function,
            SemanticNameRole::Binding,
            SemanticNameRole::SignatureAlias,
            SemanticNameRole::SignatureDependency,
        ];
        assert_eq!(roles.len(), 7);
        match roles[0] {
            SemanticNameRole::Type => {}
            _ => panic!("expected Type"),
        }
        match roles[6] {
            SemanticNameRole::SignatureDependency => {}
            _ => panic!("expected SignatureDependency"),
        }
    }

    #[test]
    fn semantic_type_descriptor_3_variants() {
        let t_nat = SemanticTypeDescriptor::Native(NativeType::Int32);
        match t_nat {
            SemanticTypeDescriptor::Native(NativeType::Int32) => {}
            _ => panic!("expected Native(Int32)"),
        }

        let t_loc = SemanticTypeDescriptor::Local("MyCustomType".into());
        match t_loc {
            SemanticTypeDescriptor::Local(name) => assert_eq!(&*name, "MyCustomType"),
            _ => panic!("expected Local"),
        }

        let t_sh = SemanticTypeDescriptor::Shared(TypeSymbol {
            module: "Core".to_string(),
            name: "User".to_string(),
        });
        match t_sh {
            SemanticTypeDescriptor::Shared(sym) => {
                assert_eq!(sym.module, "Core");
                assert_eq!(sym.name, "User");
            }
            _ => panic!("expected Shared"),
        }
    }

    #[test]
    fn type_checking_failure_8_variants() {
        let tc1 = TypeCheckingFailure::BindingInitialization {
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match tc1 {
            TypeCheckingFailure::BindingInitialization { expected, actual } => {
                match (expected, actual) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int32),
                        SemanticTypeDescriptor::Native(NativeType::String),
                    ) => {}
                    _ => panic!("unexpected descriptors"),
                }
            }
            _ => panic!("expected BindingInitialization"),
        }

        let tc2 = TypeCheckingFailure::FunctionResult {
            expected: SemanticTypeDescriptor::Native(NativeType::Bool),
            actual: SemanticTypeDescriptor::Native(NativeType::Int32),
        };
        match tc2 {
            TypeCheckingFailure::FunctionResult { .. } => {}
            _ => panic!("expected FunctionResult"),
        }

        let tc3 = TypeCheckingFailure::NumericLiteralNotRepresentable {
            expected: SemanticTypeDescriptor::Native(NativeType::Uint8),
        };
        match tc3 {
            TypeCheckingFailure::NumericLiteralNotRepresentable { .. } => {}
            _ => panic!("expected NumericLiteralNotRepresentable"),
        }

        let tc4 = TypeCheckingFailure::UnaryOperator {
            operator: UnaryOperator::Not,
            operand: SemanticTypeDescriptor::Native(NativeType::Int32),
        };
        match tc4 {
            TypeCheckingFailure::UnaryOperator { operator, .. } => match operator {
                UnaryOperator::Not => {}
                _ => panic!("expected Not"),
            },
            _ => panic!("expected UnaryOperator"),
        }

        let tc5 = TypeCheckingFailure::ArithmeticOperator {
            operator: BinaryOperator::Add,
            left: SemanticTypeDescriptor::Native(NativeType::Int32),
            right: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match tc5 {
            TypeCheckingFailure::ArithmeticOperator { operator, .. } => match operator {
                BinaryOperator::Add => {}
                _ => panic!("expected Add"),
            },
            _ => panic!("expected ArithmeticOperator"),
        }

        let tc6 = TypeCheckingFailure::LogicalOperator {
            operator: BinaryOperator::And,
            left: SemanticTypeDescriptor::Native(NativeType::Int32),
            right: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match tc6 {
            TypeCheckingFailure::LogicalOperator { operator, .. } => match operator {
                BinaryOperator::And => {}
                _ => panic!("expected And"),
            },
            _ => panic!("expected LogicalOperator"),
        }

        let tc7 = TypeCheckingFailure::Comparison {
            operator: BinaryOperator::Equal,
            left: SemanticTypeDescriptor::Native(NativeType::Int32),
            right: SemanticTypeDescriptor::Native(NativeType::Float64),
        };
        match tc7 {
            TypeCheckingFailure::Comparison { operator, .. } => match operator {
                BinaryOperator::Equal => {}
                _ => panic!("expected Equal"),
            },
            _ => panic!("expected Comparison"),
        }

        let tc8 = TypeCheckingFailure::InvalidConversion {
            source: SemanticTypeDescriptor::Native(NativeType::String),
            target: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match tc8 {
            TypeCheckingFailure::InvalidConversion { .. } => {}
            _ => panic!("expected InvalidConversion"),
        }
    }

    #[test]
    fn semantic_argument_kind_2_variants() {
        let kinds = [
            SemanticArgumentKind::Value,
            SemanticArgumentKind::SignatureDependency,
        ];
        assert_eq!(kinds.len(), 2);
        match kinds[0] {
            SemanticArgumentKind::Value => {}
            _ => panic!("expected Value"),
        }
        match kinds[1] {
            SemanticArgumentKind::SignatureDependency => {}
            _ => panic!("expected SignatureDependency"),
        }
    }

    #[test]
    fn call_failure_7_variants() {
        let c1 = CallFailure::FunctionNotFound {
            name: "missing_fn".into(),
        };
        match c1 {
            CallFailure::FunctionNotFound { name } => assert_eq!(&*name, "missing_fn"),
            _ => panic!("expected FunctionNotFound"),
        }

        let c2 = CallFailure::AmbiguousTarget {
            name: "ambiguous_fn".into(),
        };
        match c2 {
            CallFailure::AmbiguousTarget { name } => assert_eq!(&*name, "ambiguous_fn"),
            _ => panic!("expected AmbiguousTarget"),
        }

        let c3 = CallFailure::ArityMismatch {
            expected: 2,
            actual: 3,
        };
        match c3 {
            CallFailure::ArityMismatch { expected, actual } => {
                assert_eq!(expected, 2);
                assert_eq!(actual, 3);
            }
            _ => panic!("expected ArityMismatch"),
        }

        let c4 = CallFailure::ArgumentKindMismatch {
            position: 1,
            expected: SemanticArgumentKind::Value,
            actual: SemanticArgumentKind::SignatureDependency,
        };
        match c4 {
            CallFailure::ArgumentKindMismatch {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 1);
                match (expected, actual) {
                    (SemanticArgumentKind::Value, SemanticArgumentKind::SignatureDependency) => {}
                    _ => panic!("unexpected kinds"),
                }
            }
            _ => panic!("expected ArgumentKindMismatch"),
        }

        let c5 = CallFailure::ArgumentTypeMismatch {
            position: 0,
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match c5 {
            CallFailure::ArgumentTypeMismatch {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 0);
                match (expected, actual) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int32),
                        SemanticTypeDescriptor::Native(NativeType::String),
                    ) => {}
                    _ => panic!("unexpected descriptors"),
                }
            }
            _ => panic!("expected ArgumentTypeMismatch"),
        }

        let c6 = CallFailure::SignatureDependencyMismatch {
            position: 2,
            expected: SignatureSymbol {
                module: "Math".to_string(),
                name: "Adder".to_string(),
            },
            actual: SignatureSymbol {
                module: "Math".to_string(),
                name: "Multiplier".to_string(),
            },
        };
        match c6 {
            CallFailure::SignatureDependencyMismatch {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 2);
                assert_eq!(expected.name, "Adder");
                assert_eq!(actual.name, "Multiplier");
            }
            _ => panic!("expected SignatureDependencyMismatch"),
        }

        let c7 = CallFailure::FunctionCallCycle;
        match c7 {
            CallFailure::FunctionCallCycle => {}
            _ => panic!("expected FunctionCallCycle"),
        }
    }

    #[test]
    fn enum_payload_shape_3_variants() {
        let shapes = [
            EnumPayloadShape::Simple,
            EnumPayloadShape::Associated,
            EnumPayloadShape::Structured,
        ];
        assert_eq!(shapes.len(), 3);
        match shapes[0] {
            EnumPayloadShape::Simple => {}
            _ => panic!("expected Simple"),
        }
        match shapes[1] {
            EnumPayloadShape::Associated => {}
            _ => panic!("expected Associated"),
        }
        match shapes[2] {
            EnumPayloadShape::Structured => {}
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn composite_failure_10_variants() {
        let cp1 = CompositeFailure::ExpectedStruct {
            actual: SemanticTypeDescriptor::Native(NativeType::Int32),
        };
        match cp1 {
            CompositeFailure::ExpectedStruct { .. } => {}
            _ => panic!("expected ExpectedStruct"),
        }

        let cp2 = CompositeFailure::ExpectedEnum {
            actual: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match cp2 {
            CompositeFailure::ExpectedEnum { .. } => {}
            _ => panic!("expected ExpectedEnum"),
        }

        let cp3 = CompositeFailure::FieldAccessType {
            actual: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match cp3 {
            CompositeFailure::FieldAccessType { .. } => {}
            _ => panic!("expected FieldAccessType"),
        }

        let cp4 = CompositeFailure::FieldNotFound {
            field: "age".into(),
        };
        match cp4 {
            CompositeFailure::FieldNotFound { field } => assert_eq!(&*field, "age"),
            _ => panic!("expected FieldNotFound"),
        }

        let cp5 = CompositeFailure::MissingField { field: "id".into() };
        match cp5 {
            CompositeFailure::MissingField { field } => assert_eq!(&*field, "id"),
            _ => panic!("expected MissingField"),
        }

        let cp6 = CompositeFailure::DuplicateFieldInitializer {
            field: "tag".into(),
        };
        match cp6 {
            CompositeFailure::DuplicateFieldInitializer { field } => assert_eq!(&*field, "tag"),
            _ => panic!("expected DuplicateFieldInitializer"),
        }

        let cp7 = CompositeFailure::FieldTypeMismatch {
            field: "count".into(),
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match cp7 {
            CompositeFailure::FieldTypeMismatch { field, .. } => assert_eq!(&*field, "count"),
            _ => panic!("expected FieldTypeMismatch"),
        }

        let cp8 = CompositeFailure::VariantNotFound {
            variant: "Pending".into(),
        };
        match cp8 {
            CompositeFailure::VariantNotFound { variant } => assert_eq!(&*variant, "Pending"),
            _ => panic!("expected VariantNotFound"),
        }

        let cp9 = CompositeFailure::VariantPayloadShapeMismatch {
            expected: EnumPayloadShape::Simple,
            actual: EnumPayloadShape::Structured,
        };
        match cp9 {
            CompositeFailure::VariantPayloadShapeMismatch { expected, actual } => {
                match (expected, actual) {
                    (EnumPayloadShape::Simple, EnumPayloadShape::Structured) => {}
                    _ => panic!("unexpected shapes"),
                }
            }
            _ => panic!("expected VariantPayloadShapeMismatch"),
        }

        let cp10 = CompositeFailure::AssociatedPayloadTypeMismatch {
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match cp10 {
            CompositeFailure::AssociatedPayloadTypeMismatch { .. } => {}
            _ => panic!("expected AssociatedPayloadTypeMismatch"),
        }
    }

    #[test]
    fn when_failure_11_variants() {
        let w1 = WhenFailure::SubjectNotEnum {
            actual: SemanticTypeDescriptor::Native(NativeType::Int32),
        };
        match w1 {
            WhenFailure::SubjectNotEnum { .. } => {}
            _ => panic!("expected SubjectNotEnum"),
        }

        let w2 = WhenFailure::PatternEnumMismatch {
            expected: SemanticTypeDescriptor::Local("EnumA".into()),
            actual: SemanticTypeDescriptor::Local("EnumB".into()),
        };
        match w2 {
            WhenFailure::PatternEnumMismatch { expected, actual } => match (expected, actual) {
                (SemanticTypeDescriptor::Local(e), SemanticTypeDescriptor::Local(a)) => {
                    assert_eq!(&*e, "EnumA");
                    assert_eq!(&*a, "EnumB");
                }
                _ => panic!("unexpected descriptors"),
            },
            _ => panic!("expected PatternEnumMismatch"),
        }

        let w3 = WhenFailure::VariantNotFound {
            variant: "NotFound".into(),
        };
        match w3 {
            WhenFailure::VariantNotFound { variant } => assert_eq!(&*variant, "NotFound"),
            _ => panic!("expected VariantNotFound"),
        }

        let w4 = WhenFailure::DuplicateVariantCorrespondence {
            variant: "Active".into(),
        };
        match w4 {
            WhenFailure::DuplicateVariantCorrespondence { variant } => {
                assert_eq!(&*variant, "Active");
            }
            _ => panic!("expected DuplicateVariantCorrespondence"),
        }

        let w5 = WhenFailure::NonExhaustive {
            missing: alloc::vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
        };
        match w5 {
            WhenFailure::NonExhaustive { missing } => {
                assert_eq!(missing.len(), 3);
                assert_eq!(&*missing[0], "Alpha");
                assert_eq!(&*missing[1], "Beta");
                assert_eq!(&*missing[2], "Gamma");
            }
            _ => panic!("expected NonExhaustive"),
        }

        let w6 = WhenFailure::PayloadShapeMismatch {
            expected: EnumPayloadShape::Associated,
            actual: EnumPayloadShape::Simple,
        };
        match w6 {
            WhenFailure::PayloadShapeMismatch { expected, actual } => match (expected, actual) {
                (EnumPayloadShape::Associated, EnumPayloadShape::Simple) => {}
                _ => panic!("unexpected shapes"),
            },
            _ => panic!("expected PayloadShapeMismatch"),
        }

        let w7 = WhenFailure::FieldNotFound {
            field: "field1".into(),
        };
        match w7 {
            WhenFailure::FieldNotFound { field } => assert_eq!(&*field, "field1"),
            _ => panic!("expected FieldNotFound"),
        }

        let w8 = WhenFailure::DuplicateField {
            field: "dup_field".into(),
        };
        match w8 {
            WhenFailure::DuplicateField { field } => assert_eq!(&*field, "dup_field"),
            _ => panic!("expected DuplicateField"),
        }

        let w9 = WhenFailure::MissingField {
            field: "missing_field".into(),
        };
        match w9 {
            WhenFailure::MissingField { field } => assert_eq!(&*field, "missing_field"),
            _ => panic!("expected MissingField"),
        }

        let w10 = WhenFailure::ExtractionTypeMismatch {
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::String),
        };
        match w10 {
            WhenFailure::ExtractionTypeMismatch { .. } => {}
            _ => panic!("expected ExtractionTypeMismatch"),
        }

        let w11 = WhenFailure::BranchResultTypeMismatch {
            expected: SemanticTypeDescriptor::Native(NativeType::Int64),
            actual: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match w11 {
            WhenFailure::BranchResultTypeMismatch { .. } => {}
            _ => panic!("expected BranchResultTypeMismatch"),
        }
    }

    #[test]
    fn signature_mismatch_kind_6_variants() {
        let sm1 = SignatureMismatchKind::FunctionName;
        match sm1 {
            SignatureMismatchKind::FunctionName => {}
            _ => panic!("expected FunctionName"),
        }

        let sm2 = SignatureMismatchKind::ParameterCount {
            expected: 2,
            actual: 1,
        };
        match sm2 {
            SignatureMismatchKind::ParameterCount { expected, actual } => {
                assert_eq!(expected, 2);
                assert_eq!(actual, 1);
            }
            _ => panic!("expected ParameterCount"),
        }

        let sm3 = SignatureMismatchKind::ParameterKind {
            position: 0,
            expected: SemanticArgumentKind::Value,
            actual: SemanticArgumentKind::SignatureDependency,
        };
        match sm3 {
            SignatureMismatchKind::ParameterKind {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 0);
                match (expected, actual) {
                    (SemanticArgumentKind::Value, SemanticArgumentKind::SignatureDependency) => {}
                    _ => panic!("unexpected kinds"),
                }
            }
            _ => panic!("expected ParameterKind"),
        }

        let sm4 = SignatureMismatchKind::ValueParameterType {
            position: 1,
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::Float64),
        };
        match sm4 {
            SignatureMismatchKind::ValueParameterType {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 1);
                match (expected, actual) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int32),
                        SemanticTypeDescriptor::Native(NativeType::Float64),
                    ) => {}
                    _ => panic!("unexpected descriptors"),
                }
            }
            _ => panic!("expected ValueParameterType"),
        }

        let sm5 = SignatureMismatchKind::SignatureDependency {
            position: 2,
            expected: SignatureSymbol {
                module: "Math".to_string(),
                name: "Foo".to_string(),
            },
            actual: SignatureSymbol {
                module: "Math".to_string(),
                name: "Bar".to_string(),
            },
        };
        match sm5 {
            SignatureMismatchKind::SignatureDependency {
                position,
                expected,
                actual,
            } => {
                assert_eq!(position, 2);
                assert_eq!(expected.name, "Foo");
                assert_eq!(actual.name, "Bar");
            }
            _ => panic!("expected SignatureDependency"),
        }

        let sm6 = SignatureMismatchKind::ResultType {
            expected: SemanticTypeDescriptor::Native(NativeType::Int32),
            actual: SemanticTypeDescriptor::Native(NativeType::Bool),
        };
        match sm6 {
            SignatureMismatchKind::ResultType { expected, actual } => match (expected, actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int32),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("unexpected descriptors"),
            },
            _ => panic!("expected ResultType"),
        }
    }

    #[test]
    fn external_capability_failure_field() {
        let failure = ExternalCapabilityFailure {
            code: Box::from("permission_denied"),
        };
        assert_eq!(&*failure.code, "permission_denied");
    }

    #[test]
    fn compile_failure_kind_3_variants() {
        let kind_lex = CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier);
        match kind_lex {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected Lexical"),
        }

        let kind_syn = CompileFailureKind::Syntax(SyntaxFailure::MalformedDeclaration);
        match kind_syn {
            CompileFailureKind::Syntax(SyntaxFailure::MalformedDeclaration) => {}
            _ => panic!("expected Syntax"),
        }

        let kind_sem = CompileFailureKind::Semantic(SemanticFailure::Resolution(
            ResolutionFailure::UnknownType {
                name: Box::from("Foo"),
            },
        ));
        match kind_sem {
            CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::UnknownType { name },
            )) => assert_eq!(&*name, "Foo"),
            _ => panic!("expected Semantic"),
        }
    }

    #[test]
    fn compile_failure_mandatory_provenance_and_zero_width() {
        let failure_span = CompileFailure {
            kind: CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier),
            source_span: SourceSpan { start: 10, end: 15 },
        };
        assert_eq!(failure_span.source_span.start, 10);
        assert_eq!(failure_span.source_span.end, 15);

        let failure_zero_width = CompileFailure {
            kind: CompileFailureKind::Syntax(SyntaxFailure::MissingFinalReturn),
            source_span: SourceSpan { start: 20, end: 20 },
        };
        assert_eq!(failure_zero_width.source_span.start, 20);
        assert_eq!(failure_zero_width.source_span.end, 20);
    }

    #[test]
    fn compile_outcome_ok_and_err() {
        use crate::data::compiled::identities::ConstantId;
        use crate::data::compiled::instructions::Instruction;
        use crate::data::compiled::program::CompiledFunction;
        use crate::data::compiled::source_map::SourceMap;
        use crate::data::compiled::storage::Constant;
        use crate::data::semantic::ids::FunctionId;

        let program = CompiledProgram {
            functions: alloc::vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: alloc::vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: alloc::vec![],
            constants: alloc::vec![Constant::Boolean(true)],
            external_symbols: alloc::vec![],
            value_shapes: alloc::vec![],
            source_map: SourceMap {
                functions: alloc::vec![alloc::vec![
                    SourceSpan { start: 0, end: 5 },
                    SourceSpan { start: 5, end: 10 },
                ]],
            },
        };

        let ok_outcome: CompileOutcome = Ok(program);
        match ok_outcome {
            Ok(p) => assert_eq!(p.functions.len(), 1),
            Err(_) => panic!("expected Ok"),
        }

        let err_outcome: CompileOutcome = Err(CompileFailure {
            kind: CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression),
            source_span: SourceSpan { start: 0, end: 3 },
        });
        match err_outcome {
            Err(f) => assert_eq!(f.source_span.start, 0),
            Ok(_) => panic!("expected Err"),
        }
    }

    #[test]
    fn invocation_failure_variants() {
        let arity = InvocationFailure::ArityMismatch {
            expected: 2,
            actual: 1,
        };
        match arity {
            InvocationFailure::ArityMismatch { expected, actual } => {
                assert_eq!(expected, 2);
                assert_eq!(actual, 1);
            }
            _ => panic!("expected ArityMismatch"),
        }

        let shape = InvocationFailure::ArgumentShapeMismatch { position: 0 };
        match shape {
            InvocationFailure::ArgumentShapeMismatch { position } => assert_eq!(position, 0),
            _ => panic!("expected ArgumentShapeMismatch"),
        }
    }

    #[test]
    fn evaluation_failure_4_variants() {
        let failures = [
            EvaluationFailure::Overflow,
            EvaluationFailure::DivisionByZero,
            EvaluationFailure::Conversion,
            EvaluationFailure::DynamicNumericType,
        ];
        assert_eq!(failures.len(), 4);

        match failures[0] {
            EvaluationFailure::Overflow => {}
            _ => panic!("expected Overflow"),
        }
        match failures[1] {
            EvaluationFailure::DivisionByZero => {}
            _ => panic!("expected DivisionByZero"),
        }
        match failures[2] {
            EvaluationFailure::Conversion => {}
            _ => panic!("expected Conversion"),
        }
        match failures[3] {
            EvaluationFailure::DynamicNumericType => {}
            _ => panic!("expected DynamicNumericType"),
        }
    }

    #[test]
    fn external_execution_failure_3_variants() {
        let missing = ExternalExecutionFailure::MissingBinding {
            signature: SignatureSymbol {
                module: "fs".to_string(),
                name: "read".to_string(),
            },
        };
        match missing {
            ExternalExecutionFailure::MissingBinding { signature } => {
                assert_eq!(signature.module, "fs");
                assert_eq!(signature.name, "read");
            }
            _ => panic!("expected MissingBinding"),
        }

        let cap_fail = ExternalExecutionFailure::CapabilityFailure {
            signature: SignatureSymbol {
                module: "fs".to_string(),
                name: "read".to_string(),
            },
            failure: ExternalCapabilityFailure {
                code: Box::from("not_found"),
            },
        };
        match cap_fail {
            ExternalExecutionFailure::CapabilityFailure { signature, failure } => {
                assert_eq!(signature.module, "fs");
                assert_eq!(signature.name, "read");
                assert_eq!(&*failure.code, "not_found");
            }
            _ => panic!("expected CapabilityFailure"),
        }

        let mismatch = ExternalExecutionFailure::ResultContractMismatch {
            signature: SignatureSymbol {
                module: "fs".to_string(),
                name: "read".to_string(),
            },
        };
        match mismatch {
            ExternalExecutionFailure::ResultContractMismatch { signature } => {
                assert_eq!(signature.module, "fs");
                assert_eq!(signature.name, "read");
            }
            _ => panic!("expected ResultContractMismatch"),
        }
    }

    #[test]
    fn execution_failure_kind_4_variants() {
        let comp = ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(
            LexicalFailure::InvalidIdentifier,
        ));
        match comp {
            ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(
                LexicalFailure::InvalidIdentifier,
            )) => {}
            _ => panic!("expected Compilation"),
        }

        let inv = ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
            expected: 1,
            actual: 0,
        });
        match inv {
            ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                expected,
                actual,
            }) => {
                assert_eq!(expected, 1);
                assert_eq!(actual, 0);
            }
            _ => panic!("expected Invocation"),
        }

        let eval = ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero);
        match eval {
            ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
            _ => panic!("expected Evaluation"),
        }

        let ext = ExternalExecutionFailure::MissingBinding {
            signature: SignatureSymbol {
                module: "m".to_string(),
                name: "f".to_string(),
            },
        };
        let ext_kind = ExecutionFailureKind::External(ext);
        match ext_kind {
            ExecutionFailureKind::External(ExternalExecutionFailure::MissingBinding {
                signature,
            }) => {
                assert_eq!(signature.module, "m");
                assert_eq!(signature.name, "f");
            }
            _ => panic!("expected External"),
        }
    }

    #[test]
    fn execution_failure_provenance_matrix() {
        let comp_fail = ExecutionFailure {
            kind: ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(
                LexicalFailure::InvalidIdentifier,
            )),
            source_span: Some(SourceSpan { start: 1, end: 2 }),
        };
        assert!(comp_fail.source_span.is_some());

        let inv_fail = ExecutionFailure {
            kind: ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                expected: 1,
                actual: 0,
            }),
            source_span: None,
        };
        assert!(inv_fail.source_span.is_none());

        let eval_fail = ExecutionFailure {
            kind: ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow),
            source_span: Some(SourceSpan { start: 10, end: 15 }),
        };
        assert!(eval_fail.source_span.is_some());

        let ext_fail = ExecutionFailure {
            kind: ExecutionFailureKind::External(
                ExternalExecutionFailure::ResultContractMismatch {
                    signature: SignatureSymbol {
                        module: "a".to_string(),
                        name: "b".to_string(),
                    },
                },
            ),
            source_span: Some(SourceSpan { start: 20, end: 25 }),
        };
        assert!(ext_fail.source_span.is_some());
    }

    #[test]
    fn execution_outcome_ok_and_err() {
        let ok_res: ExecutionOutcome = Ok(OwnedValue::Boolean(true));
        match ok_res {
            Ok(OwnedValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Ok(Boolean)"),
        }

        let err_res: ExecutionOutcome = Err(ExecutionFailure {
            kind: ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero),
            source_span: Some(SourceSpan { start: 5, end: 8 }),
        });
        match err_res {
            Err(f) => match f.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                _ => panic!("expected Evaluation DivisionByZero"),
            },
            Ok(_) => panic!("expected Err"),
        }
    }
}
