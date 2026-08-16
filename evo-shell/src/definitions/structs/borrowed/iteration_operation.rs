#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationOperation<'operation> {
    Select(&'operation [&'operation str]),
    ToValue,
    Take(usize),
    Skip(usize),
    First,
    Last,
    Count,
    Iter,
}
