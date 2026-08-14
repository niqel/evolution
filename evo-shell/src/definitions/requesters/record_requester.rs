use crate::definitions::structs::borrowed::record::Record;
use crate::definitions::structs::owned::flow::Flow;

pub type Request = for<'record> fn(Record<'record>) -> Flow;
