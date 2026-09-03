use evo_values::Value;

use crate::collaborators::execution_initializer::INITIALIZE_EXECUTION;
use crate::collaborators::instruction_executor::EXECUTE_INSTRUCTION;
use crate::data::compiled::instructions::Instruction;
use crate::data::compiled::program::CompiledProgram;
use crate::data::failures::ExecutionOutcome;
use crate::data::vm::bindings::ApplicationBindings;
use crate::data::vm::state::VmExecution;
use crate::definitions::use_cases::execute_compiled;
use crate::resolvers::external_call_resolver::RESOLVE_EXTERNAL_CALL;

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

pub fn execute_compiled<'compiled, 'value, 'bindings>(
    compiled_program: &'compiled CompiledProgram,
    invocation_values: &'value [Value<'value>],
    application_bindings: &'bindings ApplicationBindings,
) -> ExecutionOutcome {
    let mut execution =
        INITIALIZE_EXECUTION(compiled_program, invocation_values, application_bindings)?;

    loop {
        if current_instruction_is_call_external(&execution) {
            RESOLVE_EXTERNAL_CALL(&mut execution)?;
        } else if let Some(result) = EXECUTE_INSTRUCTION(&mut execution)? {
            return Ok(result);
        }
    }
}

pub const EXECUTE_COMPILED: execute_compiled::ExecuteCompiled = execute_compiled;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use std::collections::HashMap;

    use crate::data::compiled::boundary::CompiledValueShape;
    use crate::data::compiled::identities::{CompiledValueShapeId, ConstantId, ExternalSymbolId};
    use crate::data::compiled::program::CompiledFunction;
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::compiled::storage::{Constant, ExternalSymbol};
    use crate::data::failures::{
        EvaluationFailure, ExecutionFailureKind, ExternalCapabilityFailure,
        ExternalExecutionFailure, InvocationFailure,
    };
    use crate::data::lexical::SourceSpan;
    use crate::data::semantic::SignatureSymbol;
    use crate::data::semantic::ids::FunctionId;
    use evo_values::OwnedValue;

    #[test]
    fn typed_binding() {
        let implementation: execute_compiled::ExecuteCompiled = execute_compiled;
        let binding: execute_compiled::ExecuteCompiled = EXECUTE_COMPILED;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn internal_only_success_smoke() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(42)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                ]],
            },
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
        match outcome {
            Ok(OwnedValue::Int32(v)) => assert_eq!(v, 42),
            _ => panic!("expected OwnedValue::Int32(42)"),
        }
    }

    #[test]
    fn invocation_failure_propagation() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 1,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::Return],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: vec![CompiledValueShapeId(0)],
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![SourceSpan { start: 0, end: 1 }]],
            },
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
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
    fn external_dispatch_smoke() {
        fn bool_capability<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            assert_eq!(args.len(), 0);
            Ok(OwnedValue::Boolean(true))
        }

        let symbol = SignatureSymbol {
            module: "Env".to_string(),
            name: "IsActive".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::CallExternal(ExternalSymbolId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Env".to_string(),
                    name: "IsActive".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Boolean],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 10 },
                    SourceSpan { start: 10, end: 20 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, bool_capability);

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
        match outcome {
            Ok(OwnedValue::Boolean(b)) => assert!(b),
            _ => panic!("expected OwnedValue::Boolean(true)"),
        }
    }

    #[test]
    fn external_failure_propagation() {
        fn failing_capability<'value>(
            _args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            Err(ExternalCapabilityFailure {
                code: Box::from("permission_denied"),
            })
        }

        let symbol = SignatureSymbol {
            module: "Auth".to_string(),
            name: "Check".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::CallExternal(ExternalSymbolId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Auth".to_string(),
                    name: "Check".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Boolean],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 5, end: 15 },
                    SourceSpan { start: 15, end: 25 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, failing_capability);

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
        match outcome {
            Err(failure) => {
                assert_eq!(failure.source_span, Some(SourceSpan { start: 5, end: 15 }));
                match failure.kind {
                    ExecutionFailureKind::External(
                        ExternalExecutionFailure::CapabilityFailure {
                            signature,
                            failure: cap_fail,
                        },
                    ) => {
                        assert_eq!(signature.module, "Auth");
                        assert_eq!(signature.name, "Check");
                        assert_eq!(cap_fail.code.as_ref(), "permission_denied");
                    }
                    _ => panic!("expected CapabilityFailure"),
                }
            }
            Ok(_) => panic!("expected external capability failure"),
        }
    }
}
