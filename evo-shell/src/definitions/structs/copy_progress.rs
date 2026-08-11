#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyProgress {
    pub total_bytes: Option<u64>,
    pub copied_bytes: u64,
}
