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
    use crate::data::compiled::identities::{
        CompiledValueShapeId, ConstantId, ExternalSymbolId, ParameterSlot,
    };
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

    #[test]
    fn execute_compiled_with_arguments_and_internal_call_and_arithmetic() {
        use crate::data::compiled::identities::{NumericKind, ParameterSlot};

        let program = CompiledProgram {
            functions: vec![
                CompiledFunction {
                    parameter_count: 2,
                    local_count: 0,
                    max_operand_depth: 3,
                    instructions: vec![
                        Instruction::LoadParameter(ParameterSlot(0)),
                        Instruction::LoadParameter(ParameterSlot(1)),
                        Instruction::Add(NumericKind::Int32),
                        Instruction::Call(FunctionId(1)),
                        Instruction::Return,
                    ],
                },
                CompiledFunction {
                    parameter_count: 1,
                    local_count: 0,
                    max_operand_depth: 2,
                    instructions: vec![
                        Instruction::LoadParameter(ParameterSlot(0)),
                        Instruction::LoadConstant(ConstantId(0)),
                        Instruction::Multiply(NumericKind::Int32),
                        Instruction::Return,
                    ],
                },
            ],
            entry_point: FunctionId(0),
            entry_parameter_shapes: vec![CompiledValueShapeId(0), CompiledValueShapeId(0)],
            constants: vec![Constant::Int32(2)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![
                    vec![
                        SourceSpan { start: 0, end: 1 },
                        SourceSpan { start: 1, end: 2 },
                        SourceSpan { start: 2, end: 3 },
                        SourceSpan { start: 3, end: 4 },
                        SourceSpan { start: 4, end: 5 },
                    ],
                    vec![
                        SourceSpan { start: 10, end: 11 },
                        SourceSpan { start: 11, end: 12 },
                        SourceSpan { start: 12, end: 13 },
                        SourceSpan { start: 13, end: 14 },
                    ],
                ],
            },
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let args = [Value::Int32(10), Value::Int32(20)];
        let outcome = EXECUTE_COMPILED(&program, &args, &bindings);
        match outcome {
            Ok(OwnedValue::Int32(v)) => assert_eq!(v, 60),
            _ => panic!("expected OwnedValue::Int32(60)"),
        }
    }

    #[test]
    fn execute_compiled_external_call_with_arguments() {
        fn multiply_cap<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            assert_eq!(args.len(), 2);
            match (&args[0], &args[1]) {
                (Value::Int32(a), Value::Int32(b)) => Ok(OwnedValue::Int32(a * b)),
                _ => panic!("unexpected args"),
            }
        }

        let symbol = SignatureSymbol {
            module: "Math".to_string(),
            name: "Mul".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 2,
                local_count: 0,
                max_operand_depth: 3,
                instructions: vec![
                    Instruction::LoadParameter(ParameterSlot(0)),
                    Instruction::LoadParameter(ParameterSlot(1)),
                    Instruction::CallExternal(ExternalSymbolId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: vec![CompiledValueShapeId(0), CompiledValueShapeId(0)],
            constants: Vec::new(),
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Math".to_string(),
                    name: "Mul".to_string(),
                },
                parameter_count: 2,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
                    SourceSpan { start: 3, end: 4 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, multiply_cap);

        let args = [Value::Int32(6), Value::Int32(7)];
        let outcome = EXECUTE_COMPILED(&program, &args, &bindings);
        match outcome {
            Ok(OwnedValue::Int32(v)) => assert_eq!(v, 42),
            _ => panic!("expected OwnedValue::Int32(42)"),
        }
    }

    #[test]
    fn execute_compiled_evaluation_failure_propagation() {
        use crate::data::compiled::identities::NumericKind;

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::Divide(NumericKind::Int32),
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(10), Constant::Int32(0)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan {
                        start: 100,
                        end: 110,
                    },
                ]],
            },
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
        match outcome {
            Err(failure) => {
                assert_eq!(
                    failure.source_span,
                    Some(SourceSpan {
                        start: 100,
                        end: 110
                    })
                );
                match failure.kind {
                    ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                    _ => panic!("expected DivisionByZero"),
                }
            }
            Ok(_) => panic!("expected division by zero failure"),
        }
    }

    #[test]
    fn execute_compiled_missing_binding_propagation() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::CallExternal(ExternalSymbolId(0))],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Unbound".to_string(),
                    name: "Service".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![SourceSpan { start: 20, end: 30 }]],
            },
        };
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let outcome = EXECUTE_COMPILED(&program, &[], &bindings);
        match outcome {
            Err(failure) => {
                assert_eq!(failure.source_span, Some(SourceSpan { start: 20, end: 30 }));
                match failure.kind {
                    ExecutionFailureKind::External(ExternalExecutionFailure::MissingBinding {
                        signature,
                    }) => {
                        assert_eq!(signature.module, "Unbound");
                        assert_eq!(signature.name, "Service");
                    }
                    _ => panic!("expected MissingBinding"),
                }
            }
            Ok(_) => panic!("expected missing binding failure"),
        }
    }

    #[test]
    fn execute_compiled_argument_shape_mismatch_propagation() {
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

        let args = [Value::Boolean(true)];
        let outcome = EXECUTE_COMPILED(&program, &args, &bindings);
        match outcome {
            Err(failure) => {
                assert_eq!(failure.source_span, None);
                match failure.kind {
                    ExecutionFailureKind::Invocation(
                        InvocationFailure::ArgumentShapeMismatch { position },
                    ) => {
                        assert_eq!(position, 0);
                    }
                    _ => panic!("expected ArgumentShapeMismatch"),
                }
            }
            Ok(_) => panic!("expected shape mismatch failure"),
        }
    }
}
