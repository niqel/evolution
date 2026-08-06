use evo_shell_engine::{FilesystemEntryKind, ProjectedValue, StructuredProjection};

#[allow(unused_imports)]
pub(crate) use crate::agents::iteration_rendering::{BOLD, MAGENTA, YELLOW};
use crate::agents::iteration_rendering::{
    format_kind, format_system_time, render_footer, render_header, render_row,
};
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline_value::{PipelineItems, PipelineValue};
use crate::definitions::providers::pipeline_result_presenter::Provide;
use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;

pub fn resolve(
    shell: &Shell,
    value: PipelineValue,
    provide: Provide,
) -> Result<(), PipelineResultPresentError> {
    let rendered = render(shell, &value);
    provide(&rendered)
}

fn render(shell: &Shell, value: &PipelineValue) -> String {
    match value {
        PipelineValue::StructuredItems(items) => render_structured_items(shell, items),
        PipelineValue::StructuredProjection(projection) => render_structured_projection(projection),
        PipelineValue::Value(value) => render_scalar(value),
        PipelineValue::Values(values) => render_lines(values.items()),
        PipelineValue::Arguments(arguments) => render_lines(arguments.items()),
    }
}

fn render_structured_items(shell: &Shell, items: &PipelineItems) -> String {
    let items = items.structured_items();
    let mut files = 0;
    let mut directories = 0;
    let mut output = String::new();

    output.push_str(&render_header());

    for item in items.iter() {
        let entry = item.entry();
        match entry.kind() {
            FilesystemEntryKind::File => files += 1,
            FilesystemEntryKind::Directory => directories += 1,
            FilesystemEntryKind::Symlink | FilesystemEntryKind::Other => {}
        }

        output.push_str(&render_row(item));
        output.push('\n');
    }

    output.push_str(&render_footer(
        shell.filesystem_scope().path(),
        directories,
        files,
    ));

    output
}

fn render_structured_projection(projection: &StructuredProjection) -> String {
    let headers = projection
        .properties()
        .iter()
        .map(format_property_label)
        .collect::<Vec<_>>();
    let rows = projection
        .rows()
        .iter()
        .map(|row| {
            row.values()
                .iter()
                .map(render_projected_value)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let widths = column_widths(&headers, &rows);
    let mut output = String::new();

    if headers.is_empty() {
        output.push('\n');
        return output;
    }

    output.push_str(&render_projection_row(&headers, &widths));
    output.push('\n');
    output.push_str(&render_projection_separator(&widths));

    for row in rows {
        output.push('\n');
        output.push_str(&render_projection_row(&row, &widths));
    }

    output.push('\n');
    output
}

fn render_projection_row(values: &[String], widths: &[usize]) -> String {
    values
        .iter()
        .zip(widths.iter())
        .map(|(value, width)| format!("{value:<width$}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_projection_separator(widths: &[usize]) -> String {
    widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect::<Vec<_>>()
        .join(" ")
}

fn column_widths(headers: &[String], rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths = headers
        .iter()
        .map(|header| header.len())
        .collect::<Vec<_>>();

    for row in rows {
        for (index, value) in row.iter().enumerate() {
            if index >= widths.len() {
                widths.push(value.len());
            } else if value.len() > widths[index] {
                widths[index] = value.len();
            }
        }
    }

    widths
}

fn format_property_label(property: &evo_shell_engine::SelectProperty) -> String {
    match property {
        evo_shell_engine::SelectProperty::Index => "Index".to_string(),
        evo_shell_engine::SelectProperty::Created => "Created".to_string(),
        evo_shell_engine::SelectProperty::Modified => "Modified".to_string(),
        evo_shell_engine::SelectProperty::Type => "Type".to_string(),
        evo_shell_engine::SelectProperty::Size => "Size".to_string(),
        evo_shell_engine::SelectProperty::Name => "Name".to_string(),
        evo_shell_engine::SelectProperty::Unsupported(value) => value.clone(),
    }
}

fn render_projected_value(value: &ProjectedValue) -> String {
    match value {
        ProjectedValue::Index(index) => index.to_string(),
        ProjectedValue::Created(created) => format_system_time(*created),
        ProjectedValue::Modified(modified) => format_system_time(*modified),
        ProjectedValue::Type(kind) => format_kind(*kind).to_string(),
        ProjectedValue::Size(size) => size.map(|value| value.to_string()).unwrap_or_default(),
        ProjectedValue::Name(name) => name.to_string_lossy().into_owned(),
    }
}

fn render_scalar(value: &ProjectedValue) -> String {
    let rendered = render_projected_value(value);

    if rendered.is_empty() {
        "\n".to_string()
    } else {
        format!("{rendered}\n")
    }
}

fn render_lines(values: &[ProjectedValue]) -> String {
    if values.is_empty() {
        return "\n".to_string();
    }

    let mut output = values
        .iter()
        .map(render_projected_value)
        .collect::<Vec<_>>()
        .join("\n");
    output.push('\n');
    output
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::SystemTime;

    use evo_shell_engine::{
        Arguments, ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection, Values,
        iteration_advancer, iterator, scope_setter,
    };

    use crate::agents::iteration_presenter;
    use crate::definitions::domain::entities::shell::Shell;
    use crate::definitions::domain::value_objects::pipeline_value::{PipelineItems, PipelineValue};
    use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;
    use crate::presentation_style;

    use super::{BOLD, MAGENTA, YELLOW, resolve};

    thread_local! {
        static CAPTURED: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

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
                "evo_shell_pipeline_result_presenter_{name}_{}_{}",
                std::process::id(),
                unique
            ));

            fs::create_dir(&path).expect("temporary test directory should be created");

            Self { path }
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn shell_from_directory(directory: &TestDirectory) -> Shell {
        Shell::new(scope_setter::set(directory.path.as_path()).unwrap())
    }

    fn capture(shell: &Shell, value: PipelineValue) -> String {
        fn provide_to(rendered: &str) -> Result<(), PipelineResultPresentError> {
            CAPTURED.with(|captured| {
                captured.borrow_mut().extend_from_slice(rendered.as_bytes());
            });
            Ok(())
        }

        CAPTURED.with(|captured| captured.borrow_mut().clear());

        resolve(shell, value, provide_to).unwrap();

        let output = CAPTURED.with(|captured| captured.borrow().clone());

        String::from_utf8(output).unwrap()
    }

    #[test]
    fn pipeline_result_presenter_matches_use_case_function_pointer() {
        let present: crate::PresentPipelineResult = crate::pipeline_result_presenter::present;

        let _ = present;
    }

    #[test]
    fn value_name_is_rendered_as_scalar_text() {
        let directory = TestDirectory::new("value_name");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Value(ProjectedValue::name("only.txt"));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "only.txt\n");
    }

    #[test]
    fn value_index_is_rendered_as_scalar_text() {
        let directory = TestDirectory::new("value_index");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Value(ProjectedValue::index(3));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "3\n");
    }

    #[test]
    fn value_optional_none_is_rendered_without_null_marker() {
        let directory = TestDirectory::new("value_none");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Value(ProjectedValue::size(None));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "\n");
    }

    #[test]
    fn values_render_one_per_line_in_order() {
        let directory = TestDirectory::new("values");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Values(Values::new(vec![
            ProjectedValue::name("a"),
            ProjectedValue::name("b"),
            ProjectedValue::name("c"),
        ]));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "a\nb\nc\n");
    }

    #[test]
    fn empty_values_render_successfully_without_content() {
        let directory = TestDirectory::new("values_empty");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Values(Values::new(vec![]));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "\n");
    }

    #[test]
    fn arguments_render_one_per_line_in_order() {
        let directory = TestDirectory::new("arguments");
        let shell = shell_from_directory(&directory);
        let value = PipelineValue::Arguments(Arguments::new(vec![
            ProjectedValue::name("a"),
            ProjectedValue::name("b"),
        ]));

        let rendered = capture(&shell, value);

        assert_eq!(rendered, "a\nb\n");
    }

    #[test]
    fn structured_projection_renders_headers_and_rows() {
        let directory = TestDirectory::new("projection");
        let shell = shell_from_directory(&directory);
        let projection = StructuredProjection::new(
            vec![SelectProperty::Name, SelectProperty::Size],
            vec![
                ProjectedRow::new(vec![
                    ProjectedValue::name("a.txt"),
                    ProjectedValue::size(Some(10)),
                ]),
                ProjectedRow::new(vec![
                    ProjectedValue::name("b.txt"),
                    ProjectedValue::size(Some(20)),
                ]),
            ],
        );

        let rendered = capture(&shell, PipelineValue::StructuredProjection(projection));

        assert!(rendered.contains("Name"));
        assert!(rendered.contains("Size"));
        assert!(rendered.contains("a.txt"));
        assert!(rendered.contains("b.txt"));
        assert!(!rendered.contains("Debug"));
    }

    #[test]
    fn structured_items_match_iteration_presentation_shape() {
        let directory = TestDirectory::new("structured_items");
        fs::write(directory.path.join("first.txt"), b"one").unwrap();
        fs::write(directory.path.join("second.txt"), b"two").unwrap();
        let shell = shell_from_directory(&directory);

        let mut first_iteration = iterator::iter(shell.filesystem_scope()).unwrap();
        let mut items = Vec::new();
        while let Some(item) = iteration_advancer::advance(&mut first_iteration).unwrap() {
            items.push(item);
        }

        let second_iteration = iterator::iter(shell.filesystem_scope()).unwrap();
        let mut iteration_output = Vec::new();
        iteration_presenter::present_to(&mut iteration_output, second_iteration).unwrap();

        let rendered = capture(
            &shell,
            PipelineValue::StructuredItems(PipelineItems::new(items)),
        );

        assert_eq!(
            strip_styles(&rendered),
            strip_styles(&String::from_utf8(iteration_output).unwrap())
        );
    }

    #[test]
    fn provider_propagates_io_error() {
        struct FailingWriter;

        impl io::Write for FailingWriter {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("write failed"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let mut writer = FailingWriter;
        let result = crate::providers::pipeline_result_presenter::provide_to(&mut writer, "x");

        assert!(matches!(result, Err(PipelineResultPresentError::Io(_))));
    }

    #[test]
    fn present_with_delegates_to_resolver_and_provider() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);

        fn provide(rendered: &str) -> Result<(), PipelineResultPresentError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 1);
            assert_eq!(rendered, "ok\n");
            Ok(())
        }

        let directory = TestDirectory::new("present_with");
        let shell = shell_from_directory(&directory);

        let result = crate::pipeline_result_presenter::present_with(
            |shell, value, provide| {
                assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 0);
                assert!(shell.filesystem_scope().path().is_dir());
                assert!(matches!(value, PipelineValue::Value(_)));
                provide("ok\n")
            },
            provide,
            &shell,
            PipelineValue::Value(ProjectedValue::name("only.txt")),
        );

        assert!(result.is_ok());
        assert_eq!(ORDER.load(Ordering::SeqCst), 2);
    }

    fn strip_styles(input: &str) -> String {
        input
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
}
