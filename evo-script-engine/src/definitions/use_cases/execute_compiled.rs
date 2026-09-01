use evo_values::Value;

use crate::data::compiled::program::CompiledProgram;
use crate::data::failures::ExecutionOutcome;
use crate::data::vm::bindings::ApplicationBindings;

pub type ExecuteCompiled = for<'compiled, 'value, 'bindings> fn(
    &'compiled CompiledProgram,
    &'value [Value<'value>],
    &'bindings ApplicationBindings,
) -> ExecutionOutcome;
