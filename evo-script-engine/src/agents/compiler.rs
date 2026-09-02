use crate::collaborators::bytecode_compiler::LOWER_PROGRAM;
use crate::collaborators::lexer::LEX_SOURCE;
use crate::collaborators::parser::PARSE_TOKENS;
use crate::collaborators::semantic_analyzer::ANALYZE_PROGRAM;
use crate::data::compilation_dependency::CompilationCatalog;
use crate::data::failures::CompileOutcome;
use crate::definitions::use_cases::compile;

pub fn compile<'source, 'catalog>(
    source: &'source str,
    catalog: &'catalog CompilationCatalog,
) -> CompileOutcome {
    let tokens = LEX_SOURCE(source)?;
    let program = PARSE_TOKENS(&tokens, source)?;
    let semantic_program = ANALYZE_PROGRAM(&program, catalog)?;
    let compiled_program = LOWER_PROGRAM(&semantic_program);

    Ok(compiled_program)
}

pub const COMPILE: compile::Compile = compile;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::failures::CompileFailureKind;
    use std::collections::HashMap;

    #[test]
    fn compile_agent_typed_binding_and_use_case() {
        let implementation: compile::Compile = compile;
        let binding: compile::Compile = COMPILE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn compile_success_pipeline() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int {
                return 42;
            }
        "#;
        let outcome = COMPILE(src, &catalog);
        match outcome {
            Ok(compiled) => {
                assert_eq!(compiled.entry_point.0, 0);
                assert_eq!(compiled.functions.len(), 1);
                assert_eq!(compiled.functions[0].parameter_count, 0);
            }
            Err(_) => panic!("expected compilation success"),
        }
    }

    #[test]
    fn lexical_failure_propagation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"public fn main() -> int { return "unclosed string; }"#;
        let outcome = COMPILE(src, &catalog);
        match outcome {
            Err(err) => match err.kind {
                CompileFailureKind::Lexical(_) => {}
                _ => panic!("expected CompileFailureKind::Lexical"),
            },
            Ok(_) => panic!("expected lexical failure, got Ok"),
        }
    }

    #[test]
    fn syntax_failure_propagation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() { return 42; }";
        let outcome = COMPILE(src, &catalog);
        match outcome {
            Err(err) => match err.kind {
                CompileFailureKind::Syntax(_) => {}
                _ => panic!("expected CompileFailureKind::Syntax"),
            },
            Ok(_) => panic!("expected syntax failure, got Ok"),
        }
    }

    #[test]
    fn semantic_failure_propagation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int {
                return true;
            }
        "#;
        let outcome = COMPILE(src, &catalog);
        match outcome {
            Err(err) => match err.kind {
                CompileFailureKind::Semantic(_) => {}
                _ => panic!("expected CompileFailureKind::Semantic"),
            },
            Ok(_) => panic!("expected semantic failure, got Ok"),
        }
    }
}
