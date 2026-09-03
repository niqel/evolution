use alloc::vec::Vec;

use crate::data::compiled::identities::CompiledValueShapeId;
use crate::data::compiled::instructions::Instruction;
use crate::data::failures::{ExecutionFailure, ExecutionFailureKind, ExternalExecutionFailure};
use crate::data::semantic::SignatureSymbol;
use crate::data::vm::state::{InstructionPointer, VmExecution};
use crate::tools::locate_source_span::LOCATE_SOURCE_SPAN;
use crate::tools::matches_owned_value_shape::MATCHES_OWNED_VALUE_SHAPE;
use crate::tools::materialize_owned_value::MATERIALIZE_OWNED_VALUE;
use crate::tools::observe_runtime_value::OBSERVE_RUNTIME_VALUE;

pub type ResolveExternalCall = for<'compiled, 'bindings> fn(
    &mut VmExecution<'compiled, 'bindings>,
) -> Result<(), ExecutionFailure>;

fn own_signature_symbol(symbol: &SignatureSymbol) -> SignatureSymbol {
    SignatureSymbol {
        module: symbol.module.clone(),
        name: symbol.name.clone(),
    }
}

pub fn resolve_external_call<'compiled, 'bindings>(
    execution: &mut VmExecution<'compiled, 'bindings>,
) -> Result<(), ExecutionFailure> {
    let (function_id_val, instruction_pointer_val, frame_base) = {
        let frame = execution
            .call_frames
            .last()
            .expect("active CallFrame must exist in VmExecution");
        (
            frame.function.0,
            frame.instruction_pointer.0,
            frame.frame_base,
        )
    };

    let function = execution
        .compiled_program
        .functions
        .get(function_id_val)
        .expect("active CallFrame function must exist in CompiledProgram");

    let instruction = function
        .instructions
        .get(instruction_pointer_val)
        .expect("active CallFrame instruction_pointer must exist in function instructions");

    let external_symbol_id = match instruction {
        Instruction::CallExternal(id) => id.0,
        _ => panic!("Instruction at current IP must be CallExternal, found different instruction"),
    };

    let external_symbol = execution
        .compiled_program
        .external_symbols
        .get(external_symbol_id)
        .expect("ExternalSymbolId must exist in compiled program external_symbols");

    let parameter_count = external_symbol.parameter_count;
    let result_shape_id = external_symbol.result_shape.0;

    let operand_base = frame_base + function.parameter_count + function.local_count;
    let operand_depth = execution.value_storage.cells.len() - operand_base;

    assert!(
        operand_depth >= parameter_count,
        "Insufficient caller operand depth {} for external symbol parameter_count {}",
        operand_depth,
        parameter_count
    );

    let argument_start = execution.value_storage.cells.len() - parameter_count;
    for i in argument_start..execution.value_storage.cells.len() {
        assert!(
            execution.value_storage.cells[i].is_some(),
            "Argument cell must contain Some(RuntimeValue)"
        );
    }

    let capability = match execution
        .application_bindings
        .capabilities
        .get(&external_symbol.symbol)
    {
        Some(cap) => *cap,
        None => {
            let owned_signature = own_signature_symbol(&external_symbol.symbol);
            let frame = execution.call_frames.last().unwrap();
            let span = LOCATE_SOURCE_SPAN(execution.compiled_program, frame);
            return Err(ExecutionFailure {
                kind: ExecutionFailureKind::External(ExternalExecutionFailure::MissingBinding {
                    signature: owned_signature,
                }),
                source_span: Some(span),
            });
        }
    };

    let mut arguments = Vec::with_capacity(parameter_count);
    for i in argument_start..execution.value_storage.cells.len() {
        let runtime_val = execution.value_storage.cells[i].expect("cell is Some");
        let observed = OBSERVE_RUNTIME_VALUE(
            runtime_val,
            execution.compiled_program,
            &execution.backing_store,
        );
        arguments.push(observed);
    }

    let owned_result = match capability(&arguments) {
        Ok(res) => res,
        Err(failure) => {
            let owned_signature = own_signature_symbol(&external_symbol.symbol);
            let frame = execution.call_frames.last().unwrap();
            let span = LOCATE_SOURCE_SPAN(execution.compiled_program, frame);
            return Err(ExecutionFailure {
                kind: ExecutionFailureKind::External(ExternalExecutionFailure::CapabilityFailure {
                    signature: owned_signature,
                    failure,
                }),
                source_span: Some(span),
            });
        }
    };

    let shape_matches = MATCHES_OWNED_VALUE_SHAPE(
        &owned_result,
        CompiledValueShapeId(result_shape_id),
        &execution.compiled_program.value_shapes,
    );

    if !shape_matches {
        let owned_signature = own_signature_symbol(&external_symbol.symbol);
        let frame = execution.call_frames.last().unwrap();
        let span = LOCATE_SOURCE_SPAN(execution.compiled_program, frame);
        return Err(ExecutionFailure {
            kind: ExecutionFailureKind::External(
                ExternalExecutionFailure::ResultContractMismatch {
                    signature: owned_signature,
                },
            ),
            source_span: Some(span),
        });
    }

    let new_depth = operand_depth - parameter_count + 1;
    assert!(
        new_depth <= function.max_operand_depth,
        "Result operand depth {} exceeds max_operand_depth {}",
        new_depth,
        function.max_operand_depth
    );

    let runtime_result = MATERIALIZE_OWNED_VALUE(owned_result, &mut execution.backing_store);

    execution.value_storage.cells.truncate(argument_start);
    execution.value_storage.cells.push(Some(runtime_result));

    let frame = execution.call_frames.last_mut().unwrap();
    let next_ip = frame.instruction_pointer.0 + 1;
    assert!(
        next_ip < function.instructions.len(),
        "InstructionPointer advance {} out of bounds for instruction count {}",
        next_ip,
        function.instructions.len()
    );
    frame.instruction_pointer = InstructionPointer(next_ip);

    Ok(())
}

pub const RESOLVE_EXTERNAL_CALL: ResolveExternalCall = resolve_external_call;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use alloc::vec;
    use std::collections::HashMap;

    use crate::data::compiled::boundary::CompiledValueShape;
    use crate::data::compiled::identities::{CompiledValueShapeId, ConstantId, ExternalSymbolId};
    use crate::data::compiled::instructions::Instruction;
    use crate::data::compiled::program::{CompiledFunction, CompiledProgram};
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::compiled::storage::{Constant, ExternalSymbol};
    use crate::data::failures::ExternalCapabilityFailure;
    use crate::data::lexical::SourceSpan;
    use crate::data::semantic::ids::FunctionId;
    use crate::data::vm::backing::ExecutionBackingStore;
    use crate::data::vm::bindings::ApplicationBindings;
    use crate::data::vm::state::{CallFrame, SharedValueStorage};
    use crate::data::vm::values::{RuntimeValue, StringBackingRef};
    use evo_values::{OwnedValue, Value};

    #[test]
    fn typed_binding() {
        let implementation: ResolveExternalCall = resolve_external_call;
        let binding: ResolveExternalCall = RESOLVE_EXTERNAL_CALL;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn successful_external_call_with_arguments() {
        fn add_capability<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            assert_eq!(args.len(), 2);
            match (&args[0], &args[1]) {
                (Value::Int32(a), Value::Int32(b)) => Ok(OwnedValue::Int32(a + b)),
                _ => panic!("unexpected argument types"),
            }
        }

        let symbol = SignatureSymbol {
            module: "Math".to_string(),
            name: "Add".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 3,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::CallExternal(ExternalSymbolId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(10), Constant::Int32(20)],
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Math".to_string(),
                    name: "Add".to_string(),
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
        bindings.capabilities.insert(symbol, add_capability);

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage {
                cells: vec![Some(RuntimeValue::Int32(10)), Some(RuntimeValue::Int32(20))],
            },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(2),
                frame_base: 0,
            }],
        };

        let result = resolve_external_call(&mut execution);
        assert!(result.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 3);
        assert_eq!(execution.value_storage.cells.len(), 1);
        match execution.value_storage.cells[0] {
            Some(RuntimeValue::Int32(v)) => assert_eq!(v, 30),
            _ => panic!("expected Int32(30)"),
        }
    }

    #[test]
    fn zero_parameter_external_call() {
        fn bool_capability<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            assert_eq!(args.len(), 0);
            Ok(OwnedValue::Boolean(true))
        }

        let symbol = SignatureSymbol {
            module: "System".to_string(),
            name: "Flag".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
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
                    module: "System".to_string(),
                    name: "Flag".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Boolean],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 5 },
                    SourceSpan { start: 5, end: 10 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, bool_capability);

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let result = resolve_external_call(&mut execution);
        assert!(result.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 1);
        assert_eq!(execution.value_storage.cells.len(), 1);
        match execution.value_storage.cells[0] {
            Some(RuntimeValue::Boolean(v)) => assert!(v),
            _ => panic!("expected Boolean(true)"),
        }
    }

    #[test]
    fn missing_binding_failure() {
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
                    module: "IO".to_string(),
                    name: "Print".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Boolean],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan {
                        start: 100,
                        end: 110,
                    },
                    SourceSpan {
                        start: 110,
                        end: 120,
                    },
                ]],
            },
        };

        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let err = match resolve_external_call(&mut execution) {
            Ok(_) => panic!("missing binding should return Err"),
            Err(e) => e,
        };

        assert_eq!(execution.call_frames[0].instruction_pointer.0, 0); // IP unchanged
        assert_eq!(execution.value_storage.cells.len(), 0); // stack unchanged
        match err.kind {
            ExecutionFailureKind::External(ExternalExecutionFailure::MissingBinding {
                signature,
            }) => {
                assert_eq!(signature.module, "IO");
                assert_eq!(signature.name, "Print");
            }
            _ => panic!("expected MissingBinding"),
        }
        assert_eq!(
            err.source_span,
            Some(SourceSpan {
                start: 100,
                end: 110
            })
        );
    }

    #[test]
    fn capability_failure() {
        fn failing_capability<'value>(
            _args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            Err(ExternalCapabilityFailure {
                code: Box::from("disk_full"),
            })
        }

        let symbol = SignatureSymbol {
            module: "FS".to_string(),
            name: "Write".to_string(),
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
                    module: "FS".to_string(),
                    name: "Write".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::Boolean],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 10, end: 20 },
                    SourceSpan { start: 20, end: 30 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, failing_capability);

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let err = match resolve_external_call(&mut execution) {
            Ok(_) => panic!("failing capability should return Err"),
            Err(e) => e,
        };

        assert_eq!(execution.call_frames[0].instruction_pointer.0, 0); // IP unchanged
        assert_eq!(execution.value_storage.cells.len(), 0); // stack unchanged
        match err.kind {
            ExecutionFailureKind::External(ExternalExecutionFailure::CapabilityFailure {
                signature,
                failure,
            }) => {
                assert_eq!(signature.module, "FS");
                assert_eq!(signature.name, "Write");
                assert_eq!(failure.code.as_ref(), "disk_full");
            }
            _ => panic!("expected CapabilityFailure"),
        }
        assert_eq!(err.source_span, Some(SourceSpan { start: 10, end: 20 }));
    }

    #[test]
    fn result_contract_mismatch() {
        fn string_capability<'value>(
            _args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            Ok(OwnedValue::String(Box::from("invalid_type")))
        }

        let symbol = SignatureSymbol {
            module: "Math".to_string(),
            name: "Sqrt".to_string(),
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
                    module: "Math".to_string(),
                    name: "Sqrt".to_string(),
                },
                parameter_count: 0,
                result_shape: CompiledValueShapeId(0), // Expected Float64
            }],
            value_shapes: vec![CompiledValueShape::Float64],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 50, end: 60 },
                    SourceSpan { start: 60, end: 70 },
                ]],
            },
        };

        let mut bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        bindings.capabilities.insert(symbol, string_capability);

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let err = match resolve_external_call(&mut execution) {
            Ok(_) => panic!("type mismatch should return Err"),
            Err(e) => e,
        };

        assert_eq!(execution.call_frames[0].instruction_pointer.0, 0); // IP unchanged
        assert_eq!(execution.value_storage.cells.len(), 0); // stack unchanged
        assert_eq!(execution.backing_store.strings.len(), 0); // no materialization
        match err.kind {
            ExecutionFailureKind::External(ExternalExecutionFailure::ResultContractMismatch {
                signature,
            }) => {
                assert_eq!(signature.module, "Math");
                assert_eq!(signature.name, "Sqrt");
            }
            _ => panic!("expected ResultContractMismatch"),
        }
        assert_eq!(err.source_span, Some(SourceSpan { start: 50, end: 60 }));
    }

    #[test]
    #[should_panic(expected = "Instruction at current IP must be CallExternal")]
    fn precondition_non_call_external_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::Return],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: vec![vec![SourceSpan { start: 0, end: 1 }]],
            },
        };

        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let _ = resolve_external_call(&mut execution);
    }

    #[test]
    #[should_panic(
        expected = "Insufficient caller operand depth 0 for external symbol parameter_count 1"
    )]
    fn insufficient_operand_depth_panics() {
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
                    module: "Test".to_string(),
                    name: "Fn".to_string(),
                },
                parameter_count: 1,
                result_shape: CompiledValueShapeId(0),
            }],
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

        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage { cells: Vec::new() },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(0),
                frame_base: 0,
            }],
        };

        let _ = resolve_external_call(&mut execution);
    }

    #[test]
    fn successful_external_call_with_preexisting_stack_operands_and_string_result() {
        fn string_capability<'value>(
            args: &'value [Value<'value>],
        ) -> Result<OwnedValue, ExternalCapabilityFailure> {
            assert_eq!(args.len(), 1);
            match &args[0] {
                Value::Int32(n) => Ok(OwnedValue::String(Box::from(alloc::format!("num_{n}")))),
                _ => panic!("expected Int32"),
            }
        }

        let symbol = SignatureSymbol {
            module: "Fmt".to_string(),
            name: "Num".to_string(),
        };

        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 3,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::CallExternal(ExternalSymbolId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(999), Constant::Int32(42)],
            external_symbols: vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Fmt".to_string(),
                    name: "Num".to_string(),
                },
                parameter_count: 1,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: vec![CompiledValueShape::String],
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
        bindings.capabilities.insert(symbol, string_capability);

        // Pre-existing operand (Int32(999)) + external arg (Int32(42))
        let mut execution = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage: SharedValueStorage {
                cells: vec![
                    Some(RuntimeValue::Int32(999)),
                    Some(RuntimeValue::Int32(42)),
                ],
            },
            backing_store: ExecutionBackingStore {
                strings: Vec::new(),
                dynamic_integers: Vec::new(),
                structs: Vec::new(),
                enums: Vec::new(),
            },
            call_frames: vec![CallFrame {
                function: FunctionId(0),
                instruction_pointer: InstructionPointer(2),
                frame_base: 0,
            }],
        };

        let result = resolve_external_call(&mut execution);
        assert!(result.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 3);
        // Should have 2 cells: pre-existing 999 + new string result
        assert_eq!(execution.value_storage.cells.len(), 2);
        match execution.value_storage.cells[0] {
            Some(RuntimeValue::Int32(v)) => assert_eq!(v, 999),
            _ => panic!("expected preserved base cell Int32(999)"),
        }
        match execution.value_storage.cells[1] {
            Some(RuntimeValue::String(StringBackingRef::Execution(id))) => {
                assert_eq!(execution.backing_store.strings[id.0].as_ref(), "num_42");
            }
            _ => panic!("expected ExecutionString result cell"),
        }
    }
}
