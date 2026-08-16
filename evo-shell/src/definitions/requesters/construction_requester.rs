use crate::definitions::structs::borrowed::construction::Construction;
use crate::definitions::structs::owned::flow::Flow;

pub type Request = for<'construction> fn(Construction<'construction>) -> Flow;
