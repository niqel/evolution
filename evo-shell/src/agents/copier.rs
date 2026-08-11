use crate::definitions::contracts::copy;
use crate::definitions::use_cases::copy_to;
use crate::resolvers::copy_resolver;

pub fn copy(capability: copy::Copy, origin: &str, destination: &str) -> Result<(), copy_to::Error> {
    match copy_resolver::resolve(capability, origin, destination) {
        Ok(()) => Ok(()),
        Err(copy_resolver::Error::Unavailable) => Err(copy_to::Error::CopyUnavailable),
    }
}
