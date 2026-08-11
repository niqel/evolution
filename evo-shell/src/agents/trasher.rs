use crate::definitions::contracts::trash;
use crate::definitions::use_cases::trash as trash_use_case;
use crate::resolvers::trash_resolver;

pub fn trash(capability: trash::Trash, target: &str) -> Result<(), trash_use_case::Error> {
    match trash_resolver::resolve(capability, target) {
        Ok(()) => Ok(()),
        Err(trash_resolver::Error::Unavailable) => Err(trash_use_case::Error::TrashUnavailable),
    }
}
