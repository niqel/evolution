use crate::definitions::contracts::create_dir;
use crate::definitions::use_cases::create_dir as create_dir_use_case;
use crate::resolvers::create_dir_resolver;

pub fn create_dir(
    capability: create_dir::CreateDir,
    target: &str,
) -> Result<(), create_dir_use_case::Error> {
    match create_dir_resolver::resolve(capability, target) {
        Ok(()) => Ok(()),
        Err(create_dir_resolver::Error::Unavailable) => {
            Err(create_dir_use_case::Error::CreateDirUnavailable)
        }
    }
}
