use alloc::vec::Vec;

use crate::data::compiled::program::CompiledProgram;
use crate::data::semantic::ids::FunctionId;
use crate::data::vm::backing::ExecutionBackingStore;
use crate::data::vm::bindings::ApplicationBindings;
use crate::data::vm::values::RuntimeValue;

pub(crate) struct SharedValueStorage {
    pub(crate) cells: Vec<Option<RuntimeValue>>,
}

pub(crate) struct InstructionPointer(pub(crate) usize);

pub(crate) struct CallFrame {
    pub(crate) function: FunctionId,
    pub(crate) instruction_pointer: InstructionPointer,
    pub(crate) frame_base: usize,
}

pub(crate) struct VmExecution<'compiled, 'bindings> {
    pub(crate) compiled_program: &'compiled CompiledProgram,
    pub(crate) application_bindings: &'bindings ApplicationBindings,
    pub(crate) value_storage: SharedValueStorage,
    pub(crate) backing_store: ExecutionBackingStore,
    pub(crate) call_frames: Vec<CallFrame>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::compiled::identities::ConstantId;
    use crate::data::compiled::instructions::Instruction;
    use crate::data::compiled::program::CompiledFunction;
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::compiled::storage::Constant;
    use crate::data::lexical::SourceSpan;
    use std::collections::HashMap;

    #[test]
    fn shared_value_storage_cells_some_and_none() {
        let storage = SharedValueStorage {
            cells: alloc::vec![
                Some(RuntimeValue::Int32(10)),
                None,
                Some(RuntimeValue::Boolean(true)),
            ],
        };

        assert_eq!(storage.cells.len(), 3);
        match storage.cells[0] {
            Some(RuntimeValue::Int32(v)) => assert_eq!(v, 10),
            _ => panic!("expected Some(Int32) at cell 0"),
        }
        match storage.cells[1] {
            None => {}
            _ => panic!("expected None at cell 1"),
        }
        match storage.cells[2] {
            Some(RuntimeValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Some(Boolean) at cell 2"),
        }
    }

    #[test]
    fn instruction_pointer_tuple_field() {
        let ip = InstructionPointer(7);
        assert_eq!(ip.0, 7);
    }

    #[test]
    fn call_frame_fields_and_multiple_frames_same_function() {
        let frame1 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };

        let frame2 = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(5),
            frame_base: 8,
        };

        assert_eq!(frame1.function.0, 0);
        assert_eq!(frame1.instruction_pointer.0, 0);
        assert_eq!(frame1.frame_base, 0);

        assert_eq!(frame2.function.0, 0);
        assert_eq!(frame2.instruction_pointer.0, 5);
        assert_eq!(frame2.frame_base, 8);
    }

    #[test]
    fn vm_execution_exact_five_fields_and_independent_borrows() {
        let program = CompiledProgram {
            functions: alloc::vec![CompiledFunction {
                parameter_count: 0,
                local_count: 0,
                max_operand_depth: 1,
                instructions: alloc::vec![
                    Instruction::LoadConstant(ConstantId(0)),
                    Instruction::Return,
                ],
            }],
            entry_point: FunctionId(0),
            entry_parameter_shapes: alloc::vec![],
            constants: alloc::vec![Constant::Boolean(true)],
            external_symbols: alloc::vec![],
            value_shapes: alloc::vec![],
            source_map: SourceMap {
                functions: alloc::vec![alloc::vec![
                    SourceSpan { start: 0, end: 5 },
                    SourceSpan { start: 5, end: 10 },
                ]],
            },
        };

        let bindings = ApplicationBindings {
            capabilities: HashMap::new(),
        };

        let value_storage = SharedValueStorage {
            cells: alloc::vec![],
        };

        let backing_store = ExecutionBackingStore {
            strings: alloc::vec![],
            dynamic_integers: alloc::vec![],
            structs: alloc::vec![],
            enums: alloc::vec![],
        };

        let entry_frame = CallFrame {
            function: FunctionId(0),
            instruction_pointer: InstructionPointer(0),
            frame_base: 0,
        };

        let vm = VmExecution {
            compiled_program: &program,
            application_bindings: &bindings,
            value_storage,
            backing_store,
            call_frames: alloc::vec![entry_frame],
        };

        // Check 5 fields
        assert_eq!(vm.compiled_program.functions.len(), 1);
        assert_eq!(vm.application_bindings.capabilities.len(), 0);
        assert_eq!(vm.value_storage.cells.len(), 0);
        assert_eq!(vm.backing_store.strings.len(), 0);
        assert_eq!(vm.call_frames.len(), 1);

        // Check entry call frame
        assert_eq!(vm.call_frames[0].function.0, 0);
        assert_eq!(vm.call_frames[0].instruction_pointer.0, 0);
        assert_eq!(vm.call_frames[0].frame_base, 0);
    }
}
