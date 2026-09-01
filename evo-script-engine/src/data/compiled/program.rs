use alloc::vec::Vec;

use crate::data::compiled::boundary::CompiledValueShape;
use crate::data::compiled::identities::CompiledValueShapeId;
use crate::data::compiled::instructions::Instruction;
use crate::data::compiled::source_map::SourceMap;
use crate::data::compiled::storage::{Constant, ExternalSymbol};
use crate::data::semantic::ids::FunctionId;

pub(crate) struct CompiledProgram {
    pub(crate) functions: Vec<CompiledFunction>,
    pub(crate) entry_point: FunctionId,
    pub(crate) entry_parameter_shapes: Vec<CompiledValueShapeId>,
    pub(crate) constants: Vec<Constant>,
    pub(crate) external_symbols: Vec<ExternalSymbol>,
    pub(crate) value_shapes: Vec<CompiledValueShape>,
    pub(crate) source_map: SourceMap,
}

pub(crate) struct CompiledFunction {
    pub(crate) parameter_count: usize,
    pub(crate) local_count: usize,
    pub(crate) max_operand_depth: usize,
    pub(crate) instructions: Vec<Instruction>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::compiled::identities::ConstantId;
    use crate::data::lexical::SourceSpan;
    use crate::data::semantic::SignatureSymbol;
    use alloc::string::ToString;

    #[test]
    fn compiled_function_fields() {
        let func = CompiledFunction {
            parameter_count: 2,
            local_count: 3,
            max_operand_depth: 4,
            instructions: alloc::vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Return,
            ],
        };

        assert_eq!(func.parameter_count, 2);
        assert_eq!(func.local_count, 3);
        assert_eq!(func.max_operand_depth, 4);
        assert_eq!(func.instructions.len(), 2);
    }

    #[test]
    fn compiled_program_minimal_valid() {
        let func = CompiledFunction {
            parameter_count: 1,
            local_count: 0,
            max_operand_depth: 1,
            instructions: alloc::vec![
                Instruction::LoadConstant(ConstantId(0)),
                Instruction::Return,
            ],
        };

        let program = CompiledProgram {
            functions: alloc::vec![func],
            entry_point: FunctionId(0),
            entry_parameter_shapes: alloc::vec![CompiledValueShapeId(0)],
            constants: alloc::vec![Constant::Int32(42)],
            external_symbols: alloc::vec![ExternalSymbol {
                symbol: SignatureSymbol {
                    module: "Math".to_string(),
                    name: "Square".to_string(),
                },
                parameter_count: 1,
                result_shape: CompiledValueShapeId(0),
            }],
            value_shapes: alloc::vec![CompiledValueShape::Int32],
            source_map: SourceMap {
                functions: alloc::vec![alloc::vec![
                    SourceSpan { start: 0, end: 2 },
                    SourceSpan { start: 2, end: 4 },
                ]],
            },
        };

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.entry_point.0, 0);
        assert_eq!(
            program.entry_parameter_shapes.len(),
            program.functions[0].parameter_count
        );
        assert_eq!(program.constants.len(), 1);
        assert_eq!(program.external_symbols.len(), 1);
        assert_eq!(program.value_shapes.len(), 1);
        assert_eq!(program.source_map.functions.len(), program.functions.len());
        assert_eq!(
            program.source_map.functions[0].len(),
            program.functions[0].instructions.len()
        );
    }
}
