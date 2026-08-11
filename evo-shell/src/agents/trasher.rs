use crate::definitions::contracts::trash;
use crate::definitions::use_cases::trash as trash_use_case;
use crate::resolvers::trash_resolver;

pub fn trash(capability: trash::Trash, target: &str) -> Result<(), trash_use_case::Error> {
    trash_resolver::resolve(capability, target).map_err(|_| trash_use_case::Error::TrashUnavailable)
}
