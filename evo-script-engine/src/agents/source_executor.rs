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

    fn unwrap_ok<T, E>(res: Result<T, E>) -> T {
        match res {
            Ok(val) => val,
            Err(_) => panic!("expected Ok"),
        }
    }

    fn unwrap_err<T, E>(res: Result<T, E>) -> E {
        match res {
            Err(err) => err,
            Ok(_) => panic!("expected Err"),
        }
    }

    #[test]
    fn equivalence_pure_computation_with_parameters_and_locals() {
        use crate::agents::compiled_program_executor::EXECUTE_COMPILED;
        use crate::agents::compiler::COMPILE;

        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            private fn helper(int a, int b) -> int {
                return (a + b) * 2;
            }

            public fn main(int x, int y) -> int {
                let int sum = helper(x, y);
                return sum + 1;
            }
        "#;

        let args = [Value::Int32(5), Value::Int32(7)];

        let source_outcome = EXECUTE_SOURCE(src, &args, &catalog, &bindings);
        let compiled_program = unwrap_ok(COMPILE(src, &catalog));
        let compiled_outcome = EXECUTE_COMPILED(&compiled_program, &args, &bindings);

        match (source_outcome, compiled_outcome) {
            (Ok(OwnedValue::Int32(v1)), Ok(OwnedValue::Int32(v2))) => {
                assert_eq!(v1, 25);
                assert_eq!(v1, v2);
            }
            _ => panic!("expected matching Ok(Int32(25)) outcomes"),
        }
    }

    #[test]
    fn equivalence_composites_structs_enums_and_when() {
        use crate::agents::compiled_program_executor::EXECUTE_COMPILED;
        use crate::agents::compiler::COMPILE;

        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let src = r#"
            struct Point {
                int x;
                int y;
            }

            enum Shape {
                Circle(int),
                Rect { int w; int h; },
                Empty
            }

            private fn area(Shape s) -> int {
                return when s {
                    Shape::Circle(int r) => r * r,
                    Shape::Rect { w: int w; h: int h; } => w * h,
                    Shape::Empty => 0
                };
            }

            public fn main(int w, int h) -> int {
                let Shape s = Shape::Rect { w: w, h: h };
                return area(s);
            }
        "#;

        let compiled_program = unwrap_ok(COMPILE(src, &catalog));

        let args = [Value::Int32(3), Value::Int32(4)];
        let source_outcome = EXECUTE_SOURCE(src, &args, &catalog, &bindings);
        let compiled_outcome = EXECUTE_COMPILED(&compiled_program, &args, &bindings);

        match (source_outcome, compiled_outcome) {
            (Ok(OwnedValue::Int32(v1)), Ok(OwnedValue::Int32(v2))) => {
                assert_eq!(v1, 12);
                assert_eq!(v1, v2);
            }
            _ => panic!("expected matching Ok(Int32(12)) outcomes"),
        }
    }

    #[test]
    fn equivalence_external_capability_dispatch_and_error() {
        use crate::agents::compiled_program_executor::EXECUTE_COMPILED;
        use crate::agents::compiler::COMPILE;
        use crate::data::compilation_dependency::{
            CatalogSignature, CatalogSignatureParameter, CatalogTypeRef,
        };
        use crate::data::failures::{ExternalCapabilityFailure, ExternalExecutionFailure};
        use crate::data::semantic::SignatureSymbol;
        use alloc::boxed::Box;
        use alloc::string::ToString;
        use alloc::vec;

        fn fetch_cap<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            match &args[0] {
                Value::Int32(n) if *n > 0 => Ok(OwnedValue::Int32(n * 100)),
                _ => Err(ExternalCapabilityFailure {
                    code: Box::from("invalid_id"),
                }),
            }
        }

        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "service".to_string(),
                name: "fetch".to_string(),
            },
            CatalogSignature {
                parameters: vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int)],
                result_type: CatalogTypeRef::Int,
            },
        );
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(
            SignatureSymbol {
                module: "service".to_string(),
                name: "fetch".to_string(),
            },
            fetch_cap,
        );

        let src = r#"
            import service::fetch;

            public fn main(int id) -> int {
                return fetch(id) + 1;
            }
        "#;

        let compiled_program = unwrap_ok(COMPILE(src, &catalog));

        // Success case: id = 5
        let args_success = [Value::Int32(5)];
        let src_ok = EXECUTE_SOURCE(src, &args_success, &catalog, &bindings);
        let comp_ok = EXECUTE_COMPILED(&compiled_program, &args_success, &bindings);
        match (src_ok, comp_ok) {
            (Ok(OwnedValue::Int32(v1)), Ok(OwnedValue::Int32(v2))) => {
                assert_eq!(v1, 501);
                assert_eq!(v1, v2);
            }
            _ => panic!("expected matching Ok(Int32(501))"),
        }

        // Error case: id = -1
        let args_err = [Value::Int32(-1)];
        let src_err = EXECUTE_SOURCE(src, &args_err, &catalog, &bindings);
        let comp_err = EXECUTE_COMPILED(&compiled_program, &args_err, &bindings);
        match (src_err, comp_err) {
            (Err(e1), Err(e2)) => {
                assert_eq!(e1.source_span, e2.source_span);
                match (&e1.kind, &e2.kind) {
                    (
                        ExecutionFailureKind::External(
                            ExternalExecutionFailure::CapabilityFailure {
                                signature: s1,
                                failure: f1,
                            },
                        ),
                        ExecutionFailureKind::External(
                            ExternalExecutionFailure::CapabilityFailure {
                                signature: s2,
                                failure: f2,
                            },
                        ),
                    ) => {
                        assert_eq!(s1.module, s2.module);
                        assert_eq!(s1.name, s2.name);
                        assert_eq!(f1.code, f2.code);
                    }
                    _ => panic!("expected matching CapabilityFailure kinds"),
                }
            }
            _ => panic!("expected matching Err outcomes for negative id"),
        }
    }

    #[test]
    fn equivalence_compile_failure_contextualization_matrix() {
        use crate::agents::compiler::COMPILE;

        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        // 1. Lexical failure
        let lex_src = r#"public fn main() -> int { return "unclosed; }"#;
        let compile_lex = unwrap_err(COMPILE(lex_src, &catalog));
        let exec_lex = unwrap_err(EXECUTE_SOURCE(lex_src, &[], &catalog, &bindings));
        assert_eq!(exec_lex.source_span, Some(compile_lex.source_span));
        match (compile_lex.kind, exec_lex.kind) {
            (
                CompileFailureKind::Lexical(_),
                ExecutionFailureKind::Compilation(CompileFailureKind::Lexical(_)),
            ) => {}
            _ => panic!("expected matching lexical compile failures"),
        }

        // 2. Syntax failure
        let syn_src = "public fn main() { return 42; }";
        let compile_syn = unwrap_err(COMPILE(syn_src, &catalog));
        let exec_syn = unwrap_err(EXECUTE_SOURCE(syn_src, &[], &catalog, &bindings));
        assert_eq!(exec_syn.source_span, Some(compile_syn.source_span));
        match (compile_syn.kind, exec_syn.kind) {
            (
                CompileFailureKind::Syntax(_),
                ExecutionFailureKind::Compilation(CompileFailureKind::Syntax(_)),
            ) => {}
            _ => panic!("expected matching syntax compile failures"),
        }

        // 3. Semantic failure
        let sem_src = "public fn main() -> int { return true; }";
        let compile_sem = unwrap_err(COMPILE(sem_src, &catalog));
        let exec_sem = unwrap_err(EXECUTE_SOURCE(sem_src, &[], &catalog, &bindings));
        assert_eq!(exec_sem.source_span, Some(compile_sem.source_span));
        match (compile_sem.kind, exec_sem.kind) {
            (
                CompileFailureKind::Semantic(_),
                ExecutionFailureKind::Compilation(CompileFailureKind::Semantic(_)),
            ) => {}
            _ => panic!("expected matching semantic compile failures"),
        }
    }

    #[test]
    fn equivalence_runtime_failures_matrix() {
        use crate::agents::compiled_program_executor::EXECUTE_COMPILED;
        use crate::agents::compiler::COMPILE;

        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        // 1. Invocation arity mismatch
        let arity_src = "public fn main(int x, int y) -> int { return x + y; }";
        let compiled_arity = unwrap_ok(COMPILE(arity_src, &catalog));
        let src_arity_err = unwrap_err(EXECUTE_SOURCE(arity_src, &[], &catalog, &bindings));
        let comp_arity_err = unwrap_err(EXECUTE_COMPILED(&compiled_arity, &[], &bindings));

        assert_eq!(src_arity_err.source_span, None);
        assert_eq!(comp_arity_err.source_span, None);
        match (src_arity_err.kind, comp_arity_err.kind) {
            (
                ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                    expected: e1,
                    actual: a1,
                }),
                ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                    expected: e2,
                    actual: a2,
                }),
            ) => {
                assert_eq!(e1, 2);
                assert_eq!(e1, e2);
                assert_eq!(a1, 0);
                assert_eq!(a1, a2);
            }
            _ => panic!("expected matching ArityMismatch failures"),
        }

        // 2. Runtime division by zero
        let div_src = "public fn main() -> int { return 100 / 0; }";
        let compiled_div = unwrap_ok(COMPILE(div_src, &catalog));
        let src_div_err = unwrap_err(EXECUTE_SOURCE(div_src, &[], &catalog, &bindings));
        let comp_div_err = unwrap_err(EXECUTE_COMPILED(&compiled_div, &[], &bindings));

        assert!(src_div_err.source_span.is_some());
        assert_eq!(src_div_err.source_span, comp_div_err.source_span);
        match (src_div_err.kind, comp_div_err.kind) {
            (
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero),
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero),
            ) => {}
            _ => panic!("expected matching DivisionByZero failures"),
        }
    }
}
