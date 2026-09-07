use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use num_bigint::{BigInt, Sign};

use crate::data::compiled::equality::{
    CompositeEqualityPlan, EnumEqualityPayloadPlan, EqualityRule,
};
use crate::data::compiled::identities::{
    ConstantId, FieldIndex, InstructionIndex, NumericKind, VariantDiscriminant,
};
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
use crate::tools::own_runtime_value::OWN_RUNTIME_VALUE;
use evo_values::boolean::NOT;
use evo_values::comparison::{EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL, NOT_EQUAL};
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
use evo_values::{ComparisonFailure, NumericFailure, OwnedValue, Value};

pub type ExecuteInstruction =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<Option<OwnedValue>, ExecutionFailure>;

const I128_MIN_F64: f64 = -170_141_183_460_469_231_731_687_303_715_884_105_728.0;
const I128_MAX_LIMIT_F64: f64 = 170_141_183_460_469_231_731_687_303_715_884_105_728.0;
const U128_MAX_LIMIT_F64: f64 = 340_282_366_920_938_463_463_374_607_431_768_211_456.0;

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

fn expect_comparison_result(result: Result<bool, ComparisonFailure>) -> bool {
    match result {
        Ok(v) => v,
        Err(ComparisonFailure::DifferentFamily) => {
            panic!(
                "internal invariant violation: ComparisonFailure::DifferentFamily in scalar comparison"
            )
        }
        Err(ComparisonFailure::NotComparable) => {
            panic!(
                "internal invariant violation: ComparisonFailure::NotComparable in scalar comparison"
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

fn resolve_dynamic_integer(
    dyn_int_ref: DynamicIntegerBackingRef,
    compiled: &CompiledProgram,
    backing: &ExecutionBackingStore,
) -> BigInt {
    match dyn_int_ref {
        DynamicIntegerBackingRef::Compiled(constant_id) => {
            match &compiled.constants[constant_id.0] {
                Constant::Dynamic(DynamicConstant::Integer {
                    negative,
                    magnitude,
                }) => {
                    let sign = if *negative { Sign::Minus } else { Sign::Plus };
                    BigInt::from_bytes_be(sign, magnitude)
                }
                _ => panic!(
                    "Constant referenced by DynamicIntegerBackingRef::Compiled must be DynamicConstant::Integer"
                ),
            }
        }
        DynamicIntegerBackingRef::Execution(id) => backing.dynamic_integers[id.0].value.clone(),
    }
}

fn convert_i128_to_target(val: i128, target: &NumericKind) -> Result<RuntimeValue, ()> {
    match target {
        NumericKind::Int8 => i8::try_from(val).map(RuntimeValue::Int8).map_err(|_| ()),
        NumericKind::Int16 => i16::try_from(val).map(RuntimeValue::Int16).map_err(|_| ()),
        NumericKind::Int32 => i32::try_from(val).map(RuntimeValue::Int32).map_err(|_| ()),
        NumericKind::Int64 => i64::try_from(val).map(RuntimeValue::Int64).map_err(|_| ()),
        NumericKind::Int128 => Ok(RuntimeValue::Int128(val)),

        NumericKind::Uint8 => u8::try_from(val).map(RuntimeValue::Uint8).map_err(|_| ()),
        NumericKind::Uint16 => u16::try_from(val).map(RuntimeValue::Uint16).map_err(|_| ()),
        NumericKind::Uint32 => u32::try_from(val).map(RuntimeValue::Uint32).map_err(|_| ()),
        NumericKind::Uint64 => u64::try_from(val).map(RuntimeValue::Uint64).map_err(|_| ()),
        NumericKind::Uint128 => u128::try_from(val)
            .map(RuntimeValue::Uint128)
            .map_err(|_| ()),

        NumericKind::Float32 => {
            let f = val as f32;
            if f.is_finite()
                && (f as f64) >= I128_MIN_F64
                && (f as f64) < I128_MAX_LIMIT_F64
                && (f as i128) == val
            {
                Ok(RuntimeValue::Float32(f))
            } else {
                Err(())
            }
        }
        NumericKind::Float64 => {
            let f = val as f64;
            if f.is_finite() && f >= I128_MIN_F64 && f < I128_MAX_LIMIT_F64 && (f as i128) == val {
                Ok(RuntimeValue::Float64(f))
            } else {
                Err(())
            }
        }
    }
}

fn convert_u128_to_target(val: u128, target: &NumericKind) -> Result<RuntimeValue, ()> {
    match target {
        NumericKind::Int8 => i8::try_from(val).map(RuntimeValue::Int8).map_err(|_| ()),
        NumericKind::Int16 => i16::try_from(val).map(RuntimeValue::Int16).map_err(|_| ()),
        NumericKind::Int32 => i32::try_from(val).map(RuntimeValue::Int32).map_err(|_| ()),
        NumericKind::Int64 => i64::try_from(val).map(RuntimeValue::Int64).map_err(|_| ()),
        NumericKind::Int128 => i128::try_from(val)
            .map(RuntimeValue::Int128)
            .map_err(|_| ()),

        NumericKind::Uint8 => u8::try_from(val).map(RuntimeValue::Uint8).map_err(|_| ()),
        NumericKind::Uint16 => u16::try_from(val).map(RuntimeValue::Uint16).map_err(|_| ()),
        NumericKind::Uint32 => u32::try_from(val).map(RuntimeValue::Uint32).map_err(|_| ()),
        NumericKind::Uint64 => u64::try_from(val).map(RuntimeValue::Uint64).map_err(|_| ()),
        NumericKind::Uint128 => Ok(RuntimeValue::Uint128(val)),

        NumericKind::Float32 => {
            let f = val as f32;
            if f.is_finite() && f >= 0.0 && (f as f64) < U128_MAX_LIMIT_F64 && (f as u128) == val {
                Ok(RuntimeValue::Float32(f))
            } else {
                Err(())
            }
        }
        NumericKind::Float64 => {
            let f = val as f64;
            if f.is_finite() && f >= 0.0 && f < U128_MAX_LIMIT_F64 && (f as u128) == val {
                Ok(RuntimeValue::Float64(f))
            } else {
                Err(())
            }
        }
    }
}

fn convert_f64_to_target(f: f64, target: &NumericKind) -> Result<RuntimeValue, ()> {
    if !f.is_finite() {
        return Err(());
    }
    match target {
        NumericKind::Float32 => {
            let f32_val = f as f32;
            if (f32_val as f64) == f {
                Ok(RuntimeValue::Float32(f32_val))
            } else {
                Err(())
            }
        }
        NumericKind::Float64 => Ok(RuntimeValue::Float64(f)),

        NumericKind::Int8
        | NumericKind::Int16
        | NumericKind::Int32
        | NumericKind::Int64
        | NumericKind::Int128 => {
            if f.fract() != 0.0 || f < I128_MIN_F64 || f >= I128_MAX_LIMIT_F64 {
                return Err(());
            }
            let int_val = f as i128;
            convert_i128_to_target(int_val, target)
        }

        NumericKind::Uint8
        | NumericKind::Uint16
        | NumericKind::Uint32
        | NumericKind::Uint64
        | NumericKind::Uint128 => {
            if f.fract() != 0.0 || f < 0.0 || f >= U128_MAX_LIMIT_F64 {
                return Err(());
            }
            let uint_val = f as u128;
            convert_u128_to_target(uint_val, target)
        }
    }
}

fn convert_bigint_to_f32(bigint: &BigInt) -> Result<RuntimeValue, ()> {
    if *bigint == BigInt::from(0) {
        return Ok(RuntimeValue::Float32(0.0));
    }
    let is_negative = bigint.sign() == Sign::Minus;
    let magnitude = if is_negative {
        -bigint.clone()
    } else {
        bigint.clone()
    };
    let total_bits = magnitude.bits();

    // Max finite f32 exponent is 127: total_bits - 1 <= 127 => total_bits <= 128
    if total_bits > 128 {
        return Err(());
    }

    let p = 24u64;
    let (mantissa_u32, shift) = if total_bits <= p {
        let val = u32::try_from(&magnitude).map_err(|_| ())?;
        (val, 0i32)
    } else {
        let shift = (total_bits - p) as usize;
        let mask = (BigInt::from(1) << shift) - BigInt::from(1);
        if (&magnitude & &mask) != BigInt::from(0) {
            return Err(());
        }
        let shifted = &magnitude >> shift;
        let val = u32::try_from(&shifted).map_err(|_| ())?;
        (val, shift as i32)
    };

    let f = (mantissa_u32 as f32) * 2.0f32.powi(shift);
    if !f.is_finite() {
        return Err(());
    }
    let res = if is_negative { -f } else { f };
    Ok(RuntimeValue::Float32(res))
}

fn convert_bigint_to_f64(bigint: &BigInt) -> Result<RuntimeValue, ()> {
    if *bigint == BigInt::from(0) {
        return Ok(RuntimeValue::Float64(0.0));
    }
    let is_negative = bigint.sign() == Sign::Minus;
    let magnitude = if is_negative {
        -bigint.clone()
    } else {
        bigint.clone()
    };
    let total_bits = magnitude.bits();

    // Max finite f64 exponent is 1023: total_bits - 1 <= 1023 => total_bits <= 1024
    if total_bits > 1024 {
        return Err(());
    }

    let p = 53u64;
    let (mantissa_u64, shift) = if total_bits <= p {
        let val = u64::try_from(&magnitude).map_err(|_| ())?;
        (val, 0i32)
    } else {
        let shift = (total_bits - p) as usize;
        let mask = (BigInt::from(1) << shift) - BigInt::from(1);
        if (&magnitude & &mask) != BigInt::from(0) {
            return Err(());
        }
        let shifted = &magnitude >> shift;
        let val = u64::try_from(&shifted).map_err(|_| ())?;
        (val, shift as i32)
    };

    let f = (mantissa_u64 as f64) * 2.0f64.powi(shift);
    if !f.is_finite() {
        return Err(());
    }
    let res = if is_negative { -f } else { f };
    Ok(RuntimeValue::Float64(res))
}

fn is_same_numeric_kind(a: &NumericKind, b: &NumericKind) -> bool {
    matches!(
        (a, b),
        (NumericKind::Int8, NumericKind::Int8)
            | (NumericKind::Int16, NumericKind::Int16)
            | (NumericKind::Int32, NumericKind::Int32)
            | (NumericKind::Int64, NumericKind::Int64)
            | (NumericKind::Int128, NumericKind::Int128)
            | (NumericKind::Uint8, NumericKind::Uint8)
            | (NumericKind::Uint16, NumericKind::Uint16)
            | (NumericKind::Uint32, NumericKind::Uint32)
            | (NumericKind::Uint64, NumericKind::Uint64)
            | (NumericKind::Uint128, NumericKind::Uint128)
            | (NumericKind::Float32, NumericKind::Float32)
            | (NumericKind::Float64, NumericKind::Float64)
    )
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

fn convert_fixed_numeric(
    val: RuntimeValue,
    source: &NumericKind,
    target: &NumericKind,
) -> Result<RuntimeValue, ()> {
    assert_runtime_value_matches_kind(&val, source);
    if is_same_numeric_kind(source, target) {
        return Ok(val);
    }
    match (source, val) {
        (NumericKind::Int8, RuntimeValue::Int8(v)) => convert_i128_to_target(v as i128, target),
        (NumericKind::Int16, RuntimeValue::Int16(v)) => convert_i128_to_target(v as i128, target),
        (NumericKind::Int32, RuntimeValue::Int32(v)) => convert_i128_to_target(v as i128, target),
        (NumericKind::Int64, RuntimeValue::Int64(v)) => convert_i128_to_target(v as i128, target),
        (NumericKind::Int128, RuntimeValue::Int128(v)) => convert_i128_to_target(v, target),

        (NumericKind::Uint8, RuntimeValue::Uint8(v)) => convert_u128_to_target(v as u128, target),
        (NumericKind::Uint16, RuntimeValue::Uint16(v)) => convert_u128_to_target(v as u128, target),
        (NumericKind::Uint32, RuntimeValue::Uint32(v)) => convert_u128_to_target(v as u128, target),
        (NumericKind::Uint64, RuntimeValue::Uint64(v)) => convert_u128_to_target(v as u128, target),
        (NumericKind::Uint128, RuntimeValue::Uint128(v)) => convert_u128_to_target(v, target),

        (NumericKind::Float32, RuntimeValue::Float32(v)) => convert_f64_to_target(v as f64, target),
        (NumericKind::Float64, RuntimeValue::Float64(v)) => convert_f64_to_target(v, target),

        _ => panic!("convert_fixed_numeric: runtime value family mismatch with source NumericKind"),
    }
}

fn convert_dynamic_numeric(
    dyn_val: RuntimeDynamicValue,
    target: &NumericKind,
    compiled: &CompiledProgram,
    backing: &ExecutionBackingStore,
) -> Result<RuntimeValue, ()> {
    match dyn_val {
        RuntimeDynamicValue::Float32(v) => convert_f64_to_target(v as f64, target),
        RuntimeDynamicValue::Float64(v) => convert_f64_to_target(v as f64, target),
        RuntimeDynamicValue::Integer(ref_id) => {
            let bigint = resolve_dynamic_integer(ref_id, compiled, backing);
            match target {
                NumericKind::Int8
                | NumericKind::Int16
                | NumericKind::Int32
                | NumericKind::Int64
                | NumericKind::Int128 => {
                    let val = i128::try_from(&bigint).map_err(|_| ())?;
                    convert_i128_to_target(val, target)
                }
                NumericKind::Uint8
                | NumericKind::Uint16
                | NumericKind::Uint32
                | NumericKind::Uint64
                | NumericKind::Uint128 => {
                    let val = u128::try_from(&bigint).map_err(|_| ())?;
                    convert_u128_to_target(val, target)
                }
                NumericKind::Float32 => convert_bigint_to_f32(&bigint),
                NumericKind::Float64 => convert_bigint_to_f64(&bigint),
            }
        }
    }
}

fn compare_numeric_equality(left: RuntimeValue, right: RuntimeValue, kind: &NumericKind) -> bool {
    match (kind, left, right) {
        (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => l == r,
        (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => l == r,
        (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => l == r,
        (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => l == r,
        (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => l == r,

        (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => l == r,
        (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => l == r,
        (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => l == r,
        (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => l == r,
        (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => l == r,

        (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => l == r,
        (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => l == r,

        _ => panic!("compare_numeric_equality: operand family mismatch with NumericKind"),
    }
}

fn evaluate_equality_rule(
    left: RuntimeValue,
    right: RuntimeValue,
    rule: &EqualityRule,
    compiled: &CompiledProgram,
    backing: &ExecutionBackingStore,
) -> bool {
    match rule {
        EqualityRule::Numeric(kind) => compare_numeric_equality(left, right, kind),
        EqualityRule::Boolean => match (left, right) {
            (RuntimeValue::Boolean(l), RuntimeValue::Boolean(r)) => l == r,
            _ => panic!("EqualityRule::Boolean expected Boolean values"),
        },
        EqualityRule::String => match (left, right) {
            (RuntimeValue::String(l_ref), RuntimeValue::String(r_ref)) => {
                let l_str = resolve_string(l_ref, compiled, backing);
                let r_str = resolve_string(r_ref, compiled, backing);
                l_str == r_str
            }
            _ => panic!("EqualityRule::String expected String values"),
        },
        EqualityRule::Composite(comp_plan) => {
            evaluate_composite_equality(left, right, comp_plan, compiled, backing)
        }
    }
}

fn evaluate_composite_equality(
    left: RuntimeValue,
    right: RuntimeValue,
    plan: &CompositeEqualityPlan,
    compiled: &CompiledProgram,
    backing: &ExecutionBackingStore,
) -> bool {
    match plan {
        CompositeEqualityPlan::Struct { fields } => {
            let left_id = match left {
                RuntimeValue::Struct(id) => id,
                _ => panic!("Expected Struct runtime value"),
            };
            let right_id = match right {
                RuntimeValue::Struct(id) => id,
                _ => panic!("Expected Struct runtime value"),
            };
            let left_struct = &backing.structs[left_id.0];
            let right_struct = &backing.structs[right_id.0];
            assert_eq!(left_struct.fields.len(), fields.len());
            assert_eq!(right_struct.fields.len(), fields.len());
            for (idx, rule) in fields.iter().enumerate() {
                if !evaluate_equality_rule(
                    left_struct.fields[idx],
                    right_struct.fields[idx],
                    rule,
                    compiled,
                    backing,
                ) {
                    return false;
                }
            }
            true
        }
        CompositeEqualityPlan::Enum { variants } => {
            let left_id = match left {
                RuntimeValue::Enum(id) => id,
                _ => panic!("Expected Enum runtime value"),
            };
            let right_id = match right {
                RuntimeValue::Enum(id) => id,
                _ => panic!("Expected Enum runtime value"),
            };
            let left_enum = &backing.enums[left_id.0];
            let right_enum = &backing.enums[right_id.0];

            assert!(
                left_enum.variant.0 < variants.len(),
                "left enum variant discriminant {} out of bounds for variants len {}",
                left_enum.variant.0,
                variants.len()
            );
            assert!(
                right_enum.variant.0 < variants.len(),
                "right enum variant discriminant {} out of bounds for variants len {}",
                right_enum.variant.0,
                variants.len()
            );

            if left_enum.variant.0 != right_enum.variant.0 {
                return false;
            }

            let variant_plan = &variants[left_enum.variant.0];
            match (variant_plan, &left_enum.payload, &right_enum.payload) {
                (
                    EnumEqualityPayloadPlan::Simple,
                    RuntimeEnumPayload::Simple,
                    RuntimeEnumPayload::Simple,
                ) => true,
                (
                    EnumEqualityPayloadPlan::Associated(rule),
                    RuntimeEnumPayload::Associated(l_val),
                    RuntimeEnumPayload::Associated(r_val),
                ) => evaluate_equality_rule(*l_val, *r_val, rule, compiled, backing),
                (
                    EnumEqualityPayloadPlan::Structured { fields },
                    RuntimeEnumPayload::Structured { fields: l_fields },
                    RuntimeEnumPayload::Structured { fields: r_fields },
                ) => {
                    assert_eq!(
                        l_fields.len(),
                        fields.len(),
                        "Structured payload cardinality mismatch with plan"
                    );
                    assert_eq!(
                        r_fields.len(),
                        fields.len(),
                        "Structured payload cardinality mismatch with plan"
                    );
                    for (idx, rule) in fields.iter().enumerate() {
                        if !evaluate_equality_rule(
                            l_fields[idx],
                            r_fields[idx],
                            rule,
                            compiled,
                            backing,
                        ) {
                            return false;
                        }
                    }
                    true
                }
                _ => panic!("Mismatch between EnumEqualityPayloadPlan and runtime payload"),
            }
        }
    }
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
            let runtime_dynamic = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Int16, RuntimeValue::Int16(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Int32, RuntimeValue::Int32(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Int64, RuntimeValue::Int64(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Int128, RuntimeValue::Int128(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(v)) => {
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking {
                            value: BigInt::from(v),
                        });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }

                (NumericKind::Float32, RuntimeValue::Float32(v)) => {
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(v))
                }
                (NumericKind::Float64, RuntimeValue::Float64(v)) => {
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(v))
                }

                _ => panic!("LiftDynamic: operand family mismatch with NumericKind"),
            };

            push_operand(execution, runtime_dynamic);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::DynamicNegate => {
            let operand = pop_operand(execution);
            let dyn_val = match operand {
                RuntimeValue::Dynamic(d) => d,
                _ => panic!("DynamicNegate expected Dynamic runtime value"),
            };

            let res = match dyn_val {
                RuntimeDynamicValue::Integer(ref_id) => {
                    let bigint = resolve_dynamic_integer(
                        ref_id,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let negated = -bigint;
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking { value: negated });
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(id),
                    ))
                }
                RuntimeDynamicValue::Float32(v) => {
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(-v))
                }
                RuntimeDynamicValue::Float64(v) => {
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(-v))
                }
            };

            push_operand(execution, res);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::DynamicAdd => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_dyn, r_dyn) = match (left, right) {
                (RuntimeValue::Dynamic(l), RuntimeValue::Dynamic(r)) => (l, r),
                _ => panic!("DynamicAdd expected Dynamic runtime values"),
            };

            match (l_dyn, r_dyn) {
                (RuntimeDynamicValue::Integer(l_ref), RuntimeDynamicValue::Integer(r_ref)) => {
                    let l_bi = resolve_dynamic_integer(
                        l_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let r_bi = resolve_dynamic_integer(
                        r_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let res_bi = l_bi + r_bi;
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking { value: res_bi });
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                            DynamicIntegerBackingRef::Execution(id),
                        )),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float32(l), RuntimeDynamicValue::Float32(r)) => {
                    let sum = l + r;
                    if sum.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(sum)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    let sum = l + r;
                    if sum.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(sum)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                _ => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::DynamicNumericType,
                )),
            }
        }

        Instruction::DynamicSubtract => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_dyn, r_dyn) = match (left, right) {
                (RuntimeValue::Dynamic(l), RuntimeValue::Dynamic(r)) => (l, r),
                _ => panic!("DynamicSubtract expected Dynamic runtime values"),
            };

            match (l_dyn, r_dyn) {
                (RuntimeDynamicValue::Integer(l_ref), RuntimeDynamicValue::Integer(r_ref)) => {
                    let l_bi = resolve_dynamic_integer(
                        l_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let r_bi = resolve_dynamic_integer(
                        r_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let res_bi = l_bi - r_bi;
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking { value: res_bi });
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                            DynamicIntegerBackingRef::Execution(id),
                        )),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float32(l), RuntimeDynamicValue::Float32(r)) => {
                    let diff = l - r;
                    if diff.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(diff)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    let diff = l - r;
                    if diff.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(diff)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                _ => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::DynamicNumericType,
                )),
            }
        }

        Instruction::DynamicMultiply => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_dyn, r_dyn) = match (left, right) {
                (RuntimeValue::Dynamic(l), RuntimeValue::Dynamic(r)) => (l, r),
                _ => panic!("DynamicMultiply expected Dynamic runtime values"),
            };

            match (l_dyn, r_dyn) {
                (RuntimeDynamicValue::Integer(l_ref), RuntimeDynamicValue::Integer(r_ref)) => {
                    let l_bi = resolve_dynamic_integer(
                        l_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let r_bi = resolve_dynamic_integer(
                        r_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let res_bi = l_bi * r_bi;
                    let id =
                        DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                    execution
                        .backing_store
                        .dynamic_integers
                        .push(DynamicIntegerBacking { value: res_bi });
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                            DynamicIntegerBackingRef::Execution(id),
                        )),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float32(l), RuntimeDynamicValue::Float32(r)) => {
                    let prod = l * r;
                    if prod.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(prod)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    let prod = l * r;
                    if prod.is_finite() {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(prod)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    } else {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::Overflow,
                        ))
                    }
                }
                _ => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::DynamicNumericType,
                )),
            }
        }

        Instruction::DynamicDivide => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_dyn, r_dyn) = match (left, right) {
                (RuntimeValue::Dynamic(l), RuntimeValue::Dynamic(r)) => (l, r),
                _ => panic!("DynamicDivide expected Dynamic runtime values"),
            };

            match (l_dyn, r_dyn) {
                (RuntimeDynamicValue::Integer(l_ref), RuntimeDynamicValue::Integer(r_ref)) => {
                    let l_bi = resolve_dynamic_integer(
                        l_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let r_bi = resolve_dynamic_integer(
                        r_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    if r_bi == BigInt::from(0) {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::DivisionByZero,
                        ))
                    } else {
                        let res_bi = l_bi / r_bi;
                        let id =
                            DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                        execution
                            .backing_store
                            .dynamic_integers
                            .push(DynamicIntegerBacking { value: res_bi });
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                                DynamicIntegerBackingRef::Execution(id),
                            )),
                        );
                        advance_ip(execution);
                        Ok(None)
                    }
                }
                (RuntimeDynamicValue::Float32(l), RuntimeDynamicValue::Float32(r)) => {
                    if r == 0.0 || r == -0.0 {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::DivisionByZero,
                        ))
                    } else {
                        let quotient = l / r;
                        if quotient.is_finite() {
                            push_operand(
                                execution,
                                RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(quotient)),
                            );
                            advance_ip(execution);
                            Ok(None)
                        } else {
                            Err(make_evaluation_failure(
                                execution,
                                EvaluationFailure::Overflow,
                            ))
                        }
                    }
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    if r == 0.0 || r == -0.0 {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::DivisionByZero,
                        ))
                    } else {
                        let quotient = l / r;
                        if quotient.is_finite() {
                            push_operand(
                                execution,
                                RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(quotient)),
                            );
                            advance_ip(execution);
                            Ok(None)
                        } else {
                            Err(make_evaluation_failure(
                                execution,
                                EvaluationFailure::Overflow,
                            ))
                        }
                    }
                }
                _ => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::DynamicNumericType,
                )),
            }
        }

        Instruction::DynamicRemainder => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let (l_dyn, r_dyn) = match (left, right) {
                (RuntimeValue::Dynamic(l), RuntimeValue::Dynamic(r)) => (l, r),
                _ => panic!("DynamicRemainder expected Dynamic runtime values"),
            };

            match (l_dyn, r_dyn) {
                (RuntimeDynamicValue::Integer(l_ref), RuntimeDynamicValue::Integer(r_ref)) => {
                    let l_bi = resolve_dynamic_integer(
                        l_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    let r_bi = resolve_dynamic_integer(
                        r_ref,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    if r_bi == BigInt::from(0) {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::DivisionByZero,
                        ))
                    } else {
                        let res_bi = l_bi % r_bi;
                        let id =
                            DynamicIntegerBackingId(execution.backing_store.dynamic_integers.len());
                        execution
                            .backing_store
                            .dynamic_integers
                            .push(DynamicIntegerBacking { value: res_bi });
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                                DynamicIntegerBackingRef::Execution(id),
                            )),
                        );
                        advance_ip(execution);
                        Ok(None)
                    }
                }
                (RuntimeDynamicValue::Float32(_), RuntimeDynamicValue::Float32(_))
                | (RuntimeDynamicValue::Float64(_), RuntimeDynamicValue::Float64(_)) => Err(
                    make_evaluation_failure(execution, EvaluationFailure::DynamicNumericType),
                ),
                _ => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::DynamicNumericType,
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
                Err(()) => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Conversion,
                )),
            }
        }

        Instruction::ConvertDynamic(target) => {
            let operand = pop_operand(execution);
            let dyn_val = match operand {
                RuntimeValue::Dynamic(d) => d,
                _ => panic!("ConvertDynamic expected Dynamic operand"),
            };

            match convert_dynamic_numeric(
                dyn_val,
                target,
                execution.compiled_program,
                &execution.backing_store,
            ) {
                Ok(res) => {
                    push_operand(execution, res);
                    advance_ip(execution);
                    Ok(None)
                }
                Err(()) => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Conversion,
                )),
            }
        }

        Instruction::NumericToString(kind) => {
            let operand = pop_operand(execution);
            let str_val = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => v.to_string(),
                (NumericKind::Int16, RuntimeValue::Int16(v)) => v.to_string(),
                (NumericKind::Int32, RuntimeValue::Int32(v)) => v.to_string(),
                (NumericKind::Int64, RuntimeValue::Int64(v)) => v.to_string(),
                (NumericKind::Int128, RuntimeValue::Int128(v)) => v.to_string(),

                (NumericKind::Uint8, RuntimeValue::Uint8(v)) => v.to_string(),
                (NumericKind::Uint16, RuntimeValue::Uint16(v)) => v.to_string(),
                (NumericKind::Uint32, RuntimeValue::Uint32(v)) => v.to_string(),
                (NumericKind::Uint64, RuntimeValue::Uint64(v)) => v.to_string(),
                (NumericKind::Uint128, RuntimeValue::Uint128(v)) => v.to_string(),

                (NumericKind::Float32, RuntimeValue::Float32(v)) => v.to_string(),
                (NumericKind::Float64, RuntimeValue::Float64(v)) => v.to_string(),

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
            let dyn_val = match operand {
                RuntimeValue::Dynamic(d) => d,
                _ => panic!("DynamicToString expected Dynamic operand"),
            };

            let str_val = match dyn_val {
                RuntimeDynamicValue::Integer(ref_id) => {
                    let bigint = resolve_dynamic_integer(
                        ref_id,
                        execution.compiled_program,
                        &execution.backing_store,
                    );
                    bigint.to_string()
                }
                RuntimeDynamicValue::Float32(v) => v.to_string(),
                RuntimeDynamicValue::Float64(v) => v.to_string(),
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
        Instruction::EqualComposite(plan) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let is_equal = evaluate_composite_equality(
                left,
                right,
                plan,
                execution.compiled_program,
                &execution.backing_store,
            );

            push_operand(execution, RuntimeValue::Boolean(is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualComposite(plan) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let is_equal = evaluate_composite_equality(
                left,
                right,
                plan,
                execution.compiled_program,
                &execution.backing_store,
            );

            push_operand(execution, RuntimeValue::Boolean(!is_equal));
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
        CompiledValueShapeId, ConstantId, ExternalSymbolId, InstructionIndex, LocalSlot,
        ParameterSlot,
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
        // 2^200 -> Float64 succeeds
        let big_2_200 = BigInt::from(1) << 200;
        let res_f64 = convert_bigint_to_f64(&big_2_200);
        assert!(res_f64.is_ok());
        match res_f64.unwrap() {
            RuntimeValue::Float64(f) => {
                assert!(f.is_finite());
                assert_eq!(f, 2.0f64.powi(200));
            }
            _ => panic!("expected Float64"),
        }

        // -2^200 -> Float64 succeeds
        let neg_big_2_200 = -big_2_200.clone();
        let res_neg_f64 = convert_bigint_to_f64(&neg_big_2_200);
        assert!(res_neg_f64.is_ok());
        match res_neg_f64.unwrap() {
            RuntimeValue::Float64(f) => {
                assert!(f.is_finite());
                assert_eq!(f, -2.0f64.powi(200));
            }
            _ => panic!("expected Float64"),
        }

        // (2^200) + 1 -> Float64 conversion error
        let big_2_200_plus_1 = big_2_200 + BigInt::from(1);
        let res_plus_1 = convert_bigint_to_f64(&big_2_200_plus_1);
        assert!(res_plus_1.is_err());

        // 2^127 -> Float32 succeeds
        let big_2_127 = BigInt::from(1) << 127;
        let res_f32 = convert_bigint_to_f32(&big_2_127);
        assert!(res_f32.is_ok());
        match res_f32.unwrap() {
            RuntimeValue::Float32(f) => {
                assert!(f.is_finite());
                assert_eq!(f, 2.0f32.powi(127));
            }
            _ => panic!("expected Float32"),
        }

        // 2^128 -> Float32 conversion error (exceeds finite f32)
        let big_2_128 = BigInt::from(1) << 128;
        let res_f32_overflow = convert_bigint_to_f32(&big_2_128);
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
    fn regression_dynamic_float_overflow() {
        let program = CompiledProgram {
            functions: vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 2,
                instructions: vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::LoadConstant(ConstantId(1)),
                    Instruction::DynamicAdd,
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
            Ok(_) => panic!("dynamic float overflow should fail"),
            Err(e) => e,
        };
        match err.kind {
            ExecutionFailureKind::Evaluation(EvaluationFailure::Overflow) => {}
            _ => panic!("expected Overflow failure"),
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
    #[should_panic(expected = "left enum variant discriminant 5 out of bounds for variants len 1")]
    fn regression_invalid_enum_equality_discriminant_panics() {
        let backing = ExecutionBackingStore {
            strings: Vec::new(),
            dynamic_integers: Vec::new(),
            structs: Vec::new(),
            enums: vec![
                EnumBacking {
                    variant: VariantDiscriminant(5),
                    payload: RuntimeEnumPayload::Simple,
                },
                EnumBacking {
                    variant: VariantDiscriminant(0),
                    payload: RuntimeEnumPayload::Simple,
                },
            ],
        };
        let program = CompiledProgram {
            functions: Vec::new(),
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: Vec::new(),
            },
        };
        let plan = CompositeEqualityPlan::Enum {
            variants: vec![EnumEqualityPayloadPlan::Simple],
        };

        let _ = evaluate_composite_equality(
            RuntimeValue::Enum(EnumBackingId(0)),
            RuntimeValue::Enum(EnumBackingId(1)),
            &plan,
            &program,
            &backing,
        );
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
}
