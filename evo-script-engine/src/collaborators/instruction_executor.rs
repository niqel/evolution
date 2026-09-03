use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use num_bigint::{BigInt, Sign};

use crate::data::compiled::boundary::CompiledValueShape;
use crate::data::compiled::equality::{
    CompositeEqualityPlan, EnumEqualityPayloadPlan, EqualityRule,
};
use crate::data::compiled::identities::{
    ConstantId, FieldIndex, InstructionIndex, LocalSlot, NumericKind, ParameterSlot,
    VariantDiscriminant,
};
use crate::data::compiled::instructions::Instruction;
use crate::data::compiled::program::CompiledProgram;
use crate::data::compiled::storage::{Constant, DynamicConstant};
use crate::data::failures::{
    EvaluationFailure, ExecutionFailure, ExecutionFailureKind, InvocationFailure,
};
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
use evo_values::OwnedValue;

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
    execution.value_storage.cells.push(Some(val));
}

fn advance_ip(execution: &mut VmExecution) {
    execution
        .call_frames
        .last_mut()
        .expect("active CallFrame must exist")
        .instruction_pointer
        .0 += 1;
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

fn convert_fixed_numeric(
    val: RuntimeValue,
    source: &NumericKind,
    target: &NumericKind,
) -> Result<RuntimeValue, ()> {
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
                NumericKind::Float32 => {
                    if let Ok(val) = i128::try_from(&bigint) {
                        convert_i128_to_target(val, &NumericKind::Float32)
                    } else {
                        Err(())
                    }
                }
                NumericKind::Float64 => {
                    if let Ok(val) = i128::try_from(&bigint) {
                        convert_i128_to_target(val, &NumericKind::Float64)
                    } else {
                        Err(())
                    }
                }
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
            if left_enum.variant.0 != right_enum.variant.0 {
                return false;
            }
            let variant_plan = variants
                .get(left_enum.variant.0)
                .expect("Variant plan must exist for variant discriminant");
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
                    assert_eq!(l_fields.len(), fields.len());
                    assert_eq!(r_fields.len(), fields.len());
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
            let abs_cell = frame_base + slot.0;
            let val = execution.value_storage.cells[abs_cell]
                .expect("Parameter cell must contain Some(RuntimeValue)");
            push_operand(execution, val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LoadLocal(slot) => {
            let abs_cell = frame_base + function.parameter_count + slot.0;
            let val = execution.value_storage.cells[abs_cell]
                .expect("Local cell must contain Some(RuntimeValue)");
            push_operand(execution, val);
            advance_ip(execution);
            Ok(None)
        }

        Instruction::StoreLocal(slot) => {
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
            let target_func = execution
                .compiled_program
                .functions
                .get(target_id.0)
                .expect("Call target FunctionId must exist in CompiledProgram");

            let param_count = target_func.parameter_count;
            let local_count = target_func.local_count;
            let callee_frame_base = execution.value_storage.cells.len() - param_count;

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
            let res = match (kind, operand) {
                (NumericKind::Int8, RuntimeValue::Int8(v)) => {
                    v.checked_neg().map(RuntimeValue::Int8)
                }
                (NumericKind::Int16, RuntimeValue::Int16(v)) => {
                    v.checked_neg().map(RuntimeValue::Int16)
                }
                (NumericKind::Int32, RuntimeValue::Int32(v)) => {
                    v.checked_neg().map(RuntimeValue::Int32)
                }
                (NumericKind::Int64, RuntimeValue::Int64(v)) => {
                    v.checked_neg().map(RuntimeValue::Int64)
                }
                (NumericKind::Int128, RuntimeValue::Int128(v)) => {
                    v.checked_neg().map(RuntimeValue::Int128)
                }
                (NumericKind::Float32, RuntimeValue::Float32(v)) => Some(RuntimeValue::Float32(-v)),
                (NumericKind::Float64, RuntimeValue::Float64(v)) => Some(RuntimeValue::Float64(-v)),
                _ => panic!("Negate: operand family mismatch or unsupported unsigned negation"),
            };

            match res {
                Some(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                None => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Overflow,
                )),
            }
        }

        Instruction::Add(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    l.checked_add(r).map(RuntimeValue::Int8)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    l.checked_add(r).map(RuntimeValue::Int16)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    l.checked_add(r).map(RuntimeValue::Int32)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    l.checked_add(r).map(RuntimeValue::Int64)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    l.checked_add(r).map(RuntimeValue::Int128)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    l.checked_add(r).map(RuntimeValue::Uint8)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    l.checked_add(r).map(RuntimeValue::Uint16)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    l.checked_add(r).map(RuntimeValue::Uint32)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    l.checked_add(r).map(RuntimeValue::Uint64)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    l.checked_add(r).map(RuntimeValue::Uint128)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Some(RuntimeValue::Float32(l + r))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Some(RuntimeValue::Float64(l + r))
                }

                _ => panic!("Add: operand family mismatch with NumericKind"),
            };

            match res {
                Some(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                None => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Overflow,
                )),
            }
        }

        Instruction::Subtract(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Int8)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Int16)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Int32)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Int64)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Int128)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Uint8)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Uint16)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Uint32)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Uint64)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    l.checked_sub(r).map(RuntimeValue::Uint128)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Some(RuntimeValue::Float32(l - r))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Some(RuntimeValue::Float64(l - r))
                }

                _ => panic!("Subtract: operand family mismatch with NumericKind"),
            };

            match res {
                Some(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                None => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Overflow,
                )),
            }
        }

        Instruction::Multiply(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Int8)
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Int16)
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Int32)
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Int64)
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Int128)
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Uint8)
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Uint16)
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Uint32)
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Uint64)
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    l.checked_mul(r).map(RuntimeValue::Uint128)
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    Some(RuntimeValue::Float32(l * r))
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    Some(RuntimeValue::Float64(l * r))
                }

                _ => panic!("Multiply: operand family mismatch with NumericKind"),
            };

            match res {
                Some(val) => {
                    push_operand(execution, val);
                    advance_ip(execution);
                    Ok(None)
                }
                None => Err(make_evaluation_failure(
                    execution,
                    EvaluationFailure::Overflow,
                )),
            }
        }

        Instruction::Divide(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);

            let res: Result<RuntimeValue, EvaluationFailure> = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Int8)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Int16)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Int32)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Int64)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Int128)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Uint8)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Uint16)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Uint32)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Uint64)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_div(r)
                            .map(RuntimeValue::Uint128)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    if r == 0.0 || r == -0.0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        Ok(RuntimeValue::Float32(l / r))
                    }
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    if r == 0.0 || r == -0.0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        Ok(RuntimeValue::Float64(l / r))
                    }
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
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Int8)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Int16)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Int32)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Int64)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Int128)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Uint8)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Uint16)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Uint32)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Uint64)
                            .ok_or(EvaluationFailure::Overflow)
                    }
                }
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    if r == 0 {
                        Err(EvaluationFailure::DivisionByZero)
                    } else {
                        l.checked_rem(r)
                            .map(RuntimeValue::Uint128)
                            .ok_or(EvaluationFailure::Overflow)
                    }
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
            let is_equal = compare_numeric_equality(left, right, kind);
            push_operand(execution, RuntimeValue::Boolean(is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::NotEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let is_equal = compare_numeric_equality(left, right, kind);
            push_operand(execution, RuntimeValue::Boolean(!is_equal));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LessNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => l < r,
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => l < r,
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => l < r,
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => l < r,
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => l < r,

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => l < r,
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => l < r,
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => l < r,
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => l < r,
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => l < r,

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => l < r,
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => l < r,

                _ => panic!("LessNumeric: operand family mismatch with NumericKind"),
            };
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::LessEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => l <= r,
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => l <= r,
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => l <= r,
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => l <= r,
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => l <= r,

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => l <= r,
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => l <= r,
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => l <= r,
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => l <= r,
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    l <= r
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    l <= r
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    l <= r
                }

                _ => panic!("LessEqualNumeric: operand family mismatch with NumericKind"),
            };
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::GreaterNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => l > r,
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => l > r,
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => l > r,
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => l > r,
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => l > r,

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => l > r,
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => l > r,
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => l > r,
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => l > r,
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => l > r,

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => l > r,
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => l > r,

                _ => panic!("GreaterNumeric: operand family mismatch with NumericKind"),
            };
            push_operand(execution, RuntimeValue::Boolean(res));
            advance_ip(execution);
            Ok(None)
        }

        Instruction::GreaterEqualNumeric(kind) => {
            let right = pop_operand(execution);
            let left = pop_operand(execution);
            let res = match (kind, left, right) {
                (NumericKind::Int8, RuntimeValue::Int8(l), RuntimeValue::Int8(r)) => l >= r,
                (NumericKind::Int16, RuntimeValue::Int16(l), RuntimeValue::Int16(r)) => l >= r,
                (NumericKind::Int32, RuntimeValue::Int32(l), RuntimeValue::Int32(r)) => l >= r,
                (NumericKind::Int64, RuntimeValue::Int64(l), RuntimeValue::Int64(r)) => l >= r,
                (NumericKind::Int128, RuntimeValue::Int128(l), RuntimeValue::Int128(r)) => l >= r,

                (NumericKind::Uint8, RuntimeValue::Uint8(l), RuntimeValue::Uint8(r)) => l >= r,
                (NumericKind::Uint16, RuntimeValue::Uint16(l), RuntimeValue::Uint16(r)) => l >= r,
                (NumericKind::Uint32, RuntimeValue::Uint32(l), RuntimeValue::Uint32(r)) => l >= r,
                (NumericKind::Uint64, RuntimeValue::Uint64(l), RuntimeValue::Uint64(r)) => l >= r,
                (NumericKind::Uint128, RuntimeValue::Uint128(l), RuntimeValue::Uint128(r)) => {
                    l >= r
                }

                (NumericKind::Float32, RuntimeValue::Float32(l), RuntimeValue::Float32(r)) => {
                    l >= r
                }
                (NumericKind::Float64, RuntimeValue::Float64(l), RuntimeValue::Float64(r)) => {
                    l >= r
                }

                _ => panic!("GreaterEqualNumeric: operand family mismatch with NumericKind"),
            };
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
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(l + r)),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(l + r)),
                    );
                    advance_ip(execution);
                    Ok(None)
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
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(l - r)),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(l - r)),
                    );
                    advance_ip(execution);
                    Ok(None)
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
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(l * r)),
                    );
                    advance_ip(execution);
                    Ok(None)
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    push_operand(
                        execution,
                        RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(l * r)),
                    );
                    advance_ip(execution);
                    Ok(None)
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
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(l / r)),
                        );
                        advance_ip(execution);
                        Ok(None)
                    }
                }
                (RuntimeDynamicValue::Float64(l), RuntimeDynamicValue::Float64(r)) => {
                    if r == 0.0 || r == -0.0 {
                        Err(make_evaluation_failure(
                            execution,
                            EvaluationFailure::DivisionByZero,
                        ))
                    } else {
                        push_operand(
                            execution,
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(l / r)),
                        );
                        advance_ip(execution);
                        Ok(None)
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
            execution
                .call_frames
                .last_mut()
                .expect("active CallFrame")
                .instruction_pointer = InstructionPointer(target.0);
            Ok(None)
        }

        Instruction::JumpIfFalse(target) => {
            let operand = pop_operand(execution);
            let condition = match operand {
                RuntimeValue::Boolean(b) => b,
                _ => panic!("JumpIfFalse expected Boolean operand"),
            };

            if !condition {
                execution
                    .call_frames
                    .last_mut()
                    .expect("active CallFrame")
                    .instruction_pointer = InstructionPointer(target.0);
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
            push_operand(execution, RuntimeValue::Boolean(!b));
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
            push_operand(execution, RuntimeValue::Boolean(l == r));
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
            push_operand(execution, RuntimeValue::Boolean(l != r));
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
            push_operand(execution, RuntimeValue::Boolean(l_str == r_str));
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
            push_operand(execution, RuntimeValue::Boolean(l_str != r_str));
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

            let mut canonical_fields = vec![RuntimeValue::Boolean(false); n];
            for (eval_idx, field_dest) in field_order.iter().enumerate() {
                canonical_fields[field_dest.0] = evaluated[eval_idx];
            }

            let id = StructBackingId(execution.backing_store.structs.len());
            execution.backing_store.structs.push(StructBacking {
                fields: canonical_fields.into_boxed_slice(),
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

            let mut canonical_fields = vec![RuntimeValue::Boolean(false); n];
            for (eval_idx, field_dest) in field_order.iter().enumerate() {
                canonical_fields[field_dest.0] = evaluated[eval_idx];
            }

            let id = EnumBackingId(execution.backing_store.enums.len());
            execution.backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(variant.0),
                payload: RuntimeEnumPayload::Structured {
                    fields: canonical_fields.into_boxed_slice(),
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

    use crate::data::compiled::identities::{
        CompiledValueShapeId, ConstantId, ExternalSymbolId, InstructionIndex,
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
                instructions: vec![Instruction::LoadConstant(ConstantId(0))],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: vec![Constant::Int32(123)],
            external_symbols: Vec::new(),
            value_shapes: vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: vec![vec![SourceSpan { start: 0, end: 5 }]],
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
}
