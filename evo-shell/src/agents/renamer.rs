use crate::definitions::contracts::rename;
use crate::definitions::use_cases::rename as rename_use_case;
use crate::resolvers::rename_resolver;

pub fn rename(
    capability: rename::Rename,
    target: &str,
    new_name: &str,
) -> Result<(), rename_use_case::Error> {
    rename_resolver::resolve(capability, target, new_name)
        .map_err(|_| rename_use_case::Error::RenameUnavailable)
}
