use evo_values::Value;

use crate::collaborators::bytecode_compiler::LOWER_PROGRAM;
use crate::collaborators::execution_initializer::INITIALIZE_EXECUTION;
use crate::collaborators::instruction_executor::EXECUTE_INSTRUCTION;
use crate::collaborators::lexer::LEX_SOURCE;
use crate::collaborators::parser::PARSE_TOKENS;
use crate::collaborators::semantic_analyzer::ANALYZE_PROGRAM;
use crate::data::compilation_dependency::CompilationCatalog;
use crate::data::compiled::instructions::Instruction;
use crate::data::failures::ExecutionOutcome;
use crate::data::vm::bindings::ApplicationBindings;
use crate::data::vm::state::VmExecution;
use crate::definitions::use_cases::execute_source;
use crate::resolvers::external_call_resolver::RESOLVE_EXTERNAL_CALL;
use crate::tools::contextualize_compile_failure::CONTEXTUALIZE_COMPILE_FAILURE;

fn current_instruction_is_call_external(execution: &VmExecution<'_, '_>) -> bool {
    let active_frame = execution
        .call_frames
        .last()
        .expect("active CallFrame must exist in VmExecution");

    let function = execution
        .compiled_program
        .functions
        .get(active_frame.function.0)
        .expect("active CallFrame function must exist in CompiledProgram");

    let instruction = function
        .instructions
        .get(active_frame.instruction_pointer.0)
        .expect("active CallFrame instruction_pointer must exist in function instructions");

    matches!(instruction, Instruction::CallExternal(_))
}

pub fn execute_source<'source, 'value, 'catalog, 'bindings>(
    source: &'source str,
    invocation_values: &'value [Value<'value>],
    catalog: &'catalog CompilationCatalog,
    application_bindings: &'bindings ApplicationBindings,
) -> ExecutionOutcome {
    let tokens = LEX_SOURCE(source).map_err(CONTEXTUALIZE_COMPILE_FAILURE)?;
    let program = PARSE_TOKENS(&tokens, source).map_err(CONTEXTUALIZE_COMPILE_FAILURE)?;
    let semantic_program =
        ANALYZE_PROGRAM(&program, catalog).map_err(CONTEXTUALIZE_COMPILE_FAILURE)?;
    let compiled_program = LOWER_PROGRAM(&semantic_program);

    let mut execution =
        INITIALIZE_EXECUTION(&compiled_program, invocation_values, application_bindings)?;

    loop {
        if current_instruction_is_call_external(&execution) {
            RESOLVE_EXTERNAL_CALL(&mut execution)?;
        } else if let Some(result) = EXECUTE_INSTRUCTION(&mut execution)? {
            return Ok(result);
        }
    }
}

pub const EXECUTE_SOURCE: execute_source::ExecuteSource = execute_source;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::data::failures::{
        CompileFailureKind, EvaluationFailure, ExecutionFailureKind, InvocationFailure,
    };
    use evo_values::OwnedValue;

    #[test]
    fn typed_binding() {
        let implementation: execute_source::ExecuteSource = execute_source;
        let binding: execute_source::ExecuteSource = EXECUTE_SOURCE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn compile_and_internal_execution_success() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int {
                return 42;
            }
        "#;

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Ok(OwnedValue::Int32(v)) => assert_eq!(v, 42),
            _ => panic!("expected OwnedValue::Int32(42)"),
        }
    }

    #[test]
    fn lexical_failure_contextualization() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"public fn main() -> int { return "unclosed string; }"#;

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Err(failure) => {
                assert!(failure.source_span.is_some());
                match failure.kind {
                    ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(_)) => {}
                    _ => panic!("expected Compilation(Lexical)"),
                }
            }
            Ok(_) => panic!("expected lexical failure"),
        }
    }

    #[test]
    fn syntax_failure_contextualization() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = "public fn main() { return 42; }";

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Err(failure) => {
                assert!(failure.source_span.is_some());
                match failure.kind {
                    ExecutionFailureKind::Compilation(CompileFailureKind::Syntax(_)) => {}
                    _ => panic!("expected Compilation(Syntax)"),
                }
            }
            Ok(_) => panic!("expected syntax failure"),
        }
    }

    #[test]
    fn semantic_failure_contextualization() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int {
                return true;
            }
        "#;

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Err(failure) => {
                assert!(failure.source_span.is_some());
                match failure.kind {
                    ExecutionFailureKind::Compilation(CompileFailureKind::Semantic(_)) => {}
                    _ => panic!("expected Compilation(Semantic)"),
                }
            }
            Ok(_) => panic!("expected semantic failure"),
        }
    }

    #[test]
    fn invocation_failure_propagation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            public fn main(int x) -> int {
                return x;
            }
        "#;

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Err(failure) => {
                assert_eq!(failure.source_span, None);
                match failure.kind {
                    ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                        expected,
                        actual,
                    }) => {
                        assert_eq!(expected, 1);
                        assert_eq!(actual, 0);
                    }
                    _ => panic!("expected ArityMismatch"),
                }
            }
            Ok(_) => panic!("expected invocation failure"),
        }
    }

    #[test]
    fn runtime_evaluation_failure_propagation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int {
                return 10 / 0;
            }
        "#;

        let outcome = EXECUTE_SOURCE(src, &[], &catalog, &bindings);
        match outcome {
            Err(failure) => {
                assert!(failure.source_span.is_some());
                match failure.kind {
                    ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                    _ => panic!("expected DivisionByZero"),
                }
            }
            Ok(_) => panic!("expected division by zero failure"),
        }
    }
}
