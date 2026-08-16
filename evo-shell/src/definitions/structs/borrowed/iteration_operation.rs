#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationOperation<'operation> {
    Select(&'operation [&'operation str]),
    Take(usize),
    Skip(usize),
    First,
    Last,
    Count,
    Iter,
}
