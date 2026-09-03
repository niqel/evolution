use crate::data::failures::{CompileFailure, ExecutionFailure, ExecutionFailureKind};

pub type ContextualizeCompileFailure = fn(CompileFailure) -> ExecutionFailure;

pub fn contextualize_compile_failure(failure: CompileFailure) -> ExecutionFailure {
    let CompileFailure { kind, source_span } = failure;

    ExecutionFailure {
        kind: ExecutionFailureKind::Compilation(kind),
        source_span: Some(source_span),
    }
}

pub const CONTEXTUALIZE_COMPILE_FAILURE: ContextualizeCompileFailure =
    contextualize_compile_failure;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::failures::{
        CompileFailureKind, LexicalFailure, ResolutionFailure, SemanticFailure, SyntaxFailure,
    };
    use crate::data::lexical::SourceSpan;
    use alloc::string::ToString;

    #[test]
    fn typed_binding() {
        let implementation: ContextualizeCompileFailure = contextualize_compile_failure;
        let binding: ContextualizeCompileFailure = CONTEXTUALIZE_COMPILE_FAILURE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn lexical_failure() {
        let failure = CompileFailure {
            kind: CompileFailureKind::Lexical(LexicalFailure::UnrecognizedCharacter('@')),
            source_span: SourceSpan { start: 4, end: 5 },
        };

        let exec_failure = contextualize_compile_failure(failure);

        match exec_failure.kind {
            ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(
                LexicalFailure::UnrecognizedCharacter(ch),
            )) => {
                assert_eq!(ch, '@');
            }
            _ => panic!("expected Compilation(Lexical) failure"),
        }

        assert_eq!(
            exec_failure.source_span,
            Some(SourceSpan { start: 4, end: 5 })
        );
    }

    #[test]
    fn syntax_failure() {
        let failure = CompileFailure {
            kind: CompileFailureKind::Syntax(SyntaxFailure::MalformedExpression),
            source_span: SourceSpan { start: 10, end: 15 },
        };

        let exec_failure = contextualize_compile_failure(failure);

        match exec_failure.kind {
            ExecutionFailureKind::Compilation(CompileFailureKind::Syntax(
                SyntaxFailure::MalformedExpression,
            )) => {}
            _ => panic!("expected Compilation(Syntax) failure"),
        }

        assert_eq!(
            exec_failure.source_span,
            Some(SourceSpan { start: 10, end: 15 })
        );
    }

    #[test]
    fn semantic_failure_with_owned_payload() {
        let failure = CompileFailure {
            kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::UnknownType {
                    name: "Missing".to_string().into_boxed_str(),
                },
            )),
            source_span: SourceSpan { start: 1, end: 8 },
        };

        let exec_failure = contextualize_compile_failure(failure);

        match exec_failure.kind {
            ExecutionFailureKind::Compilation(CompileFailureKind::Semantic(
                SemanticFailure::Resolution(ResolutionFailure::UnknownType { name }),
            )) => {
                assert_eq!(&*name, "Missing");
            }
            _ => panic!("expected Compilation(Semantic(UnknownType)) failure"),
        }

        assert_eq!(
            exec_failure.source_span,
            Some(SourceSpan { start: 1, end: 8 })
        );
    }

    #[test]
    fn source_span_preservation() {
        let failure = CompileFailure {
            kind: CompileFailureKind::Syntax(SyntaxFailure::MissingFinalReturn),
            source_span: SourceSpan { start: 17, end: 29 },
        };

        let exec_failure = contextualize_compile_failure(failure);

        assert_eq!(
            exec_failure.source_span,
            Some(SourceSpan { start: 17, end: 29 })
        );
    }

    #[test]
    fn zero_width_source_span() {
        let failure = CompileFailure {
            kind: CompileFailureKind::Syntax(SyntaxFailure::MissingFinalReturn),
            source_span: SourceSpan { start: 20, end: 20 },
        };

        let exec_failure = contextualize_compile_failure(failure);

        assert_eq!(
            exec_failure.source_span,
            Some(SourceSpan { start: 20, end: 20 })
        );
    }
}
