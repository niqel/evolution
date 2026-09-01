use alloc::string::String;
use alloc::vec::Vec;

use crate::data::compiled::identities::CompiledValueShapeId;
use crate::data::semantic::SignatureSymbol;

pub(crate) struct ExternalSymbol {
    pub(crate) symbol: SignatureSymbol,
    pub(crate) parameter_count: usize,
    pub(crate) result_shape: CompiledValueShapeId,
}

pub(crate) enum Constant {
    Boolean(bool),
    String(String),

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

    Dynamic(DynamicConstant),
}

pub(crate) enum DynamicConstant {
    Integer { negative: bool, magnitude: Vec<u8> },
    Float32(f32),
    Float64(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn external_symbol_fields() {
        let ext = ExternalSymbol {
            symbol: SignatureSymbol {
                module: "Math".to_string(),
                name: "Add".to_string(),
            },
            parameter_count: 2,
            result_shape: CompiledValueShapeId(0),
        };

        assert_eq!(ext.symbol.module, "Math");
        assert_eq!(ext.symbol.name, "Add");
        assert_eq!(ext.parameter_count, 2);
        assert_eq!(ext.result_shape.0, 0);
    }

    #[test]
    fn constant_15_variants_and_signed_min() {
        let c_bool = Constant::Boolean(true);
        match c_bool {
            Constant::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }

        let c_str = Constant::String("hello".to_string());
        match &c_str {
            Constant::String(s) => assert_eq!(s, "hello"),
            _ => panic!("expected String"),
        }

        let c_i8_min = Constant::Int8(-128);
        match c_i8_min {
            Constant::Int8(v) => assert_eq!(v, -128),
            _ => panic!("expected Int8"),
        }

        let c_i16 = Constant::Int16(-32768);
        match c_i16 {
            Constant::Int16(v) => assert_eq!(v, -32768),
            _ => panic!("expected Int16"),
        }

        let c_i32 = Constant::Int32(-2147483648);
        match c_i32 {
            Constant::Int32(v) => assert_eq!(v, -2147483648),
            _ => panic!("expected Int32"),
        }

        let c_i64 = Constant::Int64(-9223372036854775808);
        match c_i64 {
            Constant::Int64(v) => assert_eq!(v, -9223372036854775808),
            _ => panic!("expected Int64"),
        }

        let c_i128 = Constant::Int128(-170141183460469231731687303715884105728);
        match c_i128 {
            Constant::Int128(v) => assert_eq!(v, -170141183460469231731687303715884105728),
            _ => panic!("expected Int128"),
        }

        let c_u8 = Constant::Uint8(255);
        match c_u8 {
            Constant::Uint8(v) => assert_eq!(v, 255),
            _ => panic!("expected Uint8"),
        }

        let c_u16 = Constant::Uint16(65535);
        match c_u16 {
            Constant::Uint16(v) => assert_eq!(v, 65535),
            _ => panic!("expected Uint16"),
        }

        let c_u32 = Constant::Uint32(4294967295);
        match c_u32 {
            Constant::Uint32(v) => assert_eq!(v, 4294967295),
            _ => panic!("expected Uint32"),
        }

        let c_u64 = Constant::Uint64(18446744073709551615);
        match c_u64 {
            Constant::Uint64(v) => assert_eq!(v, 18446744073709551615),
            _ => panic!("expected Uint64"),
        }

        let c_u128 = Constant::Uint128(340282366920938463463374607431768211455);
        match c_u128 {
            Constant::Uint128(v) => assert_eq!(v, 340282366920938463463374607431768211455),
            _ => panic!("expected Uint128"),
        }

        let c_f32 = Constant::Float32(1.5);
        match c_f32 {
            Constant::Float32(v) => assert!((v - 1.5).abs() < 1e-6),
            _ => panic!("expected Float32"),
        }

        let c_f64 = Constant::Float64(42.5);
        match c_f64 {
            Constant::Float64(v) => assert!((v - 42.5).abs() < 1e-10),
            _ => panic!("expected Float64"),
        }

        let c_dyn = Constant::Dynamic(DynamicConstant::Integer {
            negative: false,
            magnitude: alloc::vec![],
        });
        match c_dyn {
            Constant::Dynamic(DynamicConstant::Integer {
                negative,
                magnitude,
            }) => {
                assert!(!negative);
                assert!(magnitude.is_empty());
            }
            _ => panic!("expected Dynamic Integer"),
        }
    }

    #[test]
    fn dynamic_constant_3_variants_and_canonical_integers() {
        let d_zero = DynamicConstant::Integer {
            negative: false,
            magnitude: alloc::vec![],
        };
        match d_zero {
            DynamicConstant::Integer {
                negative,
                magnitude,
            } => {
                assert!(!negative);
                assert_eq!(magnitude.len(), 0);
            }
            _ => panic!("expected Integer zero"),
        }

        let d_pos = DynamicConstant::Integer {
            negative: false,
            magnitude: alloc::vec![0x01, 0x00],
        };
        match d_pos {
            DynamicConstant::Integer {
                negative,
                magnitude,
            } => {
                assert!(!negative);
                assert_eq!(magnitude.len(), 2);
                assert_eq!(magnitude[0], 0x01);
                assert_eq!(magnitude[1], 0x00);
            }
            _ => panic!("expected Integer 256"),
        }

        let d_neg = DynamicConstant::Integer {
            negative: true,
            magnitude: alloc::vec![0xFF],
        };
        match d_neg {
            DynamicConstant::Integer {
                negative,
                magnitude,
            } => {
                assert!(negative);
                assert_eq!(magnitude.len(), 1);
                assert_eq!(magnitude[0], 0xFF);
            }
            _ => panic!("expected Integer -255"),
        }

        let d_f32 = DynamicConstant::Float32(2.5);
        match d_f32 {
            DynamicConstant::Float32(v) => assert!((v - 2.5).abs() < 1e-6),
            _ => panic!("expected Float32"),
        }

        let d_f64 = DynamicConstant::Float64(100.25);
        match d_f64 {
            DynamicConstant::Float64(v) => assert!((v - 100.25).abs() < 1e-10),
            _ => panic!("expected Float64"),
        }
    }
}
