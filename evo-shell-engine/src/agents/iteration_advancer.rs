use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::use_cases::iter::IterError;
use crate::providers;
use crate::resolvers::filesystem_entry;

pub fn advance(
    iteration: &mut FilesystemIteration,
) -> Result<Option<FilesystemIterationItem>, IterError> {
    let Some(entry) =
        filesystem_entry::resolve(iteration, providers::next_directory_entry::provide)?
    else {
        return Ok(None);
    };

    let index = iteration.next_index();
    iteration.advance_index();

    Ok(Some(FilesystemIterationItem::new(index, entry)))
}
