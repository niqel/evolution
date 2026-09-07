use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;

use crate::data::compiled::identities::{ConstantId, FieldIndex, NumericKind, VariantDiscriminant};
use crate::data::compiled::instructions::Instruction;
use crate::data::compiled::program::CompiledProgram;
use crate::data::compiled::storage::{Constant, DynamicConstant};
use crate::data::failures::{EvaluationFailure, ExecutionFailure, ExecutionFailureKind};
use crate::data::semantic::ids::FunctionId;
use crate::data::vm::backing::{
    DynamicIntegerBacking, EnumBacking, ExecutionBackingStore, RuntimeEnumPayload, StructBacking,
};
use crate::data::vm::state::{CallFrame, InstructionPointer, VmExecution};
use crate::data::vm::values::{
    DynamicIntegerBackingId, DynamicIntegerBackingRef, DynamicValue as RuntimeDynamicValue,
    EnumBackingId, RuntimeValue, StringBackingId, StringBackingRef, StructBackingId,
};
use crate::tools::locate_source_span::LOCATE_SOURCE_SPAN;
use crate::tools::materialize_owned_value::MATERIALIZE_OWNED_VALUE;
use crate::tools::observe_runtime_value::OBSERVE_RUNTIME_VALUE;
use crate::tools::own_runtime_value::OWN_RUNTIME_VALUE;
use evo_values::boolean::NOT;
use evo_values::comparison::{EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL, NOT_EQUAL};
use evo_values::conversion;
use evo_values::dynamic_numeric;
use evo_values::numeric::{
    ADD_F32, ADD_F64, ADD_I8, ADD_I16, ADD_I32, ADD_I64, ADD_I128, ADD_U8, ADD_U16, ADD_U32,
    ADD_U64, ADD_U128, DIVIDE_F32, DIVIDE_F64, DIVIDE_I8, DIVIDE_I16, DIVIDE_I32, DIVIDE_I64,
    DIVIDE_I128, DIVIDE_U8, DIVIDE_U16, DIVIDE_U32, DIVIDE_U64, DIVIDE_U128, MULTIPLY_F32,
    MULTIPLY_F64, MULTIPLY_I8, MULTIPLY_I16, MULTIPLY_I32, MULTIPLY_I64, MULTIPLY_I128,
    MULTIPLY_U8, MULTIPLY_U16, MULTIPLY_U32, MULTIPLY_U64, MULTIPLY_U128, NEGATE_F32, NEGATE_F64,
    NEGATE_I8, NEGATE_I16, NEGATE_I32, NEGATE_I64, NEGATE_I128, REMAINDER_I8, REMAINDER_I16,
    REMAINDER_I32, REMAINDER_I64, REMAINDER_I128, REMAINDER_U8, REMAINDER_U16, REMAINDER_U32,
    REMAINDER_U64, REMAINDER_U128, SUBTRACT_F32, SUBTRACT_F64, SUBTRACT_I8, SUBTRACT_I16,
    SUBTRACT_I32, SUBTRACT_I64, SUBTRACT_I128, SUBTRACT_U8, SUBTRACT_U16, SUBTRACT_U32,
    SUBTRACT_U64, SUBTRACT_U128,
};
use evo_values::{
    ComparisonFailure, ConversionFailure, DynamicNumericFailure, NumericFailure, OwnedValue, Value,
};

pub type ExecuteInstruction =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<Option<OwnedValue>, ExecutionFailure>;

fn make_evaluation_failure(
    execution: &VmExecution,
    failure: EvaluationFailure,
) -> ExecutionFailure {
    let frame = execution
        .call_frames
        .last()
        .expect("active CallFrame must exist for evaluation failure");
    let span = LOCATE_SOURCE_SPAN(execution.compiled_program, frame);
    ExecutionFailure {
        kind: ExecutionFailureKind::Evaluation(failure),
        source_span: Some(span),
    }
}

fn map_numeric_failure(failure: NumericFailure) -> EvaluationFailure {
    match failure {
        NumericFailure::Overflow => EvaluationFailure::Overflow,
        NumericFailure::DivisionByZero => EvaluationFailure::DivisionByZero,
        NumericFailure::InvalidBounds => {
            panic!("internal invariant violation: unexpected InvalidBounds from arithmetic UC")
        }
    }
}

fn map_conversion_failure(failure: ConversionFailure) -> EvaluationFailure {
    match failure {
        ConversionFailure::NotExactlyRepresentable => EvaluationFailure::Conversion,
    }
}

fn map_dynamic_numeric_failure(failure: DynamicNumericFailure) -> EvaluationFailure {
    match failure {
        DynamicNumericFailure::DifferentFamily => EvaluationFailure::DynamicNumericType,
        DynamicNumericFailure::DivisionByZero => EvaluationFailure::DivisionByZero,
    }
}

fn expect_comparison_result(result: Result<bool, ComparisonFailure>) -> bool {
    match result {
        Ok(v) => v,
        Err(ComparisonFailure::DifferentFamily) => {
            panic!(
                "internal invariant violation: ComparisonFailure::DifferentFamily in comparison instruction"
            )
        }
        Err(ComparisonFailure::NotComparable) => {
            panic!(
                "internal invariant violation: ComparisonFailure::NotComparable in comparison instruction"
            )
        }
    }
}

fn adapt_numeric_operands(
    left: RuntimeValue,
    right: RuntimeValue,
    kind: &NumericKind,
) -> (Value<'static>, Value<'static>) {
    match (kind, left, right) {
        (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
            (Value::Int8(l), Value::Int8(r))
        }
        (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
            (Value::Int16(l), Value::Int16(r))
        }
        (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
            (Value::Int32(l), Value::Int32(r))
        }
        (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
            (Value::Int64(l), Value::Int64(r))
        }
        (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
            (Value::Int128(l), Value::Int128(r))
        }

        (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
            (Value::Uint8(l), Value::Uint8(r))
        }
        (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
            (Value::Uint16(l), Value::Uint16(r))
        }
        (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
            (Value::Uint32(l), Value::Uint32(r))
        }
        (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
            (Value::Uint64(l), Value::Uint64(r))
        }
        (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
            (Value::Uint128(l), Value::Uint128(r))
        }

        (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
            (Value::Float32(l), Value::Float32(r))
        }
        (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
            (Value::Float64(l), Value::Float64(r))
        }

        _ => panic!("Numeric comparison: operand family mismatch with NumericKind"),
    }
}

fn pop_operand(execution: &mut VmExecution) -> RuntimeValue {
    let frame = execution
        .call_frames
        .last()
        .expect("active CallFrame must exist");
    let function = execution
        .compiled_program
        .functions
        .get(frame.function.0)
        .expect("CallFrame function must exist");
    let operand_base = frame.frame_base + function.parameter_count + function.local_count;

    if execution.value_storage.cells.len() <= operand_base {
        panic!("Operand stack underflow below operand_base");
    }

    execution
        .value_storage
        .cells
        .pop()
        .expect("pop from non-empty cells")
        .expect("Operand cell must contain Some(RuntimeValue)")
}

fn push_operand(execution: &mut VmExecution, val: RuntimeValue) {
    let frame = execution
        .call_frames
        .last()
        .expect("active CallFrame must exist");
    let function = execution
        .compiled_program
        .functions
        .get(frame.function.0)
        .expect("CallFrame function must exist");
    let operand_base = frame.frame_base + function.parameter_count + function.local_count;
    let current_depth = execution.value_storage.cells.len() - operand_base;

    assert!(
        current_depth < function.max_operand_depth,
        "Operand stack depth {} exceeded max_operand_depth {}",
        current_depth,
        function.max_operand_depth
    );

    execution.value_storage.cells.push(Some(val));
}

fn advance_ip(execution: &mut VmExecution) {
    let frame = execution
        .call_frames
        .last_mut()
        .expect("active CallFrame must exist");
    let function = execution
        .compiled_program
        .functions
        .get(frame.function.0)
        .expect("CallFrame function must exist");
    let next_ip = frame.instruction_pointer.0 + 1;
    assert!(
        next_ip < function.instructions.len(),
        "InstructionPointer advance {} out of bounds for instruction count {}",
        next_ip,
        function.instructions.len()
    );
    frame.instruction_pointer = InstructionPointer(next_ip);
}

fn jump_ip(execution: &mut VmExecution, target: usize) {
    let frame = execution
        .call_frames
        .last_mut()
        .expect("active CallFrame must exist");
    let function = execution
        .compiled_program
        .functions
        .get(frame.function.0)
        .expect("CallFrame function must exist");
    assert!(
        target < function.instructions.len(),
        "Jump target {} out of bounds for instruction count {}",
        target,
        function.instructions.len()
    );
    frame.instruction_pointer = InstructionPointer(target);
}

fn resolve_string<'a>(
    string_ref: StringBackingRef,
    compiled: &'a CompiledProgram,
    backing: &'a ExecutionBackingStore,
) -> &'a str {
    match string_ref {
        StringBackingRef::Compiled(constant_id) => match &compiled.constants[constant_id.0] {
            Constant::String(s) => s.as_str(),
            _ => {
                panic!("Constant referenced by StringBackingRef::Compiled must be Constant::String")
            }
        },
        StringBackingRef::Execution(id) => &backing.strings[id.0],
    }
}

fn assert_runtime_value_matches_kind(val: &RuntimeValue, kind: &NumericKind) {
    let matches = match (kind, val) {
        (NumericKind::Int8, RuntimeValue::Int8(_)) => true,
        (NumericKind::Int16, RuntimeValue::Int16(_)) => true,
        (NumericKind::Int32, RuntimeValue::Int32(_)) => true,
        (NumericKind::Int64, RuntimeValue::Int64(_)) => true,
        (NumericKind::Int128, RuntimeValue::Int128(_)) => true,

        (NumericKind::Uint8, RuntimeValue::Uint8(_)) => true,
        (NumericKind::Uint16, RuntimeValue::Uint16(_)) => true,
        (NumericKind::Uint32, RuntimeValue::Uint32(_)) => true,
        (NumericKind::Uint64, RuntimeValue::Uint64(_)) => true,
        (NumericKind::Uint128, RuntimeValue::Uint128(_)) => true,

        (NumericKind::Float32, RuntimeValue::Float32(_)) => true,
        (NumericKind::Float64, RuntimeValue::Float64(_)) => true,

        _ => false,
    };
    assert!(
        matches,
        "convert_fixed_numeric: runtime value family does not match source NumericKind"
    );
}

macro_rules! dispatch_to_target {
    ($target_enum:ident, $source:expr, $val:expr,
     $to_i8:ident, $to_i16:ident, $to_i32:ident, $to_i64:ident, $to_i128:ident,
     $to_u8:ident, $to_u16:ident, $to_u32:ident, $to_u64:ident, $to_u128:ident,
     $to_f32:ident, $to_f64:ident) => {
        match ($source, $val) {
            (NumericKind::Int8, RuntimeValue::Int8(v)) => {
                conversion::$to_i8(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Int16, RuntimeValue::Int16(v)) => {
                conversion::$to_i16(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Int32, RuntimeValue::Int32(v)) => {
                conversion::$to_i32(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Int64, RuntimeValue::Int64(v)) => {
                conversion::$to_i64(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Int128, RuntimeValue::Int128(v)) => {
                conversion::$to_i128(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Uint8, RuntimeValue::Uint8(v)) => {
                conversion::$to_u8(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Uint16, RuntimeValue::Uint16(v)) => {
                conversion::$to_u16(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Uint32, RuntimeValue::Uint32(v)) => {
                conversion::$to_u32(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Uint64, RuntimeValue::Uint64(v)) => {
                conversion::$to_u64(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Uint128, RuntimeValue::Uint128(v)) => {
                conversion::$to_u128(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Float32, RuntimeValue::Float32(v)) => {
                conversion::$to_f32(v).map(RuntimeValue::$target_enum)
            }
            (NumericKind::Float64, RuntimeValue::Float64(v)) => {
                conversion::$to_f64(v).map(RuntimeValue::$target_enum)
            }
            _ => panic!(
                "convert_fixed_numeric: runtime value family mismatch with source NumericKind"
            ),
        }
    };
}

fn convert_fixed_numeric(
    val: RuntimeValue,
    source: &NumericKind,
    target: &NumericKind,
) -> Result<RuntimeValue, EvaluationFailure> {
    assert_runtime_value_matches_kind(&val, source);

    let res = match target {
        NumericKind::Int8 => dispatch_to_target!(
            Int8,
            source,
            val,
            TO_INT8_FROM_I8,
            TO_INT8_FROM_I16,
            TO_INT8_FROM_I32,
            TO_INT8_FROM_I64,
            TO_INT8_FROM_I128,
            TO_INT8_FROM_U8,
            TO_INT8_FROM_U16,
            TO_INT8_FROM_U32,
            TO_INT8_FROM_U64,
            TO_INT8_FROM_U128,
            TO_INT8_FROM_F32,
            TO_INT8_FROM_F64
        ),
        NumericKind::Int16 => dispatch_to_target!(
            Int16,
            source,
            val,
            TO_INT16_FROM_I8,
            TO_INT16_FROM_I16,
            TO_INT16_FROM_I32,
            TO_INT16_FROM_I64,
            TO_INT16_FROM_I128,
            TO_INT16_FROM_U8,
            TO_INT16_FROM_U16,
            TO_INT16_FROM_U32,
            TO_INT16_FROM_U64,
            TO_INT16_FROM_U128,
            TO_INT16_FROM_F32,
            TO_INT16_FROM_F64
        ),
        NumericKind::Int32 => dispatch_to_target!(
            Int32,
            source,
            val,
            TO_INT32_FROM_I8,
            TO_INT32_FROM_I16,
            TO_INT32_FROM_I32,
            TO_INT32_FROM_I64,
            TO_INT32_FROM_I128,
            TO_INT32_FROM_U8,
            TO_INT32_FROM_U16,
            TO_INT32_FROM_U32,
            TO_INT32_FROM_U64,
            TO_INT32_FROM_U128,
            TO_INT32_FROM_F32,
            TO_INT32_FROM_F64
        ),
        NumericKind::Int64 => dispatch_to_target!(
            Int64,
            source,
            val,
            TO_INT64_FROM_I8,
            TO_INT64_FROM_I16,
            TO_INT64_FROM_I32,
            TO_INT64_FROM_I64,
            TO_INT64_FROM_I128,
            TO_INT64_FROM_U8,
            TO_INT64_FROM_U16,
            TO_INT64_FROM_U32,
            TO_INT64_FROM_U64,
            TO_INT64_FROM_U128,
            TO_INT64_FROM_F32,
            TO_INT64_FROM_F64
        ),
        NumericKind::Int128 => dispatch_to_target!(
            Int128,
            source,
            val,
            TO_INT128_FROM_I8,
            TO_INT128_FROM_I16,
            TO_INT128_FROM_I32,
            TO_INT128_FROM_I64,
            TO_INT128_FROM_I128,
            TO_INT128_FROM_U8,
            TO_INT128_FROM_U16,
            TO_INT128_FROM_U32,
            TO_INT128_FROM_U64,
            TO_INT128_FROM_U128,
            TO_INT128_FROM_F32,
            TO_INT128_FROM_F64
        ),
        NumericKind::Uint8 => dispatch_to_target!(
            Uint8,
            source,
            val,
            TO_UINT8_FROM_I8,
            TO_UINT8_FROM_I16,
            TO_UINT8_FROM_I32,
            TO_UINT8_FROM_I64,
            TO_UINT8_FROM_I128,
            TO_UINT8_FROM_U8,
            TO_UINT8_FROM_U16,
            TO_UINT8_FROM_U32,
            TO_UINT8_FROM_U64,
            TO_UINT8_FROM_U128,
            TO_UINT8_FROM_F32,
            TO_UINT8_FROM_F64
        ),
        NumericKind::Uint16 => dispatch_to_target!(
            Uint16,
            source,
            val,
            TO_UINT16_FROM_I8,
            TO_UINT16_FROM_I16,
            TO_UINT16_FROM_I32,
            TO_UINT16_FROM_I64,
            TO_UINT16_FROM_I128,
            TO_UINT16_FROM_U8,
            TO_UINT16_FROM_U16,
            TO_UINT16_FROM_U32,
            TO_UINT16_FROM_U64,
            TO_UINT16_FROM_U128,
            TO_UINT16_FROM_F32,
            TO_UINT16_FROM_F64
        ),
        NumericKind::Uint32 => dispatch_to_target!(
            Uint32,
            source,
            val,
            TO_UINT32_FROM_I8,
            TO_UINT32_FROM_I16,
            TO_UINT32_FROM_I32,
            TO_UINT32_FROM_I64,
            TO_UINT32_FROM_I128,
            TO_UINT32_FROM_U8,
            TO_UINT32_FROM_U16,
            TO_UINT32_FROM_U32,
            TO_UINT32_FROM_U64,
            TO_UINT32_FROM_U128,
            TO_UINT32_FROM_F32,
            TO_UINT32_FROM_F64
        ),
        NumericKind::Uint64 => dispatch_to_target!(
            Uint64,
            source,
            val,
            TO_UINT64_FROM_I8,
            TO_UINT64_FROM_I16,
            TO_UINT64_FROM_I32,
            TO_UINT64_FROM_I64,
            TO_UINT64_FROM_I128,
            TO_UINT64_FROM_U8,
            TO_UINT64_FROM_U16,
            TO_UINT64_FROM_U32,
            TO_UINT64_FROM_U64,
            TO_UINT64_FROM_U128,
            TO_UINT64_FROM_F32,
            TO_UINT64_FROM_F64
        ),
        NumericKind::Uint128 => dispatch_to_target!(
            Uint128,
            source,
            val,
            TO_UINT128_FROM_I8,
            TO_UINT128_FROM_I16,
            TO_UINT128_FROM_I32,
            TO_UINT128_FROM_I64,
            TO_UINT128_FROM_I128,
            TO_UINT128_FROM_U8,
            TO_UINT128_FROM_U16,
            TO_UINT128_FROM_U32,
            TO_UINT128_FROM_U64,
            TO_UINT128_FROM_U128,
            TO_UINT128_FROM_F32,
            TO_UINT128_FROM_F64
        ),
        NumericKind::Float32 => dispatch_to_target!(
            Float32,
            source,
            val,
            TO_FLOAT32_FROM_I8,
            TO_FLOAT32_FROM_I16,
            TO_FLOAT32_FROM_I32,
            TO_FLOAT32_FROM_I64,
            TO_FLOAT32_FROM_I128,
            TO_FLOAT32_FROM_U8,
            TO_FLOAT32_FROM_U16,
            TO_FLOAT32_FROM_U32,
            TO_FLOAT32_FROM_U64,
            TO_FLOAT32_FROM_U128,
            TO_FLOAT32_FROM_F32,
            TO_FLOAT32_FROM_F64
        ),
        NumericKind::Float64 => dispatch_to_target!(
            Float64,
            source,
            val,
            TO_FLOAT64_FROM_I8,
            TO_FLOAT64_FROM_I16,
            TO_FLOAT64_FROM_I32,
            TO_FLOAT64_FROM_I64,
            TO_FLOAT64_FROM_I128,
            TO_FLOAT64_FROM_U8,
            TO_FLOAT64_FROM_U16,
            TO_FLOAT64_FROM_U32,
            TO_FLOAT64_FROM_U64,
            TO_FLOAT64_FROM_U128,
            TO_FLOAT64_FROM_F32,
            TO_FLOAT64_FROM_F64
        ),
    };
    res.map_err(map_conversion_failure)
}

fn validate_and_reorder_fields(
    field_order: &[FieldIndex],
    evaluated_operands: Vec<RuntimeValue>,
) -> Box<[RuntimeValue]> {
    let n = field_order.len();
    assert_eq!(
        evaluated_operands.len(),
        n,
        "evaluated operands count must match field_order len"
    );

    let mut canonical_fields: Vec<Option<RuntimeValue>> = (0..n).map(|_| None).collect();

    for (eval_idx, field_dest) in field_order.iter().enumerate() {
        assert!(
            field_dest.0 < n,
            "field_dest index {} out of bounds for composite size {}",
            field_dest.0,
            n
        );
        assert!(
            canonical_fields[field_dest.0].is_none(),
            "duplicate field_dest index {} in field_order",
            field_dest.0
        );
        canonical_fields[field_dest.0] = Some(evaluated_operands[eval_idx]);
    }

    let mut result = Vec::with_capacity(n);
    for slot in canonical_fields {
        result.push(slot.expect("missing field_dest in field_order permutation"));
    }
    result.into_boxed_slice()
}

pub fn execute_instruction<'compiled, 'bindings>(
    execution: &mut VmExecution<'compiled, 'bindings>,
) -> Result<Option<OwnedValue>, ExecutionFailure> {
    let (function_id_val, instruction_pointer_val, frame_base) = {
        let frame = execution
            .call_frames
            .last()
            .expect("VmExecution must have at least one active CallFrame");
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
        .expect("CallFrame function must exist in CompiledProgram");

    let instruction = function
        .instructions
        .get(instruction_pointer_val)
        .expect("CallFrame instruction pointer must exist in function instructions");

    match instruction {
        // Core data movement — 4
        Instruction::LoadConstant(constant_id) => {
            let constant = execution
                .compiled_program
                .constants
                .get(constant_id.0)
                .expect("ConstantId must exist in constants");

            let runtime_val = match constant {
                Constant::Boolean(b) => RuntimeValue::Boolean(*b),

                Constant::Int8(v) => RuntimeValue::Int8(*v),
                Constant::Int16(v) => RuntimeValue::Int16(*v),
                Constant::Int32(v) => RuntimeValue::Int32(*v),
                Constant::Int64(v) => RuntimeValue::Int64(*v),
                Constant::Int128(v) => RuntimeValue::Int128(*v),

                Constant::Uint8(v) => RuntimeValue::Uint8(*v),
                Constant::Uint16(v) => RuntimeValue::Uint16(*v),
                Constant::Uint32(v) => RuntimeValue::Uint32(*v),
                Constant::Uint64(v) => RuntimeValue::Uint64(*v),
                Constant::Uint128(v) => RuntimeValue::Uint128(*v),

                Constant::Float32(v) => RuntimeValue::Float32(*v),
                Constant::Float64(v) => RuntimeValue::Float64(*v),

                Constant::String(_) => {
                    RuntimeValue::String(StringBackingRef::Compiled(ConstantId(constant_id.0)))
                }

                Constant::Dynamic(dyn_c) => match dyn_c {
                    DynamicConstant::Integer { .. } => {
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                            DynamicIntegerBackingRef::Compiled(ConstantId(constant_id.0)),
                        ))
                    }
                    DynamicConstant::Float32(v) => {
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(*v))
                    }
                    DynamicConstant::Float64(v) => {
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(*v))
                    }
                },
            };

            push_operand(execution, runtime_val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LoadParameter(slot) => {
            assert!(
                slot.0 < function.parameter_count,
                "LoadParameter slot {} out of bounds for parameter_count {}",
                slot.0,
                function.parameter_count
            );
            let abs_cell = frame_base + slot.0;
            let val = execution.value_storage.cells[abs_cell]
                .expect("Parameter cell must contain Some(RuntimeValue)");
            push_operand(execution, val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LoadLocal(slot) => {
            assert!(
                slot.0 < function.local_count,
                "LoadLocal slot {} out of bounds for local_count {}",
                slot.0,
                function.local_count
            );
            let abs_cell = frame_base + function.parameter_count + slot.0;
            let val = execution.value_storage.cells[abs_cell]
                .expect("Local cell must contain Some(RuntimeValue)");
            push_operand(execution, val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::StoreLocal(slot) => {
            assert!(
                slot.0 < function.local_count,
                "StoreLocal slot {} out of bounds for local_count {}",
                slot.0,
                function.local_count
            );
            let abs_cell = frame_base + function.parameter_count + slot.0;
            assert!(
                execution.value_storage.cells[abs_cell].is_none(),
                "StoreLocal target cell must be None"
            );
            let val = pop_operand(execution);
            execution.value_storage.cells[abs_cell] = Some(val);
            advance_ip(execution);
            Ok(None)
        }

        // Calls — 2
        Instruction::Call(target_id) => {
            let caller_operand_base = frame_base + function.parameter_count + function.local_count;
            let caller_operand_depth = execution.value_storage.cells.len() - caller_operand_base;

            let target_func = execution
                .compiled_program
                .functions
                .get(target_id.0)
                .expect("Call target FunctionId must exist in CompiledProgram");

            let param_count = target_func.parameter_count;
            let local_count = target_func.local_count;

            assert!(
                caller_operand_depth >= param_count,
                "Insufficient caller operand depth {} for target parameter_count {}",
                caller_operand_depth,
                param_count
            );

            let callee_frame_base = execution.value_storage.cells.len() - param_count;

            for i in 0..param_count {
                assert!(
                    execution.value_storage.cells[callee_frame_base + i].is_some(),
                    "Callee parameter cell must contain Some(RuntimeValue)"
                );
            }

            for _ in 0..local_count {
                execution.value_storage.cells.push(None);
            }

            execution.call_frames.push(CallFrame {
                function: FunctionId(target_id.0),
                instruction_pointer: InstructionPointer(0),
                frame_base: callee_frame_base,
            });

            Ok(None)
        }

        Instruction::CallExternal(_) => {
            panic!(
                "Instruction::CallExternal belongs to external_call_resolver (ESE-062) and cannot be executed by instruction_executor"
            );
        }

        // Fixed numeric — 12
        Instruction::Negate(kind) => {
            let operand = pop_operand(execution);
            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => NEGATE_I8(v)
                    .map(RuntimeValue::Int8)
                    .map_err(map_numeric_failure),
                (NumericKind::Int16, RuntimeValue::Int16(v)) => NEGATE_I16(v)
                    .map(RuntimeValue::Int16)
                    .map_err(map_numeric_failure),
                (NumericKind::Int32, RuntimeValue::Int32(v)) => NEGATE_I32(v)
                    .map(RuntimeValue::Int32)
                    .map_err(map_numeric_failure),
                (NumericKind::Int64, RuntimeValue::Int64(v)) => NEGATE_I64(v)
                    .map(RuntimeValue::Int64)
                    .map_err(map_numeric_failure),
                (NumericKind::Int128, RuntimeValue::Int128(v)) => NEGATE_I128(v)
                    .map(RuntimeValue::Int128)
                    .map_err(map_numeric_failure),
                (NumericKind::Float32, RuntimeValue::Float32(v)) => {
                    Ok(RuntimeValue::Float32(NEGATE_F32(v)))
                }
                (NumericKind::Float64, RuntimeValue::Float64(v)) => {
                    Ok(RuntimeValue::Float64(NEGATE_F64(v)))
                }
                _ => panic!("Negate: operand family mismatch or unsupported unsigned negation"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::Add(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => ADD_I8(l, r)
                    .map(RuntimeValue::Int8)
                    .map_err(map_numeric_failure),
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    ADD_I16(l, r)
                        .map(RuntimeValue::Int16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    ADD_I32(l, r)
                        .map(RuntimeValue::Int32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    ADD_I64(l, r)
                        .map(RuntimeValue::Int64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    ADD_I128(l, r)
                        .map(RuntimeValue::Int128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    ADD_U8(l, r)
                        .map(RuntimeValue::Uint8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    ADD_U16(l, r)
                        .map(RuntimeValue::Uint16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    ADD_U32(l, r)
                        .map(RuntimeValue::Uint32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    ADD_U64(l, r)
                        .map(RuntimeValue::Uint64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    ADD_U128(l, r)
                        .map(RuntimeValue::Uint128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Ok(RuntimeValue::Float32(ADD_F32(l, r)))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Ok(RuntimeValue::Float64(ADD_F64(l, r)))
                }

                _ => panic!("Add: operand family mismatch with NumericKind"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::Subtract(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    SUBTRACT_I8(l, r)
                        .map(RuntimeValue::Int8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    SUBTRACT_I16(l, r)
                        .map(RuntimeValue::Int16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    SUBTRACT_I32(l, r)
                        .map(RuntimeValue::Int32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    SUBTRACT_I64(l, r)
                        .map(RuntimeValue::Int64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    SUBTRACT_I128(l, r)
                        .map(RuntimeValue::Int128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    SUBTRACT_U8(l, r)
                        .map(RuntimeValue::Uint8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    SUBTRACT_U16(l, r)
                        .map(RuntimeValue::Uint16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    SUBTRACT_U32(l, r)
                        .map(RuntimeValue::Uint32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    SUBTRACT_U64(l, r)
                        .map(RuntimeValue::Uint64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    SUBTRACT_U128(l, r)
                        .map(RuntimeValue::Uint128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Ok(RuntimeValue::Float32(SUBTRACT_F32(l, r)))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Ok(RuntimeValue::Float64(SUBTRACT_F64(l, r)))
                }

                _ => panic!("Subtract: operand family mismatch with NumericKind"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::Multiply(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    MULTIPLY_I8(l, r)
                        .map(RuntimeValue::Int8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    MULTIPLY_I16(l, r)
                        .map(RuntimeValue::Int16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    MULTIPLY_I32(l, r)
                        .map(RuntimeValue::Int32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    MULTIPLY_I64(l, r)
                        .map(RuntimeValue::Int64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    MULTIPLY_I128(l, r)
                        .map(RuntimeValue::Int128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    MULTIPLY_U8(l, r)
                        .map(RuntimeValue::Uint8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    MULTIPLY_U16(l, r)
                        .map(RuntimeValue::Uint16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    MULTIPLY_U32(l, r)
                        .map(RuntimeValue::Uint32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    MULTIPLY_U64(l, r)
                        .map(RuntimeValue::Uint64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    MULTIPLY_U128(l, r)
                        .map(RuntimeValue::Uint128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Ok(RuntimeValue::Float32(MULTIPLY_F32(l, r)))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Ok(RuntimeValue::Float64(MULTIPLY_F64(l, r)))
                }

                _ => panic!("Multiply: operand family mismatch with NumericKind"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::Divide(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);

            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    DIVIDE_I8(l, r)
                        .map(RuntimeValue::Int8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    DIVIDE_I16(l, r)
                        .map(RuntimeValue::Int16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    DIVIDE_I32(l, r)
                        .map(RuntimeValue::Int32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    DIVIDE_I64(l, r)
                        .map(RuntimeValue::Int64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    DIVIDE_I128(l, r)
                        .map(RuntimeValue::Int128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    DIVIDE_U8(l, r)
                        .map(RuntimeValue::Uint8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    DIVIDE_U16(l, r)
                        .map(RuntimeValue::Uint16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    DIVIDE_U32(l, r)
                        .map(RuntimeValue::Uint32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    DIVIDE_U64(l, r)
                        .map(RuntimeValue::Uint64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    DIVIDE_U128(l, r)
                        .map(RuntimeValue::Uint128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Ok(RuntimeValue::Float32(DIVIDE_F32(l, r)))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Ok(RuntimeValue::Float64(DIVIDE_F64(l, r)))
                }

                _ => panic!("Divide: operand family mismatch with NumericKind"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::Remainder(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);

            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    REMAINDER_I8(l, r)
                        .map(RuntimeValue::Int8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    REMAINDER_I16(l, r)
                        .map(RuntimeValue::Int16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    REMAINDER_I32(l, r)
                        .map(RuntimeValue::Int32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    REMAINDER_I64(l, r)
                        .map(RuntimeValue::Int64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    REMAINDER_I128(l, r)
                        .map(RuntimeValue::Int128)
                        .map_err(map_numeric_failure)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    REMAINDER_U8(l, r)
                        .map(RuntimeValue::Uint8)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    REMAINDER_U16(l, r)
                        .map(RuntimeValue::Uint16)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    REMAINDER_U32(l, r)
                        .map(RuntimeValue::Uint32)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    REMAINDER_U64(l, r)
                        .map(RuntimeValue::Uint64)
                        .map_err(map_numeric_failure)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    REMAINDER_U128(l, r)
                        .map(RuntimeValue::Uint128)
                        .map_err(map_numeric_failure)
                }

                _ => panic!("Remainder: operand family mismatch or unsupported float Remainder"),
            };

            match res {
                Ok(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::EqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let is_equal = expect_comparison_result(EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let is_not_equal = expect_comparison_result(NOT_EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_not_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LessNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let res = expect_comparison_result(LESS(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LessEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let res = expect_comparison_result(LESS_EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::GreaterNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let res = expect_comparison_result(GREATER(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::GreaterEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_val, r_val) = adapt_numeric_operands(left, right, &kind);
            let res = expect_comparison_result(GREATER_EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        // Dynamic numeric — 7
        Instruction::LiftDynamic(kind) => {
            let operand = pop_operand(execution);
            let owned_dynamic = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_I8(v)
                }
                (NumericKind::Int16, RuntimeValue::Int16(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_I16(v)
                }
                (NumericKind::Int32, RuntimeValue::Int32(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_I32(v)
                }
                (NumericKind::Int64, RuntimeValue::Int64(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_I64(v)
                }
                (NumericKind::Int128, RuntimeValue::Int128(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_I128(v)
                }
                (NumericKind::Uint8, RuntimeValue::Uint8(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_U8(v)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_U16(v)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_U32(v)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_U64(v)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(v)) => {
                    conversion::TO_DYNAMIC_INTEGER_FROM_U128(v)
                }
                (NumericKind::Float32, RuntimeValue::Float32(v)) => {
                    conversion::TO_DYNAMIC_FLOAT32_FROM_F32(v)
                }
                (NumericKind::Float64, RuntimeValue::Float64(v)) => {
                    conversion::TO_DYNAMIC_FLOAT64_FROM_F64(v)
                }
                _ => panic!("LiftDynamic: operand family mismatch with NumericKind"),
            };

            let rt = MATERIALIZE_OWNED_VALUE(
                OwnedValue::Dynamic(owned_dynamic),
                &mut execution.backing_store,
            );
            push_operand(execution, rt);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::DynamicNegate => {
            let operand = pop_operand(execution);
            let owned_dyn = {
                let obs = OBSERVE_RUNTIME_VALUE(
                    operand,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let dyn_val = match obs {
                    Value::Dynamic(d) => d,
                    _ => panic!("DynamicNegate expected Dynamic runtime value"),
                };
                dynamic_numeric::DYNAMIC_NEGATE(&dyn_val)
            };

            let rt = MATERIALIZE_OWNED_VALUE(
                OwnedValue::Dynamic(owned_dyn),
                &mut execution.backing_store,
            );
            push_operand(execution, rt);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::DynamicAdd => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let l_obs = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let r_obs = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let (l_dyn, r_dyn) = match (l_obs, r_obs) {
                    (Value::Dynamic(l), Value::Dynamic(r)) => (l, r),
                    _ => panic!("DynamicAdd expected Dynamic runtime values"),
                };
                dynamic_numeric::DYNAMIC_ADD(&l_dyn, &r_dyn)
            };

            match result {
                Ok(owned_dyn) => {
                    let rt = MATERIALIZE_OWNED_VALUE(
                        OwnedValue::Dynamic(owned_dyn),
                        &mut execution.backing_store,
                    );
                    push_operand(execution, rt);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_dynamic_numeric_failure(fail),
                )),
            }
        }

        Instruction::DynamicSubtract => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let l_obs = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let r_obs = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let (l_dyn, r_dyn) = match (l_obs, r_obs) {
                    (Value::Dynamic(l), Value::Dynamic(r)) => (l, r),
                    _ => panic!("DynamicSubtract expected Dynamic runtime values"),
                };
                dynamic_numeric::DYNAMIC_SUBTRACT(&l_dyn, &r_dyn)
            };

            match result {
                Ok(owned_dyn) => {
                    let rt = MATERIALIZE_OWNED_VALUE(
                        OwnedValue::Dynamic(owned_dyn),
                        &mut execution.backing_store,
                    );
                    push_operand(execution, rt);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_dynamic_numeric_failure(fail),
                )),
            }
        }

        Instruction::DynamicMultiply => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let l_obs = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let r_obs = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let (l_dyn, r_dyn) = match (l_obs, r_obs) {
                    (Value::Dynamic(l), Value::Dynamic(r)) => (l, r),
                    _ => panic!("DynamicMultiply expected Dynamic runtime values"),
                };
                dynamic_numeric::DYNAMIC_MULTIPLY(&l_dyn, &r_dyn)
            };

            match result {
                Ok(owned_dyn) => {
                    let rt = MATERIALIZE_OWNED_VALUE(
                        OwnedValue::Dynamic(owned_dyn),
                        &mut execution.backing_store,
                    );
                    push_operand(execution, rt);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_dynamic_numeric_failure(fail),
                )),
            }
        }

        Instruction::DynamicDivide => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let l_obs = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let r_obs = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let (l_dyn, r_dyn) = match (l_obs, r_obs) {
                    (Value::Dynamic(l), Value::Dynamic(r)) => (l, r),
                    _ => panic!("DynamicDivide expected Dynamic runtime values"),
                };
                dynamic_numeric::DYNAMIC_DIVIDE(&l_dyn, &r_dyn)
            };

            match result {
                Ok(owned_dyn) => {
                    let rt = MATERIALIZE_OWNED_VALUE(
                        OwnedValue::Dynamic(owned_dyn),
                        &mut execution.backing_store,
                    );
                    push_operand(execution, rt);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_dynamic_numeric_failure(fail),
                )),
            }
        }

        Instruction::DynamicRemainder => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            match (&left, &right) {
                (
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(_)),
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(_)),
                ) => {}
                (RuntimeValue::Dynamic(_), RuntimeValue::Dynamic(_)) => {
                    return Err(make_evaluation_failure(
                        execution,
                        EvaluationFailure::DynamicNumericType,
                    ));
                }
                _ => panic!("DynamicRemainder expected Dynamic runtime values"),
            }

            let result = {
                let l_obs = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let r_obs = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let (l_dyn, r_dyn) = match (l_obs, r_obs) {
                    (Value::Dynamic(l), Value::Dynamic(r)) => (l, r),
                    _ => unreachable!(),
                };
                dynamic_numeric::DYNAMIC_REMAINDER(&l_dyn, &r_dyn)
            };

            match result {
                Ok(owned_dyn) => {
                    let rt = MATERIALIZE_OWNED_VALUE(
                        OwnedValue::Dynamic(owned_dyn),
                        &mut execution.backing_store,
                    );
                    push_operand(execution, rt);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_dynamic_numeric_failure(fail),
                )),
            }
        }

        // Control flow — 4
        Instruction::Jump(target) => {
            jump_ip(execution, target.0);
            Ok(None)
        }

        Instruction::JumpIfFalse(target) => {
            assert!(
                target.0 < function.instructions.len(),
                "JumpIfFalse target {} out of bounds for instruction count {}",
                target.0,
                function.instructions.len()
            );
            let operand = pop_operand(execution);
            let condition = match operand {
                RuntimeValue::Boolean(b) => b,
                _ => panic!("JumpIfFalse expected Boolean operand"),
            };

            if !condition {
                jump_ip(execution, target.0);
            } else {
                advance_ip(execution);
            }
            Ok(None)
        }

        Instruction::Discard => {
            pop_operand(execution);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::Return => {
            let operand_base = frame_base + function.parameter_count + function.local_count;
            let operand_depth = execution.value_storage.cells.len() - operand_base;
            assert_eq!(
                operand_depth, 1,
                "Return requires exactly one operand on the stack, found depth {}",
                operand_depth
            );

            let result = pop_operand(execution);

            if execution.call_frames.len() > 1 {
                let callee_frame = execution.call_frames.pop().unwrap();
                execution
                    .value_storage
                    .cells
                    .truncate(callee_frame.frame_base);
                push_operand(execution, result);
                advance_ip(execution);
                Ok(None)
            } else {
                let owned_result =
                    OWN_RUNTIME_VALUE(result, execution.compiled_program, &execution.backing_store);
                Ok(Some(owned_result))
            }
        }

        // Explicit conversions — 4
        Instruction::ConvertNumeric { source, target } => {
            let operand = pop_operand(execution);
            match convert_fixed_numeric(operand, source, target) {
                Ok(res) => {
                    push_operand(execution, res);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(failure) => Err(make_evaluation_failure(execution, failure)),
            }
        }

        Instruction::ConvertDynamic(target) => {
            let operand = pop_operand(execution);
            let result = {
                let obs = OBSERVE_RUNTIME_VALUE(
                    operand,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let dyn_val = match obs {
                    Value::Dynamic(d) => d,
                    _ => panic!("ConvertDynamic expected Dynamic operand"),
                };
                match target {
                    NumericKind::Int8 => {
                        conversion::TO_INT8_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Int8)
                    }
                    NumericKind::Int16 => {
                        conversion::TO_INT16_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Int16)
                    }
                    NumericKind::Int32 => {
                        conversion::TO_INT32_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Int32)
                    }
                    NumericKind::Int64 => {
                        conversion::TO_INT64_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Int64)
                    }
                    NumericKind::Int128 => {
                        conversion::TO_INT128_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Int128)
                    }
                    NumericKind::Uint8 => {
                        conversion::TO_UINT8_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Uint8)
                    }
                    NumericKind::Uint16 => {
                        conversion::TO_UINT16_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Uint16)
                    }
                    NumericKind::Uint32 => {
                        conversion::TO_UINT32_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Uint32)
                    }
                    NumericKind::Uint64 => {
                        conversion::TO_UINT64_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Uint64)
                    }
                    NumericKind::Uint128 => {
                        conversion::TO_UINT128_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Uint128)
                    }
                    NumericKind::Float32 => {
                        conversion::TO_FLOAT32_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Float32)
                    }
                    NumericKind::Float64 => {
                        conversion::TO_FLOAT64_FROM_DYNAMIC(&dyn_val).map(RuntimeValue::Float64)
                    }
                }
            };

            match result {
                Ok(res) => {
                    push_operand(execution, res);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(fail) => Err(make_evaluation_failure(
                    execution,
                    map_conversion_failure(fail),
                )),
            }
        }

        Instruction::NumericToString(kind) => {
            let operand = pop_operand(execution);
            let str_val = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => conversion::TO_STRING_FROM_I8(v),
                (NumericKind::Int16, RuntimeValue::Int16(v)) => conversion::TO_STRING_FROM_I16(v),
                (NumericKind::Int32, RuntimeValue::Int32(v)) => conversion::TO_STRING_FROM_I32(v),
                (NumericKind::Int64, RuntimeValue::Int64(v)) => conversion::TO_STRING_FROM_I64(v),
                (NumericKind::Int128, RuntimeValue::Int128(v)) => {
                    conversion::TO_STRING_FROM_I128(v)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(v)) => conversion::TO_STRING_FROM_U8(v),
                (NumericKind::Uint16, RuntimeValue::Uint16(v)) => conversion::TO_STRING_FROM_U16(v),
                (NumericKind::Uint32, RuntimeValue::Uint32(v)) => conversion::TO_STRING_FROM_U32(v),
                (NumericKind::Uint64, RuntimeValue::Uint64(v)) => conversion::TO_STRING_FROM_U64(v),
                (NumericKind::Uint128, RuntimeValue::Uint128(v)) => {
                    conversion::TO_STRING_FROM_U128(v)
                }

                (NumericKind::Float32, RuntimeValue::Float32(v)) => {
                    conversion::TO_STRING_FROM_F32(v)
                }
                (NumericKind::Float64, RuntimeValue::Float64(v)) => {
                    conversion::TO_STRING_FROM_F64(v)
                }

                _ => panic!("NumericToString: operand family mismatch with NumericKind"),
            };

            let id = StringBackingId(execution.backing_store.strings.len());
            execution
                .backing_store
                .strings
                .push(str_val.into_boxed_str());
            push_operand(
                execution,
                RuntimeValue::String(StringBackingRef::Execution(id)),
            );
            advance_ip(execution);
            Ok(None)
        }

        Instruction::DynamicToString => {
            let operand = pop_operand(execution);
            let str_val = {
                let obs = OBSERVE_RUNTIME_VALUE(
                    operand,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let dyn_val = match obs {
                    Value::Dynamic(d) => d,
                    _ => panic!("DynamicToString expected Dynamic operand"),
                };
                conversion::TO_STRING_FROM_DYNAMIC(&dyn_val)
            };

            let id = StringBackingId(execution.backing_store.strings.len());
            execution
                .backing_store
                .strings
                .push(str_val.into_boxed_str());
            push_operand(
                execution,
                RuntimeValue::String(StringBackingRef::Execution(id)),
            );
            advance_ip(execution);
            Ok(None)
        }

        // Scalar bool / string — 5
        Instruction::NotBoolean => {
            let operand = pop_operand(execution);
            let b = match operand {
                RuntimeValue::Boolean(val) => val,
                _ => panic!("NotBoolean expected Boolean operand"),
            };
            push_operand(execution, RuntimeValue::Boolean(NOT(b)));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::EqualBoolean => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l, r) = match (left, right) {
                (RuntimeValue::Boolean(l_b), RuntimeValue::Boolean(r_b)) => (l_b, r_b),
                _ => panic!("EqualBoolean expected Boolean operands"),
            };
            let l_val = Value::Boolean(l);
            let r_val = Value::Boolean(r);
            let is_equal = expect_comparison_result(EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualBoolean => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l, r) = match (left, right) {
                (RuntimeValue::Boolean(l_b), RuntimeValue::Boolean(r_b)) => (l_b, r_b),
                _ => panic!("NotEqualBoolean expected Boolean operands"),
            };
            let l_val = Value::Boolean(l);
            let r_val = Value::Boolean(r);
            let is_not_equal = expect_comparison_result(NOT_EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_not_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::EqualString => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_ref, r_ref) = match (left, right) {
                (RuntimeValue::String(l), RuntimeValue::String(r)) => (l, r),
                _ => panic!("EqualString expected String operands"),
            };
            let l_str = resolve_string(l_ref, execution.compiled_program, &execution.backing_store);
            let r_str = resolve_string(r_ref, execution.compiled_program, &execution.backing_store);
            let l_val = Value::String(l_str);
            let r_val = Value::String(r_str);
            let is_equal = expect_comparison_result(EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualString => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_ref, r_ref) = match (left, right) {
                (RuntimeValue::String(l), RuntimeValue::String(r)) => (l, r),
                _ => panic!("NotEqualString expected String operands"),
            };
            let l_str = resolve_string(l_ref, execution.compiled_program, &execution.backing_store);
            let r_str = resolve_string(r_ref, execution.compiled_program, &execution.backing_store);
            let l_val = Value::String(l_str);
            let r_val = Value::String(r_str);
            let is_not_equal = expect_comparison_result(NOT_EQUAL(&l_val, &r_val));
            push_operand(execution, RuntimeValue::Boolean(is_not_equal));
            advance_ip(execution);
            Ok(None)
        }

        // Composite mechanics — 8
        Instruction::ConstructStruct { field_order } => {
            let n = field_order.len();
            let mut evaluated = Vec::with_capacity(n);
            for _ in 0..n {
                evaluated.push(pop_operand(execution));
            }
            evaluated.reverse();

            let canonical_fields = validate_and_reorder_fields(field_order, evaluated);

            let id = StructBackingId(execution.backing_store.structs.len());
            execution.backing_store.structs.push(StructBacking {
                fields: canonical_fields,
            });

            push_operand(execution, RuntimeValue::Struct(id));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::GetField(field_index) => {
            let operand = pop_operand(execution);
            let struct_id = match operand {
                RuntimeValue::Struct(id) => id,
                _ => panic!("GetField expected Struct runtime value"),
            };

            let field_val = execution.backing_store.structs[struct_id.0].fields[field_index.0];
            push_operand(execution, field_val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::ConstructEnumSimple(variant) => {
            let id = EnumBackingId(execution.backing_store.enums.len());
            execution.backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(variant.0),
                payload: RuntimeEnumPayload::Simple,
            });

            push_operand(execution, RuntimeValue::Enum(id));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::ConstructEnumAssociated(variant) => {
            let payload_val = pop_operand(execution);
            let id = EnumBackingId(execution.backing_store.enums.len());
            execution.backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(variant.0),
                payload: RuntimeEnumPayload::Associated(payload_val),
            });

            push_operand(execution, RuntimeValue::Enum(id));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::ConstructEnumStructured {
            variant,
            field_order,
        } => {
            let n = field_order.len();
            let mut evaluated = Vec::with_capacity(n);
            for _ in 0..n {
                evaluated.push(pop_operand(execution));
            }
            evaluated.reverse();

            let canonical_fields = validate_and_reorder_fields(field_order, evaluated);

            let id = EnumBackingId(execution.backing_store.enums.len());
            execution.backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(variant.0),
                payload: RuntimeEnumPayload::Structured {
                    fields: canonical_fields,
                },
            });

            push_operand(execution, RuntimeValue::Enum(id));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::TestVariant(variant) => {
            let operand = pop_operand(execution);
            let enum_id = match operand {
                RuntimeValue::Enum(id) => id,
                _ => panic!("TestVariant expected Enum runtime value"),
            };

            let matches = execution.backing_store.enums[enum_id.0].variant.0 == variant.0;
            push_operand(execution, RuntimeValue::Enum(enum_id));
            push_operand(execution, RuntimeValue::Boolean(matches));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::ExtractEnumAssociated => {
            let operand = pop_operand(execution);
            let enum_id = match operand {
                RuntimeValue::Enum(id) => id,
                _ => panic!("ExtractEnumAssociated expected Enum runtime value"),
            };

            let payload_val = match &execution.backing_store.enums[enum_id.0].payload {
                RuntimeEnumPayload::Associated(val) => *val,
                _ => panic!("ExtractEnumAssociated expected Associated payload"),
            };

            push_operand(execution, payload_val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::ExtractEnumStructured { fields } => {
            let operand = pop_operand(execution);
            let enum_id = match operand {
                RuntimeValue::Enum(id) => id,
                _ => panic!("ExtractEnumStructured expected Enum runtime value"),
            };

            let struct_fields = match &execution.backing_store.enums[enum_id.0].payload {
                RuntimeEnumPayload::Structured { fields: f } => {
                    fields.iter().map(|f_idx| f[f_idx.0]).collect::<Vec<_>>()
                }
                _ => panic!("ExtractEnumStructured expected Structured payload"),
            };

            for val in struct_fields {
                push_operand(execution, val);
            }

            advance_ip(execution);
            Ok(None)
        }

        // Structural equality — 2
        Instruction::EqualComposite => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let left_value = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let right_value = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                expect_comparison_result(EQUAL(&left_value, &right_value))
            };

            push_operand(execution, RuntimeValue::Boolean(result));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualComposite => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let result = {
                let left_value = OBSERVE_RUNTIME_VALUE(
                    left,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                let right_value = OBSERVE_RUNTIME_VALUE(
                    right,
                    execution.compiled_program,
                    &execution.backing_store,
                );
                expect_comparison_result(NOT_EQUAL(&left_value, &right_value))
            };

            push_operand(execution, RuntimeValue::Boolean(result));
            advance_ip(execution);
            Ok(None)
        }
    }
}

pub const EXECUTE_INSTRUCTION: ExecuteInstruction = execute_instruction;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::data::compiled::boundary::CompiledValueShape;
    use crate::data::compiled::identities::{
        ConstantId, ExternalSymbolId, InstructionIndex, LocalSlot, ParameterSlot,
    };
    use crate::data::compiled::instructions::Instruction;
    use crate::data::compiled::program::CompiledFunction;
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::lexical::SourceSpan;
    use crate::data::semantic::ids::FunctionId;
    use crate::data::vm::bindings::ApplicationBindings;
    use crate::data::vm::state::{CallFrame, SharedValueStorage};

    #[test]
    fn typed_binding() {
        let implementation: ExecuteInstruction = execute_instruction;
        let binding: ExecuteInstruction = EXECUTE_INSTRUCTION;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn load_constant_and_ip_advance() {
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
            constants: vec![Constant::Int32(123)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 5 },
                    SourceSpan { start: 5, end: 10 },
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

        let outcome = match execute_instruction(&mut execution) {
            Ok(val) => val,
            Err(_) => panic!("instruction should execute"),
        };
        assert!(outcome.is_none());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 1);
        assert_eq!(execution.value_storage.cells.len(), 1);
        match execution.value_storage.cells[0] {
            Some(RuntimeValue::Int32(v)) => assert_eq!(v, 123),
            _ => panic!("expected Int32(123)"),
        }
    }

    #[test]
    fn internal_call_and_return() {
        let program = CompiledProgram {
            functions: vec![
                CompiledFunction {
                    parameter_count: 0,
                    local_count: 0,
                    max_operand_depth: 2,
                    instructions: vec![
                        Instruction::LoadConstant(ConstantId(0)),
                        Instruction::Call(FunctionId(1)),
                        Instruction::Return,
                    ],
                },
                CompiledFunction {
                    parameter_count: 1,
                    local_count: 0,
                    max_operand_depth: 1,
                    instructions: vec![
                        Instruction::LoadParameter(ParameterSlot(0)),
                        Instruction::Return,
                    ],
                },
            ],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(99)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![
                    vec![
                        SourceSpan { start: 0, end: 1 },
                        SourceSpan { start: 1, end: 2 },
                        SourceSpan { start: 2, end: 3 },
                    ],
                    vec![
                        SourceSpan { start: 3, end: 4 },
                        SourceSpan { start: 4, end: 5 },
                    ],
                ],
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

        // 1. LoadConstant(99)
        match execute_instruction(&mut execution) {
            Ok(_) => {}
            Err(_) => panic!("step 1 failed"),
        }
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 1);

        // 2. Call(FunctionId(1))
        match execute_instruction(&mut execution) {
            Ok(_) => {}
            Err(_) => panic!("step 2 failed"),
        }
        assert_eq!(execution.call_frames.len(), 2);
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 1); // caller IP not advanced yet
        assert_eq!(execution.call_frames[1].function.0, 1);
        assert_eq!(execution.call_frames[1].instruction_pointer.0, 0);
        assert_eq!(execution.call_frames[1].frame_base, 0);

        // 3. LoadParameter(0) in callee
        match execute_instruction(&mut execution) {
            Ok(_) => {}
            Err(_) => panic!("step 3 failed"),
        }
        assert_eq!(execution.call_frames[1].instruction_pointer.0, 1);

        // 4. Return in callee -> pops callee frame, caller IP becomes 2
        let callee_ret = match execute_instruction(&mut execution) {
            Ok(v) => v,
            Err(_) => panic!("step 4 failed"),
        };
        assert!(callee_ret.is_none());
        assert_eq!(execution.call_frames.len(), 1);
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 2);

        // 5. Return in entry frame -> execution completes
        let final_ret = match execute_instruction(&mut execution) {
            Ok(v) => v,
            Err(_) => panic!("step 5 failed"),
        };
        match final_ret {
            Some(OwnedValue::Int32(v)) => assert_eq!(v, 99),
            _ => panic!("expected OwnedValue::Int32(99)"),
        }
    }

    #[test]
    fn evaluation_failure_preserves_ip_and_locates_source_span() {
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

        match execute_instruction(&mut execution) {
            Ok(_) => {}
            Err(_) => panic!("load 10 failed"),
        }
        match execute_instruction(&mut execution) {
            Ok(_) => {}
            Err(_) => panic!("load 0 failed"),
        }

        assert_eq!(execution.call_frames[0].instruction_pointer.0, 2);

        let err = match execute_instruction(&mut execution) {
            Ok(_) => panic!("division by zero should fail"),
            Err(e) => e,
        };
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 2); // IP unchanged on failure
        match err.kind {
            ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
            _ => panic!("expected DivisionByZero failure"),
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
    #[should_panic(expected = "Instruction::CallExternal belongs to external_call_resolver")]
    fn call_external_rejected_as_invariant() {
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

        let _ = execute_instruction(&mut execution);
    }

    // Regression tests for Correction 01:

    #[test]
    #[should_panic(
        expected = "convert_fixed_numeric: runtime value family does not match source NumericKind"
    )]
    fn regression_convert_numeric_identity_validates_runtime_value() {
        let _ = convert_fixed_numeric(
            RuntimeValue::Boolean(true),
            &NumericKind::Int32,
            &NumericKind::Int32,
        );
    }

    #[test]
    fn regression_big_dynamic_integer_exact_to_float() {
        use alloc::borrow::Cow;
        use evo_values::{DynamicIntegerValue, DynamicValue};

        // 2^200 -> Float64 succeeds
        let mut mag_2_200 = vec![0u8; 26];
        mag_2_200[0] = 1;
        let val_2_200 = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&mag_2_200),
        ));
        let res_f64 = conversion::TO_FLOAT64_FROM_DYNAMIC(&val_2_200);
        assert!(res_f64.is_ok());
        let f = res_f64.unwrap();
        assert!(f.is_finite());
        assert_eq!(f, 2.0f64.powi(200));

        // -2^200 -> Float64 succeeds
        let val_neg_2_200 = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            true,
            Cow::Borrowed(&mag_2_200),
        ));
        let res_neg_f64 = conversion::TO_FLOAT64_FROM_DYNAMIC(&val_neg_2_200);
        assert!(res_neg_f64.is_ok());
        let neg_f = res_neg_f64.unwrap();
        assert!(neg_f.is_finite());
        assert_eq!(neg_f, -2.0f64.powi(200));

        // (2^200) + 1 -> Float64 conversion error
        let mut mag_2_200_plus_1 = mag_2_200.clone();
        mag_2_200_plus_1[25] = 1;
        let val_plus_1 = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&mag_2_200_plus_1),
        ));
        let res_plus_1 = conversion::TO_FLOAT64_FROM_DYNAMIC(&val_plus_1);
        assert!(res_plus_1.is_err());

        // 2^127 -> Float32 succeeds
        let mut mag_2_127 = vec![0u8; 16];
        mag_2_127[0] = 0x80;
        let val_2_127 = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&mag_2_127),
        ));
        let res_f32 = conversion::TO_FLOAT32_FROM_DYNAMIC(&val_2_127);
        assert!(res_f32.is_ok());
        let f32_val = res_f32.unwrap();
        assert!(f32_val.is_finite());
        assert_eq!(f32_val, 2.0f32.powi(127));

        // 2^128 -> Float32 conversion error (exceeds finite f32)
        let mut mag_2_128 = vec![0u8; 17];
        mag_2_128[0] = 1;
        let val_2_128 = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&mag_2_128),
        ));
        let res_f32_overflow = conversion::TO_FLOAT32_FROM_DYNAMIC(&val_2_128);
        assert!(res_f32_overflow.is_err());
    }

    #[test]
    fn regression_fixed_float_overflow_produces_infinity() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::Multiply(NumericKind::Float32),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Float32(f32::MAX), Constant::Float32(2.0)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Float32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
                    SourceSpan { start: 3, end: 4 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);
        let outcome = execute_instruction(&mut execution);
        assert!(outcome.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 3);
        match execution.value_storage.cells.last() {
            Some(Some(RuntimeValue::Float32(f))) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity"),
        }
    }

    #[test]
    fn regression_dynamic_float_overflow_produces_infinity() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::DynamicAdd,
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![
                Constant::Dynamic(DynamicConstant::Float64(f64::MAX)),
                Constant::Dynamic(DynamicConstant::Float64(f64::MAX)),
            ],
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
                    SourceSpan { start: 3, end: 4 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);
        let outcome = execute_instruction(&mut execution);
        assert!(outcome.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 3);
        match execution.value_storage.cells.last() {
            Some(Some(RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(f)))) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected dynamic float +Infinity"),
        }
    }

    #[test]
    #[should_panic(expected = "LoadParameter slot 1 out of bounds for parameter_count 1")]
    fn regression_invalid_parameter_slot_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 1,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::LoadParameter(ParameterSlot(1))],
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
            value_storage: SharedValueStorage {
                cells: vec![Some(RuntimeValue::Int32(1))],
            },
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

        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "LoadLocal slot 2 out of bounds for local_count 1")]
    fn regression_invalid_local_slot_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 1,
                max_operand_depth: 1,
                instructions: vec![Instruction::LoadLocal(LocalSlot(2))],
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
            value_storage: SharedValueStorage {
                cells: vec![Some(RuntimeValue::Int32(1))],
            },
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

        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "Insufficient caller operand depth 0 for target parameter_count 1")]
    fn regression_call_insufficient_operand_arguments_panics() {
        let program = CompiledProgram {
            functions: vec![
                CompiledFunction {
                    parameter_count: 0,
                    local_count: 0,
                    max_operand_depth: 2,
                    instructions: vec![Instruction::Call(FunctionId(1))],
                },
                CompiledFunction {
                    parameter_count: 1,
                    local_count: 0,
                    max_operand_depth: 1,
                    instructions: vec![Instruction::Return],
                },
            ],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: vec![
                    vec![SourceSpan { start: 0, end: 1 }],
                    vec![SourceSpan { start: 1, end: 2 }],
                ],
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

        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "Operand stack depth 1 exceeded max_operand_depth 1")]
    fn regression_max_operand_depth_exceeded_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(0)),
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(1)],
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution); // should panic on second push
    }

    #[test]
    #[should_panic(expected = "Jump target 99 out of bounds for instruction count 1")]
    fn regression_invalid_jump_target_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::Jump(InstructionIndex(99))],
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

        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "InstructionPointer advance 1 out of bounds for instruction count 1")]
    fn regression_past_end_advance_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: vec![Instruction::LoadConstant(ConstantId(0))],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(1)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
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

        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "Return requires exactly one operand on the stack, found depth 2")]
    fn regression_return_with_multiple_operands_panics() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(1)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);
    }

    #[test]
    #[should_panic(expected = "duplicate field_dest index 0 in field_order")]
    fn regression_duplicate_field_order_panics() {
        let field_order = vec![FieldIndex(0), FieldIndex(0)];
        let operands = vec![RuntimeValue::Int32(1), RuntimeValue::Int32(2)];
        let _ = validate_and_reorder_fields(&field_order, operands);
    }

    #[test]
    fn fixed_integer_overflow_addition() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::Add(NumericKind::Int8),
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int8(127), Constant::Int8(1)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int8],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);

        let err = match execute_instruction(&mut execution) {
            Ok(_) => panic!("overflow should fail"),
            Err(e) => e,
        };
        match err.kind {
            ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
            _ => panic!("expected Overflow failure"),
        }
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 2);
    }

    #[test]
    fn fixed_float_division_by_zero_produces_infinity() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::Divide(NumericKind::Float32),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Float32(1.0), Constant::Float32(0.0)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Float32],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
                    SourceSpan { start: 3, end: 4 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);
        let outcome = execute_instruction(&mut execution);
        assert!(outcome.is_ok());
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 3);
        match execution.value_storage.cells.last() {
            Some(Some(RuntimeValue::Float32(f))) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity"),
        }
    }

    #[test]
    fn dynamic_integer_division_by_zero() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::DynamicDivide,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![
                Constant::Dynamic(DynamicConstant::Integer {
                    negative: false,
                    magnitude: vec![10],
                }),
                Constant::Dynamic(DynamicConstant::Integer {
                    negative: false,
                    magnitude: Vec::new(),
                }),
            ],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Dynamic],
            source_map: SourceMap {
                functions: vec![vec![
                    SourceSpan { start: 0, end: 1 },
                    SourceSpan { start: 1, end: 2 },
                    SourceSpan { start: 2, end: 3 },
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

        let _ = execute_instruction(&mut execution);
        let _ = execute_instruction(&mut execution);

        let err = match execute_instruction(&mut execution) {
            Ok(_) => panic!("dynamic integer division by zero should fail"),
            Err(e) => e,
        };
        match err.kind {
            ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
            _ => panic!("expected DivisionByZero failure"),
        }
        assert_eq!(execution.call_frames[0].instruction_pointer.0, 2);
    }

    fn test_execute_instructions(
        mut instructions: Vec<Instruction>,
        constants: Vec<Constant>,
    ) -> Result<RuntimeValue, ExecutionFailure> {
        instructions.push(Instruction::Return);
        let max_operand_depth = 4;
        let mut source_map_spans = Vec::new();
        for i in 0..instructions.len() {
            source_map_spans.push(SourceSpan {
                start: i,
                end: i + 1,
            });
        }
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth,
                instructions,
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants,
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: vec![source_map_spans],
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

        let count = execution.compiled_program.functions[0].instructions.len() - 1;
        for _ in 0..count {
            match execute_instruction(&mut execution) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        let result = execution
            .value_storage
            .cells
            .pop()
            .expect("stack should have cell")
            .expect("cell should contain value");
        Ok(result)
    }

    #[test]
    fn fixed_integer_signed_negate_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Negate(NumericKind::Int8),
            ],
            vec![Constant::Int8(i8::MIN)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_signed_add_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Add(NumericKind::Int16),
            ],
            vec![Constant::Int16(i16::MAX), Constant::Int16(1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_unsigned_add_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Add(NumericKind::Uint8),
            ],
            vec![Constant::Uint8(u8::MAX), Constant::Uint8(1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_signed_subtract_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Subtract(NumericKind::Int8),
            ],
            vec![Constant::Int8(i8::MIN), Constant::Int8(1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_unsigned_subtract_underflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Subtract(NumericKind::Uint8),
            ],
            vec![Constant::Uint8(0), Constant::Uint8(1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_signed_multiply_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Multiply(NumericKind::Int8),
            ],
            vec![Constant::Int8(i8::MAX), Constant::Int8(2)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_unsigned_multiply_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Multiply(NumericKind::Uint8),
            ],
            vec![Constant::Uint8(u8::MAX), Constant::Uint8(2)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_divide_by_zero() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Int32),
            ],
            vec![Constant::Int32(10), Constant::Int32(0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                _ => panic!("expected DivisionByZero failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_divide_min_by_neg_one_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Int8),
            ],
            vec![Constant::Int8(i8::MIN), Constant::Int8(-1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_remainder_by_zero() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Remainder(NumericKind::Int32),
            ],
            vec![Constant::Int32(10), Constant::Int32(0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                _ => panic!("expected DivisionByZero failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_integer_remainder_min_by_neg_one_overflow() {
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Remainder(NumericKind::Int8),
            ],
            vec![Constant::Int8(i8::MIN), Constant::Int8(-1)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
                _ => panic!("expected Overflow failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn fixed_float_ieee_add_overflow_to_infinity() {
        let res_f32 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Add(NumericKind::Float32),
            ],
            vec![Constant::Float32(f32::MAX), Constant::Float32(f32::MAX)],
        );
        match res_f32 {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f32"),
        }

        let res_f64 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Add(NumericKind::Float64),
            ],
            vec![Constant::Float64(f64::MAX), Constant::Float64(f64::MAX)],
        );
        match res_f64 {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f64"),
        }
    }

    #[test]
    fn fixed_float_ieee_multiply_overflow_to_infinity() {
        let res_f32 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Multiply(NumericKind::Float32),
            ],
            vec![Constant::Float32(f32::MAX), Constant::Float32(2.0)],
        );
        match res_f32 {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f32"),
        }

        let res_f64 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Multiply(NumericKind::Float64),
            ],
            vec![Constant::Float64(f64::MAX), Constant::Float64(2.0)],
        );
        match res_f64 {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f64"),
        }
    }

    #[test]
    fn fixed_float_ieee_divide_by_positive_zero() {
        let res_f32_pos = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float32),
            ],
            vec![Constant::Float32(1.0), Constant::Float32(0.0)],
        );
        match res_f32_pos {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f32"),
        }

        let res_f32_neg = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float32),
            ],
            vec![Constant::Float32(-1.0), Constant::Float32(0.0)],
        );
        match res_f32_neg {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -Infinity f32"),
        }

        let res_f64_pos = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float64),
            ],
            vec![Constant::Float64(1.0), Constant::Float64(0.0)],
        );
        match res_f64_pos {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f64"),
        }

        let res_f64_neg = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float64),
            ],
            vec![Constant::Float64(-1.0), Constant::Float64(0.0)],
        );
        match res_f64_neg {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -Infinity f64"),
        }
    }

    #[test]
    fn fixed_float_ieee_divide_by_negative_zero() {
        let res_f32_pos = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float32),
            ],
            vec![Constant::Float32(1.0), Constant::Float32(-0.0)],
        );
        match res_f32_pos {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -Infinity f32"),
        }

        let res_f32_neg = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float32),
            ],
            vec![Constant::Float32(-1.0), Constant::Float32(-0.0)],
        );
        match res_f32_neg {
            Ok(RuntimeValue::Float32(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f32"),
        }

        let res_f64_pos = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float64),
            ],
            vec![Constant::Float64(1.0), Constant::Float64(-0.0)],
        );
        match res_f64_pos {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -Infinity f64"),
        }

        let res_f64_neg = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float64),
            ],
            vec![Constant::Float64(-1.0), Constant::Float64(-0.0)],
        );
        match res_f64_neg {
            Ok(RuntimeValue::Float64(f)) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected +Infinity f64"),
        }
    }

    #[test]
    fn fixed_float_ieee_zero_divided_by_zero_produces_nan() {
        let res_f32 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float32),
            ],
            vec![Constant::Float32(0.0), Constant::Float32(0.0)],
        );
        match res_f32 {
            Ok(RuntimeValue::Float32(f)) => assert!(f.is_nan()),
            _ => panic!("expected NaN f32"),
        }

        let res_f64 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Divide(NumericKind::Float64),
            ],
            vec![Constant::Float64(0.0), Constant::Float64(0.0)],
        );
        match res_f64 {
            Ok(RuntimeValue::Float64(f)) => assert!(f.is_nan()),
            _ => panic!("expected NaN f64"),
        }
    }

    #[test]
    fn fixed_float_ieee_non_finite_inputs_and_results() {
        let res_f32 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Subtract(NumericKind::Float32),
            ],
            vec![
                Constant::Float32(f32::INFINITY),
                Constant::Float32(f32::INFINITY),
            ],
        );
        match res_f32 {
            Ok(RuntimeValue::Float32(f)) => assert!(f.is_nan()),
            _ => panic!("expected NaN f32 for Infinity - Infinity"),
        }

        let res_f64 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Subtract(NumericKind::Float64),
            ],
            vec![
                Constant::Float64(f64::INFINITY),
                Constant::Float64(f64::INFINITY),
            ],
        );
        match res_f64 {
            Ok(RuntimeValue::Float64(f)) => assert!(f.is_nan()),
            _ => panic!("expected NaN f64 for Infinity - Infinity"),
        }
    }

    #[test]
    fn fixed_float_negate_preserves_signed_zero() {
        let res_f32 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Negate(NumericKind::Float32),
            ],
            vec![Constant::Float32(0.0)],
        );
        match res_f32 {
            Ok(RuntimeValue::Float32(f)) => {
                assert_eq!(f, 0.0);
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -0.0 f32"),
        }

        let res_f64 = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Negate(NumericKind::Float64),
            ],
            vec![Constant::Float64(0.0)],
        );
        match res_f64 {
            Ok(RuntimeValue::Float64(f)) => {
                assert_eq!(f, 0.0);
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected -0.0 f64"),
        }
    }

    #[test]
    fn boolean_not_delegation() {
        let res_true = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::NotBoolean,
            ],
            vec![Constant::Boolean(true)],
        );
        match res_true {
            Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, false),
            _ => panic!("expected Boolean(false)"),
        }

        let res_false = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::NotBoolean,
            ],
            vec![Constant::Boolean(false)],
        );
        match res_false {
            Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, true),
            _ => panic!("expected Boolean(true)"),
        }
    }

    #[test]
    #[should_panic(expected = "Remainder: operand family mismatch or unsupported float Remainder")]
    fn fixed_float32_remainder_unsupported_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Remainder(NumericKind::Float32),
            ],
            vec![Constant::Float32(5.0), Constant::Float32(2.0)],
        );
    }

    #[test]
    #[should_panic(expected = "Remainder: operand family mismatch or unsupported float Remainder")]
    fn fixed_float64_remainder_unsupported_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::Remainder(NumericKind::Float64),
            ],
            vec![Constant::Float64(5.0), Constant::Float64(2.0)],
        );
    }

    #[test]
    #[should_panic(expected = "Negate: operand family mismatch or unsupported unsigned negation")]
    fn fixed_unsigned_negate_unsupported_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Negate(NumericKind::Uint32),
            ],
            vec![Constant::Uint32(5)],
        );
    }

    #[test]
    fn scalar_comparison_numeric_equality_int32_and_float64() {
        let assert_cmp = |inst: Instruction, c1: Constant, c2: Constant, expected: bool| {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    inst,
                ],
                vec![c1, c2],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, expected),
                _ => panic!("expected Boolean({expected})"),
            }
        };

        // Int32
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Int32),
            Constant::Int32(42),
            Constant::Int32(42),
            true,
        );
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Int32),
            Constant::Int32(42),
            Constant::Int32(43),
            false,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Int32),
            Constant::Int32(42),
            Constant::Int32(43),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Int32),
            Constant::Int32(42),
            Constant::Int32(42),
            false,
        );

        // Float64
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float64),
            Constant::Float64(3.5),
            Constant::Float64(3.5),
            true,
        );
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float64),
            Constant::Float64(3.5),
            Constant::Float64(2.25),
            false,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float64),
            Constant::Float64(3.5),
            Constant::Float64(2.25),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float64),
            Constant::Float64(3.5),
            Constant::Float64(3.5),
            false,
        );
    }

    #[test]
    fn scalar_comparison_numeric_ordering_all_families() {
        let assert_cmp = |inst: Instruction, c1: Constant, c2: Constant, expected: bool| {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    inst,
                ],
                vec![c1, c2],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, expected),
                _ => panic!("expected Boolean({expected})"),
            }
        };

        // Signed Int (Int32)
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(20),
            true,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Int32),
            Constant::Int32(20),
            Constant::Int32(10),
            false,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(10),
            false,
        );

        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(20),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(10),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Int32),
            Constant::Int32(20),
            Constant::Int32(10),
            false,
        );

        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Int32),
            Constant::Int32(20),
            Constant::Int32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(20),
            false,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(10),
            false,
        );

        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Int32),
            Constant::Int32(20),
            Constant::Int32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Int32),
            Constant::Int32(10),
            Constant::Int32(20),
            false,
        );

        // Unsigned Int (Uint32)
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(20),
            true,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Uint32),
            Constant::Uint32(20),
            Constant::Uint32(10),
            false,
        );

        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(20),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(10),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(20),
            Constant::Uint32(10),
            false,
        );

        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Uint32),
            Constant::Uint32(20),
            Constant::Uint32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(20),
            false,
        );

        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(20),
            Constant::Uint32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(10),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Uint32),
            Constant::Uint32(10),
            Constant::Uint32(20),
            false,
        );

        // Float (Float64)
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Float64),
            Constant::Float64(1.5),
            Constant::Float64(2.5),
            true,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Float64),
            Constant::Float64(2.5),
            Constant::Float64(1.5),
            false,
        );

        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float64),
            Constant::Float64(1.5),
            Constant::Float64(2.5),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float64),
            Constant::Float64(1.5),
            Constant::Float64(1.5),
            true,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float64),
            Constant::Float64(2.5),
            Constant::Float64(1.5),
            false,
        );

        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Float64),
            Constant::Float64(2.5),
            Constant::Float64(1.5),
            true,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Float64),
            Constant::Float64(1.5),
            Constant::Float64(2.5),
            false,
        );

        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float64),
            Constant::Float64(2.5),
            Constant::Float64(1.5),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float64),
            Constant::Float64(2.5),
            Constant::Float64(2.5),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float64),
            Constant::Float64(1.5),
            Constant::Float64(2.5),
            false,
        );
    }

    #[test]
    fn scalar_comparison_float_ieee_nan_and_zeros() {
        let assert_cmp = |inst: Instruction, c1: Constant, c2: Constant, expected: bool| {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    inst,
                ],
                vec![c1, c2],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, expected),
                _ => panic!("expected Boolean({expected})"),
            }
        };

        // Float32 NaN
        let nan32 = f32::NAN;
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            false,
        );
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(nan32),
            false,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(nan32),
            true,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            false,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Float32),
            Constant::Float32(1.0),
            Constant::Float32(nan32),
            false,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            false,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float32),
            Constant::Float32(1.0),
            Constant::Float32(nan32),
            false,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            false,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Float32),
            Constant::Float32(1.0),
            Constant::Float32(nan32),
            false,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float32),
            Constant::Float32(nan32),
            Constant::Float32(1.0),
            false,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float32),
            Constant::Float32(1.0),
            Constant::Float32(nan32),
            false,
        );

        // Float64 NaN
        let nan64 = f64::NAN;
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            false,
        );
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(nan64),
            false,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(nan64),
            true,
        );
        assert_cmp(
            Instruction::LessNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            false,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            false,
        );
        assert_cmp(
            Instruction::GreaterNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            false,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float64),
            Constant::Float64(nan64),
            Constant::Float64(1.0),
            false,
        );

        // Signed Zeros (+0.0 == -0.0 and +0.0 != -0.0)
        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float32),
            Constant::Float32(0.0),
            Constant::Float32(-0.0),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float32),
            Constant::Float32(0.0),
            Constant::Float32(-0.0),
            false,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float32),
            Constant::Float32(0.0),
            Constant::Float32(-0.0),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float32),
            Constant::Float32(0.0),
            Constant::Float32(-0.0),
            true,
        );

        assert_cmp(
            Instruction::EqualNumeric(NumericKind::Float64),
            Constant::Float64(0.0),
            Constant::Float64(-0.0),
            true,
        );
        assert_cmp(
            Instruction::NotEqualNumeric(NumericKind::Float64),
            Constant::Float64(0.0),
            Constant::Float64(-0.0),
            false,
        );
        assert_cmp(
            Instruction::LessEqualNumeric(NumericKind::Float64),
            Constant::Float64(0.0),
            Constant::Float64(-0.0),
            true,
        );
        assert_cmp(
            Instruction::GreaterEqualNumeric(NumericKind::Float64),
            Constant::Float64(0.0),
            Constant::Float64(-0.0),
            true,
        );
    }

    #[test]
    fn scalar_comparison_boolean_equality() {
        let assert_cmp = |inst: Instruction, c1: Constant, c2: Constant, expected: bool| {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    inst,
                ],
                vec![c1, c2],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, expected),
                _ => panic!("expected Boolean({expected})"),
            }
        };

        assert_cmp(
            Instruction::EqualBoolean,
            Constant::Boolean(true),
            Constant::Boolean(true),
            true,
        );
        assert_cmp(
            Instruction::EqualBoolean,
            Constant::Boolean(true),
            Constant::Boolean(false),
            false,
        );
        assert_cmp(
            Instruction::EqualBoolean,
            Constant::Boolean(false),
            Constant::Boolean(false),
            true,
        );

        assert_cmp(
            Instruction::NotEqualBoolean,
            Constant::Boolean(true),
            Constant::Boolean(false),
            true,
        );
        assert_cmp(
            Instruction::NotEqualBoolean,
            Constant::Boolean(true),
            Constant::Boolean(true),
            false,
        );
        assert_cmp(
            Instruction::NotEqualBoolean,
            Constant::Boolean(false),
            Constant::Boolean(false),
            false,
        );
    }

    #[test]
    fn scalar_comparison_string_equality() {
        let assert_cmp = |inst: Instruction, s1: &str, s2: &str, expected: bool| {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    inst,
                ],
                vec![
                    Constant::String(s1.to_string()),
                    Constant::String(s2.to_string()),
                ],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => assert_eq!(b, expected),
                _ => panic!("expected Boolean({expected})"),
            }
        };

        // Compiled strings
        assert_cmp(Instruction::EqualString, "evo", "evo", true);
        assert_cmp(Instruction::EqualString, "evo", "rust", false);
        assert_cmp(Instruction::NotEqualString, "evo", "rust", true);
        assert_cmp(Instruction::NotEqualString, "evo", "evo", false);

        // Execution strings (via NumericToString) compared against compiled string
        let res_exec_eq = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::NumericToString(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::EqualString,
            ],
            vec![Constant::Int32(42), Constant::String("42".to_string())],
        );
        match res_exec_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true) for execution string EqualString"),
        }

        let res_exec_ne = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::NumericToString(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::NotEqualString,
            ],
            vec![Constant::Int32(42), Constant::String("99".to_string())],
        );
        match res_exec_ne {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true) for execution string NotEqualString"),
        }
    }

    #[test]
    #[should_panic(expected = "Numeric comparison: operand family mismatch with NumericKind")]
    fn scalar_comparison_numeric_kind_mismatch_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::EqualNumeric(NumericKind::Int32),
            ],
            vec![Constant::Int32(42), Constant::Float32(42.0)],
        );
    }

    #[test]
    #[should_panic(expected = "EqualBoolean expected Boolean operands")]
    fn scalar_comparison_boolean_type_mismatch_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::EqualBoolean,
            ],
            vec![Constant::Int32(1), Constant::Boolean(true)],
        );
    }

    #[test]
    #[should_panic(expected = "EqualString expected String operands")]
    fn scalar_comparison_string_type_mismatch_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::EqualString,
            ],
            vec![Constant::Int32(1), Constant::String("test".to_string())],
        );
    }

    #[test]
    fn composite_equality_struct_simple() {
        // Equal
        let res_eq = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![Constant::Int32(10), Constant::String("evo".to_string())],
        );
        match res_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Different int field
        let res_diff_int = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![
                Constant::Int32(10),
                Constant::String("evo".to_string()),
                Constant::Int32(20),
            ],
        );
        match res_diff_int {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }

        // Different string field
        let res_diff_str = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![
                Constant::Int32(10),
                Constant::String("evo".to_string()),
                Constant::String("rust".to_string()),
            ],
        );
        match res_diff_str {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }
    }

    #[test]
    fn composite_equality_struct_nested() {
        // Equal nested struct: { { 10, "evo" }, true }
        let res_eq = test_execute_instructions(
            vec![
                // Left
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                // Right
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![
                Constant::Int32(10),
                Constant::String("evo".to_string()),
                Constant::Boolean(true),
            ],
        );
        match res_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Different nested leaf: { { 10, "evo" }, true } vs { { 20, "evo" }, true }
        let res_diff = test_execute_instructions(
            vec![
                // Left
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                // Right
                Instruction::LoadConstant(ConstantId(3)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![
                Constant::Int32(10),
                Constant::String("evo".to_string()),
                Constant::Boolean(true),
                Constant::Int32(20),
            ],
        );
        match res_diff {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }
    }

    #[test]
    fn composite_equality_enum_simple() {
        let res_eq = test_execute_instructions(
            vec![
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::EqualComposite,
            ],
            vec![],
        );
        match res_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        let res_diff = test_execute_instructions(
            vec![
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::ConstructEnumSimple(VariantDiscriminant(1)),
                Instruction::EqualComposite,
            ],
            vec![],
        );
        match res_diff {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }
    }

    #[test]
    fn composite_equality_enum_associated() {
        // Same variant, same value
        let res_eq = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(0)),
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(0)),
                Instruction::EqualComposite,
            ],
            vec![Constant::Int32(42)],
        );
        match res_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Same variant, different value
        let res_diff_val = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(0)),
                Instruction::EqualComposite,
            ],
            vec![Constant::Int32(42), Constant::Int32(99)],
        );
        match res_diff_val {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }

        // Different variant, same value
        let res_diff_var = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(0)),
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructEnumAssociated(VariantDiscriminant(1)),
                Instruction::EqualComposite,
            ],
            vec![Constant::Int32(42)],
        );
        match res_diff_var {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }
    }

    #[test]
    fn composite_equality_enum_structured() {
        // Same variant, same fields
        let res_eq = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructEnumStructured {
                    variant: VariantDiscriminant(0),
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructEnumStructured {
                    variant: VariantDiscriminant(0),
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![Constant::Int32(1), Constant::Boolean(true)],
        );
        match res_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Same variant, different field
        let res_diff = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructEnumStructured {
                    variant: VariantDiscriminant(0),
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::ConstructEnumStructured {
                    variant: VariantDiscriminant(0),
                    field_order: vec![FieldIndex(0), FieldIndex(1)],
                },
                Instruction::EqualComposite,
            ],
            vec![
                Constant::Int32(1),
                Constant::Boolean(true),
                Constant::Boolean(false),
            ],
        );
        match res_diff {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }
    }

    #[test]
    fn composite_not_equal_struct_and_enum() {
        // Struct equal -> NotEqualComposite returns false
        let res_struct_eq = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0)],
                },
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0)],
                },
                Instruction::NotEqualComposite,
            ],
            vec![Constant::Int32(10)],
        );
        match res_struct_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }

        // Struct different -> NotEqualComposite returns true
        let res_struct_diff = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0)],
                },
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::ConstructStruct {
                    field_order: vec![FieldIndex(0)],
                },
                Instruction::NotEqualComposite,
            ],
            vec![Constant::Int32(10), Constant::Int32(20)],
        );
        match res_struct_diff {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Enum equal -> NotEqualComposite returns false
        let res_enum_eq = test_execute_instructions(
            vec![
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::NotEqualComposite,
            ],
            vec![],
        );
        match res_enum_eq {
            Ok(RuntimeValue::Boolean(b)) => assert!(!b),
            _ => panic!("expected Boolean(false)"),
        }

        // Enum different -> NotEqualComposite returns true
        let res_enum_diff = test_execute_instructions(
            vec![
                Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
                Instruction::ConstructEnumSimple(VariantDiscriminant(1)),
                Instruction::NotEqualComposite,
            ],
            vec![],
        );
        match res_enum_diff {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }
    }

    fn assert_fixed_ok(res: Result<RuntimeValue, EvaluationFailure>, expected: RuntimeValue) {
        match (res, expected) {
            (Ok(RuntimeValue::Int8(a)), RuntimeValue::Int8(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Int16(a)), RuntimeValue::Int16(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Int32(a)), RuntimeValue::Int32(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Int64(a)), RuntimeValue::Int64(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Int128(a)), RuntimeValue::Int128(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Uint8(a)), RuntimeValue::Uint8(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Uint16(a)), RuntimeValue::Uint16(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Uint32(a)), RuntimeValue::Uint32(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Uint64(a)), RuntimeValue::Uint64(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Uint128(a)), RuntimeValue::Uint128(b)) => assert_eq!(a, b),
            (Ok(RuntimeValue::Float32(a)), RuntimeValue::Float32(b)) => {
                assert_eq!(a.to_bits(), b.to_bits())
            }
            (Ok(RuntimeValue::Float64(a)), RuntimeValue::Float64(b)) => {
                assert_eq!(a.to_bits(), b.to_bits())
            }
            _ => panic!("assert_fixed_ok: value mismatch or unexpected Err"),
        }
    }

    fn assert_fixed_conversion_err(res: Result<RuntimeValue, EvaluationFailure>) {
        match res {
            Err(EvaluationFailure::Conversion) => {}
            _ => panic!("expected Err(EvaluationFailure::Conversion)"),
        }
    }

    #[test]
    fn test_convert_numeric_integer_to_integer() {
        // Widening
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int8(42),
                &NumericKind::Int8,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(42),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint8(200),
                &NumericKind::Uint8,
                &NumericKind::Uint64,
            ),
            RuntimeValue::Uint64(200),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int16(-10),
                &NumericKind::Int16,
                &NumericKind::Int128,
            ),
            RuntimeValue::Int128(-10),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint32(1000),
                &NumericKind::Uint32,
                &NumericKind::Uint128,
            ),
            RuntimeValue::Uint128(1000),
        );

        // Narrowing exact
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int32(42),
                &NumericKind::Int32,
                &NumericKind::Int8,
            ),
            RuntimeValue::Int8(42),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint64(255),
                &NumericKind::Uint64,
                &NumericKind::Uint8,
            ),
            RuntimeValue::Uint8(255),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int128(-128),
                &NumericKind::Int128,
                &NumericKind::Int8,
            ),
            RuntimeValue::Int8(-128),
        );

        // Narrowing failure
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int32(300),
            &NumericKind::Int32,
            &NumericKind::Int8,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Uint16(256),
            &NumericKind::Uint16,
            &NumericKind::Uint8,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int64(i64::MAX),
            &NumericKind::Int64,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Uint128(u128::MAX),
            &NumericKind::Uint128,
            &NumericKind::Uint64,
        ));

        // Signed <-> Unsigned failure
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int8(-1),
            &NumericKind::Int8,
            &NumericKind::Uint8,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int32(-100),
            &NumericKind::Int32,
            &NumericKind::Uint32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Uint8(200),
            &NumericKind::Uint8,
            &NumericKind::Int8,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Uint64(u64::MAX),
            &NumericKind::Uint64,
            &NumericKind::Int64,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int64(-1),
            &NumericKind::Int64,
            &NumericKind::Uint64,
        ));

        // Signed <-> Unsigned exact success
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint8(100),
                &NumericKind::Uint8,
                &NumericKind::Int8,
            ),
            RuntimeValue::Int8(100),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int8(100),
                &NumericKind::Int8,
                &NumericKind::Uint8,
            ),
            RuntimeValue::Uint8(100),
        );

        // Identity for all fixed integer kinds
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int8(7),
                &NumericKind::Int8,
                &NumericKind::Int8,
            ),
            RuntimeValue::Int8(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int16(7),
                &NumericKind::Int16,
                &NumericKind::Int16,
            ),
            RuntimeValue::Int16(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int32(7),
                &NumericKind::Int32,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int64(7),
                &NumericKind::Int64,
                &NumericKind::Int64,
            ),
            RuntimeValue::Int64(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int128(7),
                &NumericKind::Int128,
                &NumericKind::Int128,
            ),
            RuntimeValue::Int128(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint8(7),
                &NumericKind::Uint8,
                &NumericKind::Uint8,
            ),
            RuntimeValue::Uint8(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint16(7),
                &NumericKind::Uint16,
                &NumericKind::Uint16,
            ),
            RuntimeValue::Uint16(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint32(7),
                &NumericKind::Uint32,
                &NumericKind::Uint32,
            ),
            RuntimeValue::Uint32(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint64(7),
                &NumericKind::Uint64,
                &NumericKind::Uint64,
            ),
            RuntimeValue::Uint64(7),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint128(7),
                &NumericKind::Uint128,
                &NumericKind::Uint128,
            ),
            RuntimeValue::Uint128(7),
        );
    }

    #[test]
    fn test_convert_numeric_integer_to_float() {
        // 2^24 = 16_777_216 is exact in f32
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int32(16_777_216),
                &NumericKind::Int32,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(16_777_216.0),
        );
        // 2^24 + 1 = 16_777_217 is not exact in f32
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int32(16_777_217),
            &NumericKind::Int32,
            &NumericKind::Float32,
        ));

        // Uint32 to Float32
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint32(16_777_216),
                &NumericKind::Uint32,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(16_777_216.0),
        );
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Uint32(16_777_217),
            &NumericKind::Uint32,
            &NumericKind::Float32,
        ));

        // 2^53 = 9_007_199_254_740_992 is exact in f64
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int64(9_007_199_254_740_992),
                &NumericKind::Int64,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(9_007_199_254_740_992.0),
        );
        // 2^53 + 1 = 9_007_199_254_740_993 is not exact in f64
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Int64(9_007_199_254_740_993),
            &NumericKind::Int64,
            &NumericKind::Float64,
        ));

        // Small integers are exact in floats
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int8(-10),
                &NumericKind::Int8,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(-10.0),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Int8(-10),
                &NumericKind::Int8,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(-10.0),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Uint8(255),
                &NumericKind::Uint8,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(255.0),
        );
    }

    #[test]
    fn test_convert_numeric_float_to_integer() {
        // Exact integer floats
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(42.0),
                &NumericKind::Float64,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(42),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(42.0),
                &NumericKind::Float32,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(42),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(-0.0),
                &NumericKind::Float64,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(0),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(-0.0),
                &NumericKind::Float32,
                &NumericKind::Int32,
            ),
            RuntimeValue::Int32(0),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(100.0),
                &NumericKind::Float64,
                &NumericKind::Uint32,
            ),
            RuntimeValue::Uint32(100),
        );

        // Non-integer floats fail
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(42.5),
            &NumericKind::Float64,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float32(42.5),
            &NumericKind::Float32,
            &NumericKind::Int32,
        ));

        // Non-finite floats fail
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(f64::NAN),
            &NumericKind::Float64,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(f64::INFINITY),
            &NumericKind::Float64,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(f64::NEG_INFINITY),
            &NumericKind::Float64,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float32(f32::NAN),
            &NumericKind::Float32,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float32(f32::INFINITY),
            &NumericKind::Float32,
            &NumericKind::Int32,
        ));
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float32(f32::NEG_INFINITY),
            &NumericKind::Float32,
            &NumericKind::Int32,
        ));

        // Negative float to unsigned fails
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(-1.0),
            &NumericKind::Float64,
            &NumericKind::Uint32,
        ));
    }

    #[test]
    fn test_convert_numeric_float_to_float() {
        // Float32 -> Float64
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(1.5),
                &NumericKind::Float32,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(1.5),
        );
        // Float64 -> Float32
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(1.5),
                &NumericKind::Float64,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(1.5),
        );
        // Float64 -> Float32 non-representable fails
        assert_fixed_conversion_err(convert_fixed_numeric(
            RuntimeValue::Float64(1e300),
            &NumericKind::Float64,
            &NumericKind::Float32,
        ));

        // Identity
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(2.5),
                &NumericKind::Float32,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(2.5),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(2.5),
                &NumericKind::Float64,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(2.5),
        );

        // Infinities
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(f64::INFINITY),
                &NumericKind::Float64,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(f32::INFINITY),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(f64::NEG_INFINITY),
                &NumericKind::Float64,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(f32::NEG_INFINITY),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(f32::INFINITY),
                &NumericKind::Float32,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(f64::INFINITY),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(f32::NEG_INFINITY),
                &NumericKind::Float32,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(f64::NEG_INFINITY),
        );

        // NaNs
        let res_nan_to_f32 = convert_fixed_numeric(
            RuntimeValue::Float64(f64::NAN),
            &NumericKind::Float64,
            &NumericKind::Float32,
        );
        match res_nan_to_f32 {
            Ok(RuntimeValue::Float32(f)) => assert!(f.is_nan()),
            _ => panic!("expected Float32(NaN)"),
        }
        let res_nan_to_f64 = convert_fixed_numeric(
            RuntimeValue::Float32(f32::NAN),
            &NumericKind::Float32,
            &NumericKind::Float64,
        );
        match res_nan_to_f64 {
            Ok(RuntimeValue::Float64(f)) => assert!(f.is_nan()),
            _ => panic!("expected Float64(NaN)"),
        }

        // Signed zero preservation
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float64(-0.0),
                &NumericKind::Float64,
                &NumericKind::Float32,
            ),
            RuntimeValue::Float32(-0.0),
        );
        assert_fixed_ok(
            convert_fixed_numeric(
                RuntimeValue::Float32(-0.0),
                &NumericKind::Float32,
                &NumericKind::Float64,
            ),
            RuntimeValue::Float64(-0.0),
        );
    }

    #[test]
    fn test_convert_numeric_vm_execution() {
        // Success execution
        let res_ok = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConvertNumeric {
                    source: NumericKind::Int32,
                    target: NumericKind::Int64,
                },
            ],
            vec![Constant::Int32(42)],
        );
        match res_ok {
            Ok(RuntimeValue::Int64(v)) => assert_eq!(v, 42),
            _ => panic!("expected Int64(42)"),
        }

        // Failure execution
        let res_err = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::ConvertNumeric {
                    source: NumericKind::Int32,
                    target: NumericKind::Int8,
                },
            ],
            vec![Constant::Int32(300)],
        );
        match res_err {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Conversion) => {}
                _ => panic!("expected EvaluationFailure::Conversion"),
            },
            Ok(_) => panic!("expected execution failure"),
        }
    }

    #[test]
    fn test_numeric_to_string_vm_execution() {
        let test_cases = vec![
            (Constant::Int8(-42), NumericKind::Int8, "-42"),
            (Constant::Int16(-1000), NumericKind::Int16, "-1000"),
            (Constant::Int32(-12345), NumericKind::Int32, "-12345"),
            (Constant::Int64(-999999), NumericKind::Int64, "-999999"),
            (
                Constant::Int128(-1234567890123456789),
                NumericKind::Int128,
                "-1234567890123456789",
            ),
            (Constant::Uint8(255), NumericKind::Uint8, "255"),
            (Constant::Uint16(65535), NumericKind::Uint16, "65535"),
            (Constant::Uint32(123456), NumericKind::Uint32, "123456"),
            (
                Constant::Uint64(123456789),
                NumericKind::Uint64,
                "123456789",
            ),
            (
                Constant::Uint128(1234567890123456789),
                NumericKind::Uint128,
                "1234567890123456789",
            ),
            (Constant::Float32(3.25), NumericKind::Float32, "3.25"),
            (Constant::Float32(f32::NAN), NumericKind::Float32, "NaN"),
            (
                Constant::Float32(f32::INFINITY),
                NumericKind::Float32,
                "inf",
            ),
            (
                Constant::Float32(f32::NEG_INFINITY),
                NumericKind::Float32,
                "-inf",
            ),
            (Constant::Float32(-0.0), NumericKind::Float32, "-0"),
            (Constant::Float64(f64::NAN), NumericKind::Float64, "NaN"),
            (
                Constant::Float64(f64::INFINITY),
                NumericKind::Float64,
                "inf",
            ),
            (
                Constant::Float64(f64::NEG_INFINITY),
                NumericKind::Float64,
                "-inf",
            ),
            (Constant::Float64(-0.0), NumericKind::Float64, "-0"),
        ];

        for (constant, kind, expected_str) in test_cases {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::NumericToString(kind),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::EqualString,
                ],
                vec![constant, Constant::String(expected_str.to_string())],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => {
                    assert!(b, "expected string match for {}", expected_str);
                }
                _ => panic!("expected EqualString to return Boolean"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "NumericToString: operand family mismatch with NumericKind")]
    fn test_numeric_to_string_operand_mismatch_panics() {
        let _ = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::NumericToString(NumericKind::Int32),
            ],
            vec![Constant::Boolean(true)],
        );
    }

    #[test]
    fn test_dynamic_lift_and_string() {
        let test_cases = vec![
            (Constant::Int32(42), NumericKind::Int32, "42"),
            (Constant::Int128(-1000), NumericKind::Int128, "-1000"),
            (Constant::Uint64(999), NumericKind::Uint64, "999"),
            (Constant::Float32(1.5), NumericKind::Float32, "1.5"),
            (Constant::Float64(-2.5), NumericKind::Float64, "-2.5"),
        ];

        for (constant, kind, expected_str) in test_cases {
            let res = test_execute_instructions(
                vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LiftDynamic(kind),
                    Instruction::DynamicToString,
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::EqualString,
                ],
                vec![constant, Constant::String(expected_str.to_string())],
            );
            match res {
                Ok(RuntimeValue::Boolean(b)) => {
                    assert!(b, "expected string match for {}", expected_str);
                }
                _ => panic!(
                    "expected EqualString to return Boolean for {}",
                    expected_str
                ),
            }
        }
    }

    #[test]
    fn test_dynamic_arithmetic_integers() {
        // Add: 10 + 20 = 30
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicAdd,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Int32(10),
                Constant::Int32(20),
                Constant::String("30".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Subtract: 50 - 15 = 35
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicSubtract,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Int32(50),
                Constant::Int32(15),
                Constant::String("35".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Multiply: 7 * 6 = 42
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicMultiply,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Int32(7),
                Constant::Int32(6),
                Constant::String("42".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Divide: 100 / 4 = 25
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicDivide,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Int32(100),
                Constant::Int32(4),
                Constant::String("25".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Negate: -(42) = -42
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicNegate,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::EqualString,
            ],
            vec![Constant::Int32(42), Constant::String("-42".to_string())],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Remainder: 17 % 5 = 2
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicRemainder,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Int32(17),
                Constant::Int32(5),
                Constant::String("2".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }
    }

    #[test]
    fn test_dynamic_arithmetic_floats() {
        // Float Add: 1.25 + 2.5 = 3.75
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::DynamicAdd,
                Instruction::DynamicToString,
                Instruction::LoadConstant(ConstantId(2)),
                Instruction::EqualString,
            ],
            vec![
                Constant::Float64(1.25),
                Constant::Float64(2.5),
                Constant::String("3.75".to_string()),
            ],
        );
        match res {
            Ok(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }

        // Float Divide by 0.0 succeeds (IEEE infinity)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::DynamicDivide,
            ],
            vec![Constant::Float64(1.0), Constant::Float64(0.0)],
        );
        match res {
            Ok(RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(f))) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected dynamic float +Infinity"),
        }
    }

    #[test]
    fn test_dynamic_failures() {
        // Integer division by zero
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicDivide,
            ],
            vec![Constant::Int32(10), Constant::Int32(0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                _ => panic!("expected DivisionByZero failure"),
            },
            Ok(_) => panic!("expected failure"),
        }

        // Integer remainder by zero
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::DynamicRemainder,
            ],
            vec![Constant::Int32(10), Constant::Int32(0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DivisionByZero) => {}
                _ => panic!("expected DivisionByZero failure"),
            },
            Ok(_) => panic!("expected failure"),
        }

        // Dynamic remainder on floats MUST fail with DynamicNumericType (language guard)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::DynamicRemainder,
            ],
            vec![Constant::Float64(10.0), Constant::Float64(3.0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DynamicNumericType) => {}
                _ => panic!("expected DynamicNumericType failure"),
            },
            Ok(_) => panic!("expected failure"),
        }

        // Dynamic remainder mixed integer and float -> DynamicNumericType
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::DynamicRemainder,
            ],
            vec![Constant::Int32(10), Constant::Float64(3.0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DynamicNumericType) => {}
                _ => panic!("expected DynamicNumericType failure"),
            },
            Ok(_) => panic!("expected failure"),
        }

        // DynamicAdd mixed families -> DynamicNumericType
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::LoadConstant(ConstantId(1)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::DynamicAdd,
            ],
            vec![Constant::Int32(10), Constant::Float64(3.0)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::DynamicNumericType) => {}
                _ => panic!("expected DynamicNumericType failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }

    #[test]
    fn test_convert_dynamic() {
        // Dynamic integer 42 -> ConvertDynamic(Int8) -> Int8(42)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::ConvertDynamic(NumericKind::Int8),
            ],
            vec![Constant::Int32(42)],
        );
        match res {
            Ok(RuntimeValue::Int8(v)) => assert_eq!(v, 42),
            _ => panic!("expected Int8(42)"),
        }

        // Dynamic integer 1000 -> ConvertDynamic(Int8) -> Err(Conversion)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Int32),
                Instruction::ConvertDynamic(NumericKind::Int8),
            ],
            vec![Constant::Int32(1000)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Conversion) => {}
                _ => panic!("expected Conversion failure"),
            },
            Ok(_) => panic!("expected failure"),
        }

        // Dynamic float 2.0 -> ConvertDynamic(Int32) -> Int32(2)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::ConvertDynamic(NumericKind::Int32),
            ],
            vec![Constant::Float64(2.0)],
        );
        match res {
            Ok(RuntimeValue::Int32(v)) => assert_eq!(v, 2),
            _ => panic!("expected Int32(2)"),
        }

        // Dynamic float 2.5 -> ConvertDynamic(Int32) -> Err(Conversion)
        let res = test_execute_instructions(
            vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::LiftDynamic(NumericKind::Float64),
                Instruction::ConvertDynamic(NumericKind::Int32),
            ],
            vec![Constant::Float64(2.5)],
        );
        match res {
            Err(e) => match e.kind {
                ExecutionFailureKind::Evaluation(EvaluationFailure::Conversion) => {}
                _ => panic!("expected Conversion failure"),
            },
            Ok(_) => panic!("expected failure"),
        }
    }
}
