use crate::data::compilation_dependency::CompilationCatalog;
use crate::data::failures::CompileOutcome;

pub type Compile =
    for<'source, 'catalog> fn(&'source str, &'catalog CompilationCatalog) -> CompileOutcome;
