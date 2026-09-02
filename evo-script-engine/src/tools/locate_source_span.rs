use crate::data::compiled::program::CompiledProgram;
use crate::data::lexical::SourceSpan;
use crate::data::vm::state::CallFrame;

pub type LocateSourceSpan = fn(&CompiledProgram, &CallFrame) -> SourceSpan;

pub fn locate_source_span(
    compiled_program: &CompiledProgram,
    call_frame: &CallFrame,
) -> SourceSpan {
    let function_spans = compiled_program
        .source_map
        .functions
        .get(call_frame.function.0)
        .expect("CallFrame function ID must exist in SourceMap");

    *function_spans
        .get(call_frame.instruction_pointer.0)
        .expect("CallFrame instruction pointer must exist in function SourceMap spans")
}

pub const LOCATE_SOURCE_SPAN: LocateSourceSpan = locate_source_span;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::data::compiled::instructions::Instruction;
    use crate::data::compiled::program::CompiledFunction;
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::semantic::ids::FunctionId;
    use crate::data::vm::state::InstructionPointer;

    fn make_program(source_map_functions: Vec<Vec<SourceSpan>>) -> CompiledProgram {
        let functions = source_map_functions
            .iter()
            .map(|spans| CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 0,
                instructions: spans.iter().map(|_| Instruction::Return).collect(),
            })
            .collect();

        CompiledProgram {
            functions,
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: source_map_functions,
            },
        }
    }

    #[test]
    fn typed_binding() {
        let implementation: LocateSourceSpan = locate_source_span;
        let binding: LocateSourceSpan = LOCATE_SOURCE_SPAN;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn first_function_first_instruction() {
        let program = make_program(vec![vec![SourceSpan { start: 0, end: 5 }]]);
        let frame = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };

        let span = locate_source_span(&program, &frame);
        assert_eq!(span, SourceSpan { start: 0, end: 5 });
    }

    #[test]
    fn same_function_different_instructions() {
        let program = make_program(vec![vec![
            SourceSpan { start: 0, end: 5 },
            SourceSpan { start: 5, end: 10 },
            SourceSpan { start: 10, end: 25 },
        ]]);

        let frame_0 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };
        assert_eq!(
            locate_source_span(&program, &frame_0),
            SourceSpan { start: 0, end: 5 }
        );

        let frame_1 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(1),
            frame_base: 0,
        };
        assert_eq!(
            locate_source_span(&program, &frame_1),
            SourceSpan { start: 5, end: 10 }
        );

        let frame_2 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(2),
            frame_base: 0,
        };
        assert_eq!(
            locate_source_span(&program, &frame_2),
            SourceSpan { start: 10, end: 25 }
        );
    }

    #[test]
    fn multiple_functions() {
        let program = make_program(vec![
            vec![SourceSpan { start: 10, end: 20 }],
            vec![SourceSpan {
                start: 100,
                end: 200,
            }],
        ]);

        let frame_fn0 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };
        assert_eq!(
            locate_source_span(&program, &frame_fn0),
            SourceSpan { start: 10, end: 20 }
        );

        let frame_fn1 = CallFrame {
            function: FunctionId(1),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };
        assert_eq!(
            locate_source_span(&program, &frame_fn1),
            SourceSpan {
                start: 100,
                end: 200,
            }
        );
    }

    #[test]
    fn frame_base_independence() {
        let program = make_program(vec![vec![SourceSpan { start: 42, end: 84 }]]);

        let frame_a = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };
        let frame_b = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 100,
        };

        assert_eq!(
            locate_source_span(&program, &frame_a),
            locate_source_span(&program, &frame_b)
        );
        assert_eq!(
            locate_source_span(&program, &frame_a),
            SourceSpan { start: 42, end: 84 }
        );
    }

    #[test]
    fn zero_width_source_span() {
        let program = make_program(vec![vec![SourceSpan { start: 17, end: 17 }]]);
        let frame = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };

        let span = locate_source_span(&program, &frame);
        assert_eq!(span, SourceSpan { start: 17, end: 17 });
    }

    #[test]
    #[should_panic(expected = "CallFrame function ID must exist in SourceMap")]
    fn dangling_function_id_is_invariant_violation() {
        let program = make_program(vec![vec![SourceSpan { start: 0, end: 5 }]]);
        let frame = CallFrame {
            function: FunctionId(99),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };

        locate_source_span(&program, &frame);
    }

    #[test]
    #[should_panic(
        expected = "CallFrame instruction pointer must exist in function SourceMap spans"
    )]
    fn dangling_instruction_pointer_is_invariant_violation() {
        let program = make_program(vec![vec![SourceSpan { start: 0, end: 5 }]]);
        let frame = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(99),
            frame_base: 0,
        };

        locate_source_span(&program, &frame);
    }
}
