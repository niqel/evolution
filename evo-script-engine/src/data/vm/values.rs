use crate::data::compiled::identities::ConstantId;

#[derive(Clone, Copy)]
pub(crate) struct StringBackingId(pub(crate) usize);

#[derive(Clone, Copy)]
pub(crate) struct DynamicIntegerBackingId(pub(crate) usize);

#[derive(Clone, Copy)]
pub(crate) struct StructBackingId(pub(crate) usize);

#[derive(Clone, Copy)]
pub(crate) struct EnumBackingId(pub(crate) usize);

#[derive(Clone, Copy)]
pub(crate) enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}

#[derive(Clone, Copy)]
pub(crate) enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}

#[derive(Clone, Copy)]
pub(crate) enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}

#[derive(Clone, Copy)]
pub(crate) enum RuntimeValue {
    Boolean(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    Float32(f32),
    Float64(f64),

    String(StringBackingRef),
    Dynamic(DynamicValue),
    Struct(StructBackingId),
    Enum(EnumBackingId),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backing_ids_and_copy_semantics() {
        let s_id = StringBackingId(1);
        let s_copied = s_id;
        assert_eq!(s_id.0, 1);
        assert_eq!(s_copied.0, 1);

        let d_id = DynamicIntegerBackingId(2);
        let d_copied = d_id;
        assert_eq!(d_id.0, 2);
        assert_eq!(d_copied.0, 2);

        let st_id = StructBackingId(3);
        let st_copied = st_id;
        assert_eq!(st_id.0, 3);
        assert_eq!(st_copied.0, 3);

        let e_id = EnumBackingId(4);
        let e_copied = e_id;
        assert_eq!(e_id.0, 4);
        assert_eq!(e_copied.0, 4);
    }

    #[test]
    fn backing_refs_variants_and_copy_semantics() {
        let s_ref_compiled = StringBackingRef::Compiled(ConstantId(10));
        let s_ref_compiled_copied = s_ref_compiled;
        match s_ref_compiled_copied {
            StringBackingRef::Compiled(cid) => assert_eq!(cid.0, 10),
            _ => panic!("expected Compiled"),
        }

        let s_ref_exec = StringBackingRef::Execution(StringBackingId(20));
        let s_ref_exec_copied = s_ref_exec;
        match s_ref_exec_copied {
            StringBackingRef::Execution(id) => assert_eq!(id.0, 20),
            _ => panic!("expected Execution"),
        }

        let d_ref_compiled = DynamicIntegerBackingRef::Compiled(ConstantId(30));
        let d_ref_compiled_copied = d_ref_compiled;
        match d_ref_compiled_copied {
            DynamicIntegerBackingRef::Compiled(cid) => assert_eq!(cid.0, 30),
            _ => panic!("expected Compiled"),
        }

        let d_ref_exec = DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(40));
        let d_ref_exec_copied = d_ref_exec;
        match d_ref_exec_copied {
            DynamicIntegerBackingRef::Execution(id) => assert_eq!(id.0, 40),
            _ => panic!("expected Execution"),
        }
    }

    #[test]
    fn dynamic_value_3_variants_and_copy_semantics() {
        let dyn_int = DynamicValue::Integer(DynamicIntegerBackingRef::Compiled(ConstantId(1)));
        let dyn_int_copied = dyn_int;
        match dyn_int_copied {
            DynamicValue::Integer(DynamicIntegerBackingRef::Compiled(cid)) => assert_eq!(cid.0, 1),
            _ => panic!("expected Integer"),
        }

        let dyn_f32 = DynamicValue::Float32(1.5);
        let dyn_f32_copied = dyn_f32;
        match dyn_f32_copied {
            DynamicValue::Float32(v) => assert!((v - 1.5).abs() < 1e-6),
            _ => panic!("expected Float32"),
        }

        let dyn_f64 = DynamicValue::Float64(2.5);
        let dyn_f64_copied = dyn_f64;
        match dyn_f64_copied {
            DynamicValue::Float64(v) => assert!((v - 2.5).abs() < 1e-10),
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn runtime_value_exactly_17_variants() {
        let values = [
            RuntimeValue::Boolean(true),
            RuntimeValue::Int8(-1),
            RuntimeValue::Int16(-2),
            RuntimeValue::Int32(-3),
            RuntimeValue::Int64(-4),
            RuntimeValue::Int128(-5),
            RuntimeValue::Uint8(1),
            RuntimeValue::Uint16(2),
            RuntimeValue::Uint32(3),
            RuntimeValue::Uint64(4),
            RuntimeValue::Uint128(5),
            RuntimeValue::Float32(1.25),
            RuntimeValue::Float64(2.5),
            RuntimeValue::String(StringBackingRef::Compiled(ConstantId(0))),
            RuntimeValue::Dynamic(DynamicValue::Integer(DynamicIntegerBackingRef::Execution(
                DynamicIntegerBackingId(0),
            ))),
            RuntimeValue::Struct(StructBackingId(0)),
            RuntimeValue::Enum(EnumBackingId(0)),
        ];

        assert_eq!(values.len(), 17);

        // Pattern matching and Copy semantics checks
        let v_bool = values[0];
        match v_bool {
            RuntimeValue::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }

        let v_i32 = values[3];
        match v_i32 {
            RuntimeValue::Int32(v) => assert_eq!(v, -3),
            _ => panic!("expected Int32"),
        }

        let v_f64 = values[12];
        match v_f64 {
            RuntimeValue::Float64(v) => assert!((v - 2.5).abs() < 1e-10),
            _ => panic!("expected Float64"),
        }

        let v_str_compiled = values[13];
        match v_str_compiled {
            RuntimeValue::String(StringBackingRef::Compiled(cid)) => assert_eq!(cid.0, 0),
            _ => panic!("expected String Compiled"),
        }

        let v_str_exec = RuntimeValue::String(StringBackingRef::Execution(StringBackingId(42)));
        let v_str_exec_copied = v_str_exec;
        match v_str_exec_copied {
            RuntimeValue::String(StringBackingRef::Execution(id)) => assert_eq!(id.0, 42),
            _ => panic!("expected String Execution"),
        }

        let v_dyn_int = values[14];
        match v_dyn_int {
            RuntimeValue::Dynamic(DynamicValue::Integer(DynamicIntegerBackingRef::Execution(
                id,
            ))) => assert_eq!(id.0, 0),
            _ => panic!("expected Dynamic Integer"),
        }

        let v_struct = values[15];
        match v_struct {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Struct"),
        }

        let v_enum = values[16];
        match v_enum {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Enum"),
        }
    }
}
