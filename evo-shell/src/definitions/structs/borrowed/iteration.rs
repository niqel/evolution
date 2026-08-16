use super::iteration_operation::IterationOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Iteration<'iteration> {
    pub operations: &'iteration [IterationOperation<'iteration>],
}
