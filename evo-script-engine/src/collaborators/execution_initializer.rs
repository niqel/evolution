use alloc::vec::Vec;

use crate::data::compiled::identities::CompiledValueShapeId;
use crate::data::compiled::program::CompiledProgram;
use crate::data::failures::{ExecutionFailure, ExecutionFailureKind, InvocationFailure};
use crate::data::semantic::ids::FunctionId;
use crate::data::vm::backing::ExecutionBackingStore;
use crate::data::vm::bindings::ApplicationBindings;
use crate::data::vm::state::{CallFrame, InstructionPointer, SharedValueStorage, VmExecution};
use crate::tools::matches_value_shape::MATCHES_VALUE_SHAPE;
use crate::tools::materialize_value::MATERIALIZE_VALUE;
use evo_values::Value;

pub type Initialize = for<'compiled, 'value, 'bindings> fn(
    &'compiled CompiledProgram,
    &'value [Value<'value>],
    &'bindings ApplicationBindings,
) -> Result<
    VmExecution<'compiled, 'bindings>,
    ExecutionFailure,
>;

pub fn initialize_execution<'compiled, 'value, 'bindings>(
    compiled_program: &'compiled CompiledProgram,
    invocation_values: &'value [Value<'value>],
    application_bindings: &'bindings ApplicationBindings,
) -> Result<VmExecution<'compiled, 'bindings>, ExecutionFailure> {
    let expected = compiled_program.entry_parameter_shapes.len();
    let actual = invocation_values.len();
    if actual != expected {
        return Err(ExecutionFailure {
            kind: ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                expected,
                actual,
            }),
            source_span: None,
        });
    }

    for (position, (val, expected_shape_id)) in invocation_values
        .iter()
        .zip(compiled_program.entry_parameter_shapes.iter())
        .enumerate()
    {
        if !MATCHES_VALUE_SHAPE(
            val,
            CompiledValueShapeId(expected_shape_id.0),
            &compiled_program.value_shapes,
        ) {
            return Err(ExecutionFailure {
                kind: ExecutionFailureKind::Invocation(InvocationFailure::ArgumentShapeMismatch {
                    position,
                }),
                source_span: None,
            });
        }
    }

    let entry_function = compiled_program
        .functions
        .get(compiled_program.entry_point.0)
        .expect("CompiledProgram entry_point FunctionId must exist in functions");

    let mut backing_store = ExecutionBackingStore {
        strings: Vec::new(),
        dynamic_integers: Vec::new(),
        structs: Vec::new(),
        enums: Vec::new(),
    };

    let mut cells = Vec::with_capacity(
        entry_function.parameter_count
            + entry_function.local_count
            + entry_function.max_operand_depth,
    );

    for val in invocation_values.iter() {
        let runtime_val = MATERIALIZE_VALUE(val, &mut backing_store);
        cells.push(Some(runtime_val));
    }

    for _ in 0..entry_function.local_count {
        cells.push(None);
    }

    let value_storage = SharedValueStorage { cells };

    let entry_frame = CallFrame {
        function: FunctionId(compiled_program.entry_point.0),
        instruction_pointer: InstructionPointer(0),
        frame_base: 0,
    };

    let call_frames = alloc::vec![entry_frame];

    Ok(VmExecution {
        compiled_program,
        application_bindings,
        value_storage,
        backing_store,
        call_frames,
    })
}

pub const INITIALIZE_EXECUTION: Initialize = initialize_execution;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use std::collections::HashMap;

    use crate::data::compiled::boundary::CompiledValueShape;
    use crate::data::compiled::identities::CompiledValueShapeId;
    use crate::data::compiled::program::CompiledFunction;
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::semantic::ids::FunctionId;
    use crate::data::vm::values::RuntimeValue;

    fn make_test_program() -> CompiledProgram {
        CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 1,
                local_count: 2,
                max_operand_depth: 3,
                instructions: Vec::new(),
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: vec![CompiledValueShapeId(0)],
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![Vec::new()],
            },
        }
    }

    #[test]
    fn typed_binding() {
        let implementation: Initialize = initialize_execution;
        let binding: Initialize = INITIALIZE_EXECUTION;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn basic_successful_initialization() {
        let program = make_test_program();
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let args = [Value::Int32(42)];

        let execution = match initialize_execution(&program, &args, &bindings) {
            Ok(exec) => exec,
            Err(_) => panic!("should initialize"),
        };

        assert_eq!(execution.call_frames.len(), 1);
        assert_eq!(execution.call_frames[0].function.0, 0);
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 0);
        assert_eq!(execution.call_frames[0].frame_base, 0);

        assert_eq!(execution.value_storage.cells.len(), 3);
        match execution.value_storage.cells[0] {
            Some(RuntimeValue::Int32(v)) => assert_eq!(v, 42),
            _ => panic!("expected parameter cell to be Some(Int32(42))"),
        }
        assert!(execution.value_storage.cells[1].is_none());
        assert!(execution.value_storage.cells[2].is_none());
    }

    #[test]
    fn arity_mismatch_failure() {
        let program = make_test_program();
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let args = [];

        let err = match initialize_execution(&program, &args, &bindings) {
            Ok(_) => panic!("should fail arity"),
            Err(e) => e,
        };
        match err.kind {
            ExecutionFailureKind::Invocation(InvocationFailure::ArityMismatch {
                expected,
                actual,
            }) => {
                assert_eq!(expected, 1);
                assert_eq!(actual, 0);
            }
            _ => panic!("expected ArityMismatch"),
        }
        assert!(err.source_span.is_none());
    }

    #[test]
    fn argument_shape_mismatch_failure() {
        let program = make_test_program();
        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };
        let args = [Value::Boolean(true)];

        let err = match initialize_execution(&program, &args, &bindings) {
            Ok(_) => panic!("should fail shape"),
            Err(e) => e,
        };
        match err.kind {
            ExecutionFailureKind::Invocation(InvocationFailure::ArgumentShapeMismatch {
                position,
            }) => {
                assert_eq!(position, 0);
            }
            _ => panic!("expected ArgumentShapeMismatch"),
        }
        assert!(err.source_span.is_none());
    }
}
