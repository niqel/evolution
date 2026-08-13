use crate::definitions::contracts::move_item;
use crate::definitions::requesters::move_requester;
use crate::definitions::requesters::transfer_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    MoveUnavailable,
}

pub type Move = for<'origin, 'destination> fn(
    &'origin str,
    &'destination str,
    transfer_progress_requester::Request,
    move_requester::Request,
    move_item::Move,
);
