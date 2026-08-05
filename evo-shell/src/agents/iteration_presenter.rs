use std::ffi::OsStr;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use evo_shell_engine::{
    FilesystemEntry, FilesystemEntryKind, FilesystemIteration, IterError, iteration_advancer,
};

const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

pub fn present(iteration: FilesystemIteration) -> Result<(), PresentIterationError> {
    let mut stdout = io::stdout();
    present_to(&mut stdout, iteration)
}

pub fn present_to(
    writer: &mut impl Write,
    mut iteration: FilesystemIteration,
) -> Result<(), PresentIterationError> {
    let mut files = 0;
    let mut directories = 0;

    writeln!(
        writer,
        "{BOLD}{:<3} {:<20} {:<7} {:<9} {}{RESET}",
        "#", "Modified", "Type", "Size", "Name"
    )?;

    loop {
        match iteration_advancer::advance(&mut iteration) {
            Ok(Some(item)) => {
                let entry = item.entry();
                match entry.kind() {
                    FilesystemEntryKind::File => files += 1,
                    FilesystemEntryKind::Directory => directories += 1,
                    FilesystemEntryKind::Symlink | FilesystemEntryKind::Other => {}
                }

                render_row(writer, item.index(), entry)?;
            }
            Ok(None) => {
                writeln!(writer)?;
                writeln!(writer, "{}", format_directory_count(directories))?;
                writeln!(writer, "{}", format_file_count(files))?;
                return Ok(());
            }
            Err(error) => return Err(PresentIterationError::Iter(error)),
        }
    }
}

fn render_row(
    writer: &mut impl Write,
    index: usize,
    entry: &FilesystemEntry,
) -> Result<(), PresentIterationError> {
    let kind = format_kind(entry.kind());
    let size = format_size(entry.size());
    let modified = format_modified(entry.modified());
    let name = format_name(entry.name(), entry.kind());
    let name = color_name(&name, entry.kind());

    writeln!(
        writer,
        "{:<3} {:<20} {:<7} {:<9} {}",
        index, modified, kind, size, name
    )?;

    Ok(())
}

fn format_kind(kind: FilesystemEntryKind) -> &'static str {
    match kind {
        FilesystemEntryKind::File => "file",
        FilesystemEntryKind::Directory => "dir",
        FilesystemEntryKind::Symlink => "symlink",
        FilesystemEntryKind::Other => "other",
    }
}

fn format_size(size: Option<u64>) -> String {
    let Some(bytes) = size else {
        return String::new();
    };

    if bytes < 1_000 {
        return format!("{bytes} B");
    }

    if bytes < 1_000_000 {
        return format_decimal_unit(bytes as f64 / 1_000.0, "kB");
    }

    format_decimal_unit(bytes as f64 / 1_000_000.0, "MB")
}

fn format_decimal_unit(value: f64, unit: &str) -> String {
    let rounded = (value * 10.0).round() / 10.0;

    if rounded.fract() == 0.0 {
        format!("{} {unit}", rounded as u64)
    } else {
        format!("{rounded:.1} {unit}")
    }
}

fn format_modified(modified: Option<SystemTime>) -> String {
    let Some(modified) = modified else {
        return String::new();
    };

    match modified.duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix {}s", duration.as_secs()),
        Err(_) => "before unix epoch".to_string(),
    }
}

fn format_name(name: &OsStr, kind: FilesystemEntryKind) -> String {
    let name = name.to_string_lossy();

    match kind {
        FilesystemEntryKind::Directory => format!("{name}/"),
        FilesystemEntryKind::Symlink => format!("{name}@"),
        FilesystemEntryKind::File | FilesystemEntryKind::Other => name.into_owned(),
    }
}

fn color_for_kind(kind: FilesystemEntryKind) -> &'static str {
    match kind {
        FilesystemEntryKind::File => "",
        FilesystemEntryKind::Directory => CYAN,
        FilesystemEntryKind::Symlink => MAGENTA,
        FilesystemEntryKind::Other => YELLOW,
    }
}

fn color_name(name: &str, kind: FilesystemEntryKind) -> String {
    let color = color_for_kind(kind);

    if color.is_empty() {
        name.to_string()
    } else {
        format!("{color}{name}{RESET}")
    }
}

fn format_directory_count(count: usize) -> String {
    if count == 1 {
        "1 directory".to_string()
    } else {
        format!("{count} directories")
    }
}

fn format_file_count(count: usize) -> String {
    if count == 1 {
        "1 file".to_string()
    } else {
        format!("{count} files")
    }
}

#[derive(Debug)]
pub enum PresentIterationError {
    Io(io::Error),
    Iter(IterError),
}

impl From<io::Error> for PresentIterationError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        format_directory_count, format_file_count, format_kind, format_modified, format_size,
    };
    use evo_shell_engine::FilesystemEntryKind;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn format_kind_uses_user_visible_values() {
        assert_eq!(format_kind(FilesystemEntryKind::File), "file");
        assert_eq!(format_kind(FilesystemEntryKind::Directory), "dir");
        assert_eq!(format_kind(FilesystemEntryKind::Symlink), "symlink");
        assert_eq!(format_kind(FilesystemEntryKind::Other), "other");
    }

    #[test]
    fn format_size_uses_decimal_units() {
        assert_eq!(format_size(Some(0)), "0 B");
        assert_eq!(format_size(Some(151)), "151 B");
        assert_eq!(format_size(Some(1_200)), "1.2 kB");
        assert_eq!(format_size(Some(52_700)), "52.7 kB");
        assert_eq!(format_size(Some(2_400_000)), "2.4 MB");
        assert_eq!(format_size(Some(1_000)), "1 kB");
        assert_eq!(format_size(None), "");
    }

    #[test]
    fn format_modified_uses_structured_system_time_without_calendar_conversion() {
        let time = UNIX_EPOCH + Duration::from_secs(42);

        assert_eq!(format_modified(Some(time)), "unix 42s");
        assert_eq!(format_modified(None), "");
    }

    #[test]
    fn summary_uses_singular_and_plural() {
        assert_eq!(format_directory_count(1), "1 directory");
        assert_eq!(format_directory_count(2), "2 directories");
        assert_eq!(format_file_count(1), "1 file");
        assert_eq!(format_file_count(2), "2 files");
    }
}
