use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;

use evo_shell_engine::{FilesystemEntryKind, FilesystemIterationItem};
use time::{OffsetDateTime, UtcOffset, format_description};

use crate::presentation_style;

pub(crate) const BOLD: &str = "\x1b[1m";
pub(crate) const MAGENTA: &str = "\x1b[35m";
pub(crate) const YELLOW: &str = "\x1b[33m";
pub(crate) const INDEX_WIDTH: usize = 3;
pub(crate) const DATETIME_WIDTH: usize = 20;
pub(crate) const TYPE_WIDTH: usize = 7;
pub(crate) const SIZE_WIDTH: usize = 9;

pub(crate) fn render_header() -> String {
    format!(
        "\n{BOLD}{}{:<3} {:<20} {:<20} {:<7} {:<9} {}{}\n",
        presentation_style::PRIMARY_STYLE,
        "#",
        "Created",
        "Modified",
        "Type",
        "Size",
        "Name",
        presentation_style::RESET
    )
}

pub(crate) fn render_row(item: &FilesystemIterationItem) -> String {
    let entry = item.entry();
    let index = format_index(item.index());
    let kind = format_type(entry.kind());
    let size = format_size_cell(entry.size(), entry.kind());
    let created = format_created_cell(entry.created());
    let modified = format_modified_cell(entry.modified());
    let name = format_name(entry.name(), entry.kind());
    let name = color_name(&name, entry.kind());

    format!("{index} {created} {modified} {kind} {size} {name}")
}

pub(crate) fn render_footer(path: &Path, directories: usize, files: usize) -> String {
    format!(
        "\n{}\n{}\n{}\n\n",
        format_directory_count(directories),
        format_file_count(files),
        render_path(path)
    )
}

pub(crate) fn render_path(path: &Path) -> String {
    format!(
        "{}Path:{} {}{}{}",
        presentation_style::PROMPT_SCOPE_STYLE,
        presentation_style::RESET,
        presentation_style::PROMPT_LOCATION_STYLE,
        path.display(),
        presentation_style::RESET
    )
}

pub(crate) fn format_kind(kind: FilesystemEntryKind) -> &'static str {
    match kind {
        FilesystemEntryKind::File => "file",
        FilesystemEntryKind::Directory => "dir",
        FilesystemEntryKind::Symlink => "symlink",
        FilesystemEntryKind::Other => "other",
    }
}

pub(crate) fn format_size(size: Option<u64>) -> String {
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

pub(crate) fn format_size_cell(size: Option<u64>, kind: FilesystemEntryKind) -> String {
    let visible = format_size(size);
    let padding = SIZE_WIDTH.saturating_sub(visible.len());

    if visible.is_empty() {
        return " ".repeat(SIZE_WIDTH);
    }

    if kind == FilesystemEntryKind::File {
        format!(
            "{}{}{}{}",
            presentation_style::FILE_STYLE,
            visible,
            presentation_style::RESET,
            " ".repeat(padding)
        )
    } else {
        format!("{}{}", visible, " ".repeat(padding))
    }
}

pub(crate) fn format_decimal_unit(value: f64, unit: &str) -> String {
    let rounded = (value * 10.0).round() / 10.0;

    if rounded.fract() == 0.0 {
        format!("{} {unit}", rounded as u64)
    } else {
        format!("{rounded:.1} {unit}")
    }
}

pub(crate) fn format_modified(modified: Option<SystemTime>) -> String {
    format_system_time(modified)
}

pub(crate) fn format_created(created: Option<SystemTime>) -> String {
    format_system_time(created)
}

pub(crate) fn format_created_cell(created: Option<SystemTime>) -> String {
    let visible = format_created(created);
    let padding = DATETIME_WIDTH.saturating_sub(visible.len());

    if visible.is_empty() {
        " ".repeat(DATETIME_WIDTH)
    } else {
        format!(
            "{}{}{}{}",
            presentation_style::CREATED_STYLE,
            visible,
            presentation_style::RESET,
            " ".repeat(padding)
        )
    }
}

pub(crate) fn format_modified_cell(modified: Option<SystemTime>) -> String {
    let visible = format_modified(modified);
    let padding = DATETIME_WIDTH.saturating_sub(visible.len());

    if visible.is_empty() {
        " ".repeat(DATETIME_WIDTH)
    } else {
        format!(
            "{}{}{}{}",
            presentation_style::MODIFIED_STYLE,
            visible,
            presentation_style::RESET,
            " ".repeat(padding)
        )
    }
}

pub(crate) fn format_system_time(time: Option<SystemTime>) -> String {
    let Some(time) = time else {
        return String::new();
    };

    let utc = OffsetDateTime::from(time);
    let offset = UtcOffset::local_offset_at(utc).unwrap_or(UtcOffset::UTC);

    format_offset_datetime(utc.to_offset(offset))
}

pub(crate) fn format_offset_datetime(datetime: OffsetDateTime) -> String {
    let Ok(format) =
        format_description::parse_borrowed::<2>("[day]/[month]/[year] [hour]:[minute]")
    else {
        return String::new();
    };

    datetime.format(&format).unwrap_or_default()
}

pub(crate) fn format_name(name: &OsStr, kind: FilesystemEntryKind) -> String {
    let name = name.to_string_lossy();

    match kind {
        FilesystemEntryKind::Directory => format!("{name}/"),
        FilesystemEntryKind::Symlink => format!("{name}@"),
        FilesystemEntryKind::File | FilesystemEntryKind::Other => name.into_owned(),
    }
}

pub(crate) fn color_for_kind(kind: FilesystemEntryKind) -> &'static str {
    match kind {
        FilesystemEntryKind::File => presentation_style::FILE_STYLE,
        FilesystemEntryKind::Directory => presentation_style::LOCATION_STYLE,
        FilesystemEntryKind::Symlink => MAGENTA,
        FilesystemEntryKind::Other => YELLOW,
    }
}

pub(crate) fn color_name(name: &str, kind: FilesystemEntryKind) -> String {
    color_segment(name, kind)
}

pub(crate) fn format_type(kind: FilesystemEntryKind) -> String {
    let visible = format_kind(kind);
    let padding = TYPE_WIDTH.saturating_sub(visible.len());

    format!("{}{}", color_segment(visible, kind), " ".repeat(padding))
}

pub(crate) fn format_index(index: usize) -> String {
    let visible = format!("{index:<INDEX_WIDTH$}");

    format!(
        "{}{}{}",
        presentation_style::PRIMARY_STYLE,
        visible,
        presentation_style::RESET
    )
}

pub(crate) fn color_segment(value: &str, kind: FilesystemEntryKind) -> String {
    let color = color_for_kind(kind);

    if color.is_empty() {
        value.to_string()
    } else {
        format!("{color}{value}{}", presentation_style::RESET)
    }
}

pub(crate) fn format_directory_count(count: usize) -> String {
    let word = if count == 1 {
        "directory"
    } else {
        "directories"
    };

    format!(
        "{BOLD}{}{}{}{} {}{}",
        presentation_style::LOCATION_STYLE,
        count,
        presentation_style::RESET,
        presentation_style::LOCATION_STYLE,
        word,
        presentation_style::RESET
    )
}

pub(crate) fn format_file_count(count: usize) -> String {
    let word = if count == 1 { "file" } else { "files" };

    format!(
        "{BOLD}{}{}{}{} {}{}",
        presentation_style::FILE_STYLE,
        count,
        presentation_style::RESET,
        presentation_style::FILE_STYLE,
        word,
        presentation_style::RESET
    )
}
