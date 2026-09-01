use alloc::vec::Vec;

use crate::data::compiled::equality::CompositeEqualityPlan;
use crate::data::compiled::identities::{
    ConstantId, ExternalSymbolId, FieldIndex, InstructionIndex, LocalSlot, NumericKind,
    ParameterSlot, VariantDiscriminant,
};
use crate::data::semantic::ids::FunctionId;

pub(crate) enum Instruction {
    // Core data movement — 4
    LoadConstant(ConstantId),
    LoadParameter(ParameterSlot),
    LoadLocal(LocalSlot),
    StoreLocal(LocalSlot),

    // Calls — 2
    Call(FunctionId),
    CallExternal(ExternalSymbolId),

    // Fixed numeric — 12
    Negate(NumericKind),
    Add(NumericKind),
    Subtract(NumericKind),
    Multiply(NumericKind),
    Divide(NumericKind),
    Remainder(NumericKind),

    EqualNumeric(NumericKind),
    NotEqualNumeric(NumericKind),
    LessNumeric(NumericKind),
    LessEqualNumeric(NumericKind),
    GreaterNumeric(NumericKind),
    GreaterEqualNumeric(NumericKind),

    // Dynamic numeric — 7
    LiftDynamic(NumericKind),
    DynamicNegate,
    DynamicAdd,
    DynamicSubtract,
    DynamicMultiply,
    DynamicDivide,
    DynamicRemainder,

    // Control flow — 4
    Jump(InstructionIndex),
    JumpIfFalse(InstructionIndex),
    Discard,
    Return,

    // Explicit conversions — 4
    ConvertNumeric {
        source: NumericKind,
        target: NumericKind,
    },
    ConvertDynamic(NumericKind),
    NumericToString(NumericKind),
    DynamicToString,

    // Scalar bool / string — 5
    NotBoolean,
    EqualBoolean,
    NotEqualBoolean,
    EqualString,
    NotEqualString,

    // Composite mechanics — 8
    ConstructStruct {
        field_order: Vec<FieldIndex>,
    },

    GetField(FieldIndex),

    ConstructEnumSimple(VariantDiscriminant),

    ConstructEnumAssociated(VariantDiscriminant),

    ConstructEnumStructured {
        variant: VariantDiscriminant,
        field_order: Vec<FieldIndex>,
    },

    TestVariant(VariantDiscriminant),

    ExtractEnumAssociated,

    ExtractEnumStructured {
        fields: Vec<FieldIndex>,
    },

    // Structural equality — 2
    EqualComposite(CompositeEqualityPlan),

    NotEqualComposite(CompositeEqualityPlan),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_inventory_exactly_48_variants() {
        let all_instructions = [
            Instruction::LoadConstant(ConstantId(0)),
            Instruction::LoadParameter(ParameterSlot(0)),
            Instruction::LoadLocal(LocalSlot(0)),
            Instruction::StoreLocal(LocalSlot(0)),
            Instruction::Call(FunctionId(0)),
            Instruction::CallExternal(ExternalSymbolId(0)),
            Instruction::Negate(NumericKind::Int32),
            Instruction::Add(NumericKind::Int32),
            Instruction::Subtract(NumericKind::Int32),
            Instruction::Multiply(NumericKind::Int32),
            Instruction::Divide(NumericKind::Int32),
            Instruction::Remainder(NumericKind::Int32),
            Instruction::EqualNumeric(NumericKind::Int32),
            Instruction::NotEqualNumeric(NumericKind::Int32),
            Instruction::LessNumeric(NumericKind::Int32),
            Instruction::LessEqualNumeric(NumericKind::Int32),
            Instruction::GreaterNumeric(NumericKind::Int32),
            Instruction::GreaterEqualNumeric(NumericKind::Int32),
            Instruction::LiftDynamic(NumericKind::Int32),
            Instruction::DynamicNegate,
            Instruction::DynamicAdd,
            Instruction::DynamicSubtract,
            Instruction::DynamicMultiply,
            Instruction::DynamicDivide,
            Instruction::DynamicRemainder,
            Instruction::Jump(InstructionIndex(10)),
            Instruction::JumpIfFalse(InstructionIndex(20)),
            Instruction::Discard,
            Instruction::Return,
            Instruction::ConvertNumeric {
                source: NumericKind::Int32,
                target: NumericKind::Float64,
            },
            Instruction::ConvertDynamic(NumericKind::Int32),
            Instruction::NumericToString(NumericKind::Int32),
            Instruction::DynamicToString,
            Instruction::NotBoolean,
            Instruction::EqualBoolean,
            Instruction::NotEqualBoolean,
            Instruction::EqualString,
            Instruction::NotEqualString,
            Instruction::ConstructStruct {
                field_order: alloc::vec![FieldIndex(0), FieldIndex(1)],
            },
            Instruction::GetField(FieldIndex(2)),
            Instruction::ConstructEnumSimple(VariantDiscriminant(0)),
            Instruction::ConstructEnumAssociated(VariantDiscriminant(1)),
            Instruction::ConstructEnumStructured {
                variant: VariantDiscriminant(2),
                field_order: alloc::vec![FieldIndex(0), FieldIndex(1)],
            },
            Instruction::TestVariant(VariantDiscriminant(0)),
            Instruction::ExtractEnumAssociated,
            Instruction::ExtractEnumStructured {
                fields: alloc::vec![FieldIndex(0), FieldIndex(1)],
            },
            Instruction::EqualComposite(CompositeEqualityPlan::Struct {
                fields: alloc::vec![],
            }),
            Instruction::NotEqualComposite(CompositeEqualityPlan::Struct {
                fields: alloc::vec![],
            }),
        ];

        assert_eq!(all_instructions.len(), 48);

        // Pattern match verification of key variant payloads
        match &all_instructions[0] {
            Instruction::LoadConstant(cid) => assert_eq!(cid.0, 0),
            _ => panic!("expected LoadConstant"),
        }

        match &all_instructions[4] {
            Instruction::Call(fid) => assert_eq!(fid.0, 0),
            _ => panic!("expected Call"),
        }

        match &all_instructions[5] {
            Instruction::CallExternal(esid) => assert_eq!(esid.0, 0),
            _ => panic!("expected CallExternal"),
        }

        match &all_instructions[25] {
            Instruction::Jump(idx) => assert_eq!(idx.0, 10),
            _ => panic!("expected Jump"),
        }

        match &all_instructions[26] {
            Instruction::JumpIfFalse(idx) => assert_eq!(idx.0, 20),
            _ => panic!("expected JumpIfFalse"),
        }

        match &all_instructions[29] {
            Instruction::ConvertNumeric { source, target } => match (source, target) {
                (NumericKind::Int32, NumericKind::Float64) => {}
                _ => panic!("unexpected numeric conversion kinds"),
            },
            _ => panic!("expected ConvertNumeric"),
        }

        match &all_instructions[38] {
            Instruction::ConstructStruct { field_order } => {
                assert_eq!(field_order.len(), 2);
                assert_eq!(field_order[0].0, 0);
                assert_eq!(field_order[1].0, 1);
            }
            _ => panic!("expected ConstructStruct"),
        }

        match &all_instructions[42] {
            Instruction::ConstructEnumStructured {
                variant,
                field_order,
            } => {
                assert_eq!(variant.0, 2);
                assert_eq!(field_order.len(), 2);
                assert_eq!(field_order[0].0, 0);
                assert_eq!(field_order[1].0, 1);
            }
            _ => panic!("expected ConstructEnumStructured"),
        }

        match &all_instructions[45] {
            Instruction::ExtractEnumStructured { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, 0);
                assert_eq!(fields[1].0, 1);
            }
            _ => panic!("expected ExtractEnumStructured"),
        }

        match &all_instructions[46] {
            Instruction::EqualComposite(plan) => match plan {
                CompositeEqualityPlan::Struct { fields } => assert_eq!(fields.len(), 0),
                _ => panic!("expected Struct plan"),
            },
            _ => panic!("expected EqualComposite"),
        }

        match &all_instructions[47] {
            Instruction::NotEqualComposite(plan) => match plan {
                CompositeEqualityPlan::Struct { fields } => assert_eq!(fields.len(), 0),
                _ => panic!("expected Struct plan"),
            },
            _ => panic!("expected NotEqualComposite"),
        }
    }
}
