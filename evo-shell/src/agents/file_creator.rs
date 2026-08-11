use crate::definitions::contracts::create_file;
use crate::definitions::use_cases::create_file as create_file_use_case;
use crate::resolvers::create_file_resolver;

pub fn create_file(
    capability: create_file::CreateFile,
    target: &str,
) -> Result<(), create_file_use_case::Error> {
    match create_file_resolver::resolve(capability, target) {
        Ok(()) => Ok(()),
        Err(create_file_resolver::Error::Unavailable) => {
            Err(create_file_use_case::Error::CreateFileUnavailable)
        }
    }
}
