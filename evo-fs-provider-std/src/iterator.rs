use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::field::Field;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::record::Record;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;

pub fn iterate<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    if !iteration.operations.is_empty() {
        return Err(iterate_contract::Error::ProviderIncompatible);
    }

    let current_dir = std::env::current_dir().map_err(|_| iterate_contract::Error::Unavailable)?;
    let read_dir =
        std::fs::read_dir(&current_dir).map_err(|_| iterate_contract::Error::Unavailable)?;

    for (index, entry_result) in read_dir.enumerate() {
        let entry = entry_result.map_err(|_| iterate_contract::Error::Unavailable)?;

        let file_name = entry.file_name();
        let name_str = file_name
            .to_str()
            .ok_or(iterate_contract::Error::ExternalTypeIncompatible("name"))?;

        let path_buf = entry.path();
        let path_str = path_buf
            .to_str()
            .ok_or(iterate_contract::Error::ExternalTypeIncompatible("path"))?;

        let file_type = entry
            .file_type()
            .map_err(|_| iterate_contract::Error::Unavailable)?;

        let kind_str = if file_type.is_file() {
            "file"
        } else if file_type.is_dir() {
            "directory"
        } else if file_type.is_symlink() {
            "symlink"
        } else {
            "other"
        };

        let index_field = Field {
            name: "index",
            value: Value::Unsigned(index as u64),
        };
        let name_field = Field {
            name: "name",
            value: Value::Text(name_str),
        };
        let path_field = Field {
            name: "path",
            value: Value::Text(path_str),
        };
        let kind_field = Field {
            name: "kind",
            value: Value::Text(kind_str),
        };

        let flow = if file_type.is_file() {
            let metadata = entry
                .metadata()
                .map_err(|_| iterate_contract::Error::Unavailable)?;
            let size_field = Field {
                name: "size",
                value: Value::Unsigned(metadata.len()),
            };
            let fields = [index_field, name_field, path_field, kind_field, size_field];
            let record = Record { fields: &fields };
            request(Construction::Record(record))
        } else {
            let fields = [index_field, name_field, path_field, kind_field];
            let record = Record { fields: &fields };
            request(Construction::Record(record))
        };

        if flow == Flow::Stop {
            return Ok(());
        }
    }

    Ok(())
}

pub const ITERATE: iterate_contract::Iterate = iterate;
