use crate::definitions::contracts::delete;
use crate::definitions::use_cases::delete as delete_use_case;
use crate::resolvers::delete_resolver;

pub fn delete(capability: delete::Delete, target: &str) -> Result<(), delete_use_case::Error> {
    match delete_resolver::resolve(capability, target) {
        Ok(()) => Ok(()),
        Err(delete_resolver::Error::Unavailable) => Err(delete_use_case::Error::DeleteUnavailable),
    }
}
