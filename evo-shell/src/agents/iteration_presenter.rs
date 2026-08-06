use std::io::{self, Write};

use evo_shell_engine::{FilesystemEntryKind, FilesystemIteration, IterError, iteration_advancer};

#[allow(unused_imports)]
pub(crate) use crate::agents::iteration_rendering::{
    BOLD, MAGENTA, YELLOW, color_name, format_created, format_created_cell, format_decimal_unit,
    format_directory_count, format_file_count, format_index, format_kind, format_modified,
    format_modified_cell, format_offset_datetime, format_size, format_size_cell, format_type,
};

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

    write!(
        writer,
        "{}",
        crate::agents::iteration_rendering::render_header()
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

                write!(
                    writer,
                    "{}\n",
                    crate::agents::iteration_rendering::render_row(&item)
                )?;
            }
            Ok(None) => {
                write!(
                    writer,
                    "{}",
                    crate::agents::iteration_rendering::render_footer(
                        iteration.path(),
                        directories,
                        files,
                    )
                )?;
                return Ok(());
            }
            Err(error) => return Err(PresentIterationError::Iter(error)),
        }
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
        BOLD, MAGENTA, YELLOW, color_name, format_created, format_created_cell,
        format_directory_count, format_file_count, format_index, format_kind, format_modified,
        format_modified_cell, format_offset_datetime, format_size, format_size_cell, format_type,
        present_to,
    };
    use crate::presentation_style;
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

    fn without_styles(output: &str) -> String {
        output
            .replace(BOLD, "")
            .replace(MAGENTA, "")
            .replace(YELLOW, "")
            .replace(presentation_style::PRIMARY_STYLE, "")
            .replace(presentation_style::LOCATION_STYLE, "")
            .replace(presentation_style::FILE_STYLE, "")
            .replace(presentation_style::CREATED_STYLE, "")
            .replace(presentation_style::MODIFIED_STYLE, "")
            .replace(presentation_style::PROMPT_SCOPE_STYLE, "")
            .replace(presentation_style::PROMPT_LOCATION_STYLE, "")
            .replace(presentation_style::RESET, "")
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
            format!(
                "{}dir{}    ",
                presentation_style::LOCATION_STYLE,
                presentation_style::RESET
            )
        );
        assert_eq!(
            color_name("release/", FilesystemEntryKind::Directory),
            format!(
                "{}release/{}",
                presentation_style::LOCATION_STYLE,
                presentation_style::RESET
            )
        );
    }

    #[test]
    fn file_kind_and_name_use_file_style() {
        assert_eq!(
            format_type(FilesystemEntryKind::File),
            format!(
                "{}file{}   ",
                presentation_style::FILE_STYLE,
                presentation_style::RESET
            )
        );
        assert_eq!(
            color_name("Cargo.toml", FilesystemEntryKind::File),
            format!(
                "{}Cargo.toml{}",
                presentation_style::FILE_STYLE,
                presentation_style::RESET
            )
        );
    }

    #[test]
    fn format_index_uses_primary_style_with_visible_padding() {
        assert_eq!(
            format_index(7),
            format!(
                "{}7  {}",
                presentation_style::PRIMARY_STYLE,
                presentation_style::RESET
            )
        );
    }

    #[test]
    fn type_padding_uses_visible_width_before_ansi_color() {
        let file = format!(
            "{:<3} {} {} {} {} {}",
            0,
            format_created_cell(Some(UNIX_EPOCH + Duration::from_secs(42))),
            format_modified_cell(Some(UNIX_EPOCH + Duration::from_secs(43))),
            format_type(FilesystemEntryKind::File),
            format_size_cell(Some(327_600), FilesystemEntryKind::File),
            "libevo_shell.rlib"
        );
        let directory = format!(
            "{:<3} {} {} {} {} {}",
            1,
            format_created_cell(None),
            format_modified_cell(Some(UNIX_EPOCH + Duration::from_secs(44))),
            format_type(FilesystemEntryKind::Directory),
            format_size_cell(None, FilesystemEntryKind::Directory),
            color_name("build/", FilesystemEntryKind::Directory)
        );
        let file_visible = without_styles(&file);
        let directory_visible = without_styles(&directory);

        assert!(file.contains(&format!(
            "{}file{}   ",
            presentation_style::FILE_STYLE,
            presentation_style::RESET
        )));
        assert!(!file.contains(&format!("{}05/08/2026", presentation_style::FILE_STYLE)));
        assert!(file.contains(&format!(
            "{}327.6 kB{} ",
            presentation_style::FILE_STYLE,
            presentation_style::RESET
        )));
        assert!(directory.contains(&format!(
            "{}dir{}",
            presentation_style::LOCATION_STYLE,
            presentation_style::RESET
        )));
        assert!(directory.contains(&format!(
            "{}build/{}",
            presentation_style::LOCATION_STYLE,
            presentation_style::RESET
        )));
        assert_eq!(
            file_visible.find("libevo_shell.rlib"),
            directory_visible.find("build/")
        );
    }

    #[test]
    fn temporal_cells_use_distinct_teals_without_bold() {
        let created = format_created_cell(Some(UNIX_EPOCH + Duration::from_secs(42)));
        let modified = format_modified_cell(Some(UNIX_EPOCH + Duration::from_secs(42)));

        assert_eq!(presentation_style::CREATED_STYLE, "\x1b[38;2;33;142;128m");
        assert_eq!(presentation_style::MODIFIED_STYLE, "\x1b[38;2;24;130;115m");
        assert!(created.starts_with(presentation_style::CREATED_STYLE));
        assert!(!created.contains(BOLD));
        assert!(created.contains(presentation_style::RESET));
        assert!(modified.starts_with(presentation_style::MODIFIED_STYLE));
        assert!(!modified.contains(BOLD));
        assert!(modified.contains(presentation_style::RESET));
        assert_ne!(
            presentation_style::CREATED_STYLE,
            presentation_style::MODIFIED_STYLE
        );
        assert_eq!(without_styles(&created).len(), 20);
        assert_eq!(without_styles(&modified).len(), 20);
    }

    #[test]
    fn empty_temporal_cells_keep_width_without_style() {
        assert_eq!(format_created_cell(None), " ".repeat(20));
        assert_eq!(format_modified_cell(None), " ".repeat(20));
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
    fn format_size_cell_styles_file_size_without_changing_visible_width() {
        assert_eq!(
            format_size_cell(Some(228), FilesystemEntryKind::File),
            format!(
                "{}228 B{}    ",
                presentation_style::FILE_STYLE,
                presentation_style::RESET
            )
        );
        assert_eq!(
            format_size_cell(None, FilesystemEntryKind::Directory),
            " ".repeat(9)
        );
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
    fn format_created_uses_same_datetime_rules_as_modified() {
        let time = UNIX_EPOCH + Duration::from_secs(42);
        let created = format_created(Some(time));
        let modified = format_modified(Some(time));

        assert_eq!(created, modified);
        assert!(!created.contains("unix"));
        assert_eq!(created.len(), "01/01/1970 00:00".len());
    }

    #[test]
    fn format_created_none_is_empty() {
        assert_eq!(format_created(None), "");
    }

    #[test]
    fn summary_uses_singular_and_plural() {
        assert_eq!(without_styles(&format_directory_count(1)), "1 directory");
        assert_eq!(without_styles(&format_directory_count(2)), "2 directories");
        assert_eq!(without_styles(&format_file_count(1)), "1 file");
        assert_eq!(without_styles(&format_file_count(2)), "2 files");
        assert_eq!(
            format_directory_count(2),
            format!(
                "{BOLD}{}2{}{} directories{}",
                presentation_style::LOCATION_STYLE,
                presentation_style::RESET,
                presentation_style::LOCATION_STYLE,
                presentation_style::RESET
            )
        );
        assert_eq!(
            format_file_count(2),
            format!(
                "{BOLD}{}2{}{} files{}",
                presentation_style::FILE_STYLE,
                presentation_style::RESET,
                presentation_style::FILE_STYLE,
                presentation_style::RESET
            )
        );
    }

    #[test]
    fn present_renders_header_after_initial_blank_and_path_as_footer() {
        let directory = TestDirectory::new("path_context");
        fs::write(directory.path().join("Cargo.toml"), "manifest").unwrap();
        fs::create_dir(directory.path().join("src")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();
        let expected_path = scope.path().display().to_string();
        let mut output = Vec::new();

        present_to(&mut output, iteration).unwrap();

        let output = String::from_utf8(output).unwrap();
        let visible_output = without_styles(&output);
        let visible_path_line = format!("Path: {expected_path}");
        let styled_path_line = format!(
            "{}Path:{} {}{}{}",
            presentation_style::PRIMARY_STYLE,
            presentation_style::RESET,
            presentation_style::LOCATION_STYLE,
            expected_path,
            presentation_style::RESET
        );
        assert!(output.starts_with('\n'));
        assert!(output.starts_with(&format!("\n{BOLD}{}#", presentation_style::PRIMARY_STYLE)));
        assert!(visible_output.contains("#   Created"));
        assert!(visible_output.find("Created").unwrap() < visible_output.find("Modified").unwrap());
        assert!(visible_output.contains("Cargo.toml"));
        assert!(visible_output.contains("src/"));
        assert!(visible_output.contains("1 directory\n1 file"));
        assert!(visible_output.contains(&format!("1 directory\n1 file\n{visible_path_line}\n\n")));
        assert!(!visible_output.contains(&format!("1 file\n\n{visible_path_line}")));
        assert_eq!(visible_output.matches(&visible_path_line).count(), 1);
        assert!(
            visible_output.find("#   Created").unwrap()
                < visible_output.find(&visible_path_line).unwrap()
        );
        assert!(output.contains(&styled_path_line));
        assert!(output.contains(&format!(
            "{}0  {}",
            presentation_style::PRIMARY_STYLE,
            presentation_style::RESET
        )));
        assert!(output.contains(&format!(
            "{}file{}",
            presentation_style::FILE_STYLE,
            presentation_style::RESET
        )));
        assert!(visible_output.ends_with("\n\n"));
        assert!(!output.contains("unix"));
    }

    #[test]
    fn present_empty_iteration_renders_footer_path_without_path_header() {
        let directory = TestDirectory::new("empty_path_context");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();
        let expected_path = scope.path().display().to_string();
        let mut output = Vec::new();

        present_to(&mut output, iteration).unwrap();

        let output = String::from_utf8(output).unwrap();
        let visible_output = without_styles(&output);
        let path_line = format!("Path: {expected_path}");
        assert!(output.starts_with(&format!("\n{BOLD}{}#", presentation_style::PRIMARY_STYLE)));
        assert!(visible_output.contains("0 directories\n0 files"));
        assert!(visible_output.contains(&format!("0 directories\n0 files\n{path_line}\n\n")));
        assert!(!visible_output.contains(&format!("0 files\n\n{path_line}")));
        assert_eq!(visible_output.matches(&path_line).count(), 1);
    }

    #[test]
    fn footer_path_reuses_prompt_styles_and_resets_each_segment() {
        let directory = TestDirectory::new("footer_style");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();
        let expected_path = scope.path().display().to_string();
        let mut output = Vec::new();

        present_to(&mut output, iteration).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(&format!(
            "{}Path:{} {}{}{}\n\n",
            presentation_style::PRIMARY_STYLE,
            presentation_style::RESET,
            presentation_style::LOCATION_STYLE,
            expected_path,
            presentation_style::RESET
        )));
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
