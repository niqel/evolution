use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

use evo_shell_engine::{
    FilesystemEntry, FilesystemEntryKind, FilesystemIteration, IterError, iteration_advancer,
};
use time::{OffsetDateTime, UtcOffset, format_description};

const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";
const TYPE_WIDTH: usize = 7;

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

    writeln!(writer)?;
    render_path(writer, iteration.path())?;
    writeln!(writer)?;

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
                writeln!(writer)?;
                render_path(writer, iteration.path())?;
                writeln!(writer)?;
                return Ok(());
            }
            Err(error) => return Err(PresentIterationError::Iter(error)),
        }
    }
}

fn render_path(writer: &mut impl Write, path: &Path) -> io::Result<()> {
    writeln!(writer, "Path: {}", path.display())
}

fn render_row(
    writer: &mut impl Write,
    index: usize,
    entry: &FilesystemEntry,
) -> Result<(), PresentIterationError> {
    let kind = format_type(entry.kind());
    let size = format_size(entry.size());
    let modified = format_modified(entry.modified());
    let name = format_name(entry.name(), entry.kind());
    let name = color_name(&name, entry.kind());

    writeln!(
        writer,
        "{:<3} {:<20} {} {:<9} {}",
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

    let utc = OffsetDateTime::from(modified);
    let offset = UtcOffset::local_offset_at(utc).unwrap_or(UtcOffset::UTC);

    format_offset_datetime(utc.to_offset(offset))
}

fn format_offset_datetime(datetime: OffsetDateTime) -> String {
    let Ok(format) =
        format_description::parse_borrowed::<2>("[day]/[month]/[year] [hour]:[minute]")
    else {
        return String::new();
    };

    datetime.format(&format).unwrap_or_default()
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
    color_segment(name, kind)
}

fn format_type(kind: FilesystemEntryKind) -> String {
    let visible = format_kind(kind);
    let padding = TYPE_WIDTH.saturating_sub(visible.len());

    format!("{}{}", color_segment(visible, kind), " ".repeat(padding))
}

fn color_segment(value: &str, kind: FilesystemEntryKind) -> String {
    let color = color_for_kind(kind);

    if color.is_empty() {
        value.to_string()
    } else {
        format!("{color}{value}{RESET}")
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
        BOLD, CYAN, RESET, color_name, format_directory_count, format_file_count, format_kind,
        format_modified, format_offset_datetime, format_size, format_type, present_to,
    };
    use evo_shell_engine::{FilesystemEntryKind, iteration_advancer, iterator, scope_setter};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use time::{Date, Month, PrimitiveDateTime, Time, UtcOffset};

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("system time should be after UNIX_EPOCH")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "evo_shell_presenter_{name}_{}_{}",
                std::process::id(),
                unique
            ));

            fs::create_dir(&path).expect("temporary test directory should be created");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn format_kind_uses_user_visible_values() {
        assert_eq!(format_kind(FilesystemEntryKind::File), "file");
        assert_eq!(format_kind(FilesystemEntryKind::Directory), "dir");
        assert_eq!(format_kind(FilesystemEntryKind::Symlink), "symlink");
        assert_eq!(format_kind(FilesystemEntryKind::Other), "other");
    }

    #[test]
    fn directory_kind_and_name_use_same_color() {
        assert_eq!(
            format_type(FilesystemEntryKind::Directory),
            format!("{CYAN}dir{RESET}    ")
        );
        assert_eq!(
            color_name("release/", FilesystemEntryKind::Directory),
            format!("{CYAN}release/{RESET}")
        );
    }

    #[test]
    fn file_kind_and_name_remain_unstyled() {
        assert_eq!(format_type(FilesystemEntryKind::File), "file   ");
        assert_eq!(
            color_name("Cargo.toml", FilesystemEntryKind::File),
            "Cargo.toml"
        );
    }

    #[test]
    fn type_padding_uses_visible_width_before_ansi_color() {
        let file = format!(
            "{:<3} {:<20} {} {:<9} {}",
            0,
            "05/08/2026 01:43",
            format_type(FilesystemEntryKind::File),
            "327.6 kB",
            "libevo_shell.rlib"
        );
        let directory = format!(
            "{:<3} {:<20} {} {:<9} {}",
            1,
            "05/08/2026 00:18",
            format_type(FilesystemEntryKind::Directory),
            "",
            color_name("build/", FilesystemEntryKind::Directory)
        );
        let file_visible = file.replace(CYAN, "").replace(RESET, "");
        let directory_visible = directory.replace(CYAN, "").replace(RESET, "");

        assert!(file.contains("file    327.6 kB"));
        assert!(directory.contains(&format!("{CYAN}dir{RESET}")));
        assert!(directory.contains(&format!("{CYAN}build/{RESET}")));
        assert_eq!(
            file_visible.find("libevo_shell.rlib"),
            directory_visible.find("build/")
        );
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
    fn format_offset_datetime_uses_required_calendar_format() {
        let date = Date::from_calendar_date(2026, Month::January, 1).unwrap();
        let time = Time::from_hms(7, 5, 9).unwrap();
        let datetime = PrimitiveDateTime::new(date, time).assume_offset(UtcOffset::UTC);

        assert_eq!(format_offset_datetime(datetime), "01/01/2026 07:05");
    }

    #[test]
    fn format_offset_datetime_uses_zero_padding_for_day_month_hour_and_minute() {
        let date = Date::from_calendar_date(2026, Month::August, 5).unwrap();
        let time = Time::from_hms(0, 4, 59).unwrap();
        let datetime = PrimitiveDateTime::new(date, time).assume_offset(UtcOffset::UTC);

        assert_eq!(format_offset_datetime(datetime), "05/08/2026 00:04");
    }

    #[test]
    fn format_modified_does_not_emit_unix_or_seconds() {
        let time = UNIX_EPOCH + Duration::from_secs(42);
        let formatted = format_modified(Some(time));

        assert!(!formatted.contains("unix"));
        assert!(!formatted.ends_with('s'));
        assert_eq!(formatted.len(), "01/01/1970 00:00".len());
    }

    #[test]
    fn format_modified_none_is_empty() {
        assert_eq!(format_modified(None), "");
    }

    #[test]
    fn summary_uses_singular_and_plural() {
        assert_eq!(format_directory_count(1), "1 directory");
        assert_eq!(format_directory_count(2), "2 directories");
        assert_eq!(format_file_count(1), "1 file");
        assert_eq!(format_file_count(2), "2 files");
    }

    #[test]
    fn present_renders_path_before_header_and_after_summary() {
        let directory = TestDirectory::new("path_context");
        fs::write(directory.path().join("Cargo.toml"), "manifest").unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();
        let expected_path = scope.path().display().to_string();
        let mut output = Vec::new();

        present_to(&mut output, iteration).unwrap();

        let output = String::from_utf8(output).unwrap();
        let path_line = format!("Path: {expected_path}");
        assert!(output.starts_with('\n'));
        assert!(output.contains(&format!("\n{path_line}\n\n{BOLD}#")));
        assert!(output.contains("#   Modified"));
        assert!(output.contains("Cargo.toml"));
        assert!(output.contains("src/"));
        assert!(output.contains("1 directory\n1 file"));
        assert!(output.contains(&format!("1 directory\n1 file\n\n{path_line}\n")));
        assert!(output.ends_with('\n'));
        assert_eq!(output.matches(&path_line).count(), 2);
        assert!(!output.contains("unix"));
    }

    #[test]
    fn present_empty_iteration_renders_path_header_zero_summary_and_path() {
        let directory = TestDirectory::new("empty_path_context");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();
        let expected_path = scope.path().display().to_string();
        let mut output = Vec::new();

        present_to(&mut output, iteration).unwrap();

        let output = String::from_utf8(output).unwrap();
        let path_line = format!("Path: {expected_path}");
        assert!(output.contains(&format!("\n{path_line}\n\n{BOLD}#")));
        assert!(output.contains("0 directories\n0 files"));
        assert!(output.contains(&format!("0 directories\n0 files\n\n{path_line}\n")));
        assert_eq!(output.matches(&path_line).count(), 2);
    }

    #[test]
    fn present_does_not_change_lazy_advance_semantics() {
        let directory = TestDirectory::new("lazy_advance_unchanged");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        assert!(
            iteration_advancer::advance(&mut iteration)
                .unwrap()
                .is_some()
        );
        assert!(
            iteration_advancer::advance(&mut iteration)
                .unwrap()
                .is_none()
        );
    }
}
