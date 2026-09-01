use evo_values::Value;

use crate::data::compilation_dependency::CompilationCatalog;
use crate::data::failures::ExecutionOutcome;
use crate::data::vm::bindings::ApplicationBindings;

pub type ExecuteSource = for<'source, 'value, 'catalog, 'bindings> fn(
    &'source str,
    &'value [Value<'value>],
    &'catalog CompilationCatalog,
    &'bindings ApplicationBindings,
) -> ExecutionOutcome;
