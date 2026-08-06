use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;
use crate::definitions::providers::pipeline_result_presenter::Provide;
use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;
use crate::providers::pipeline_result_presenter as provider;
use crate::resolvers::pipeline_result_presenter as resolver;

pub fn present(shell: &Shell, value: PipelineValue) -> Result<(), PipelineResultPresentError> {
    let resolve = resolver::resolve;
    let provide = provider::provide;

    present_with(resolve, provide, shell, value)
}

pub(crate) fn present_with(
    resolve: fn(&Shell, PipelineValue, Provide) -> Result<(), PipelineResultPresentError>,
    provide: Provide,
    shell: &Shell,
    value: PipelineValue,
) -> Result<(), PipelineResultPresentError> {
    resolve(shell, value, provide)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::definitions::domain::entities::shell::Shell;
    use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;
    use crate::definitions::providers::pipeline_result_presenter::Provide;
    use crate::definitions::use_cases::pipeline_result_presenter::{
        PipelineResultPresentError, PresentPipelineResult,
    };
    use evo_shell_engine::scope_setter;

    use super::{present, present_with};

    fn shell() -> Shell {
        Shell::new(scope_setter::set(std::env::current_dir().unwrap().as_path()).unwrap())
    }

    #[test]
    fn present_matches_function_pointer() {
        let present: PresentPipelineResult = present;

        let _ = present;
    }

    #[test]
    fn present_with_delegates_to_resolver_and_provider() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);

        fn resolve(
            shell: &Shell,
            value: PipelineValue,
            provide: Provide,
        ) -> Result<(), PipelineResultPresentError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 0);
            assert!(shell.filesystem_scope().path().is_dir());
            assert!(matches!(value, PipelineValue::Value(_)));
            provide("presented\n")?;
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 2);
            Ok(())
        }

        fn provide(rendered: &str) -> Result<(), PipelineResultPresentError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 1);
            assert_eq!(rendered, "presented\n");
            Ok(())
        }

        let shell = shell();
        let result = present_with(
            resolve,
            provide,
            &shell,
            PipelineValue::Value(evo_shell_engine::ProjectedValue::index(1)),
        );

        assert!(result.is_ok());
        assert_eq!(ORDER.load(Ordering::SeqCst), 3);
    }

    use crate::{ExecutionResult, TokenStream, executor, parser, tokenizer};
    use std::cell::RefCell;
    use std::fs;
    use std::path::PathBuf;
    use std::time::SystemTime;

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
                "evo_shell_pres_agent_{name}_{}_{}",
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

    thread_local! {
        static CAPTURED_OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    fn capture_rendered(rendered: &str) -> Result<(), PipelineResultPresentError> {
        CAPTURED_OUTPUT.with(|buffer| {
            buffer.borrow_mut().extend_from_slice(rendered.as_bytes());
        });

        Ok(())
    }

    fn captured_rendered_output() -> String {
        CAPTURED_OUTPUT.with(|buffer| {
            String::from_utf8(buffer.borrow().clone()).expect("rendered output should be utf8")
        })
    }

    fn clear_captured_output() {
        CAPTURED_OUTPUT.with(|buffer| {
            buffer.borrow_mut().clear();
        });
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_equals_output() {
        let directory = TestDirectory::new("pipeline_filter_equals");
        fs::write(directory.path.join("alpha.txt"), "alpha").unwrap();
        fs::write(directory.path.join("beta.txt"), "beta").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new(
            r#"iter |> filter name equals "alpha.txt" |> take 1 |> select name |> to-value"#,
        );
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        present_with(
            crate::resolvers::pipeline_result_presenter::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "alpha.txt\n");
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_not_equals_output() {
        let directory = TestDirectory::new("pipeline_filter_not_equals");
        fs::write(directory.path.join("alpha.txt"), "alpha").unwrap();
        fs::write(directory.path.join("beta.txt"), "beta").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new(
            r#"iter |> filter name not-equals "alpha.txt" |> select name |> to-values"#,
        );
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        present_with(
            crate::resolvers::pipeline_result_presenter::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "beta.txt\n");
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_size_decimal_output() {
        let directory = TestDirectory::new("pipeline_filter_size_decimal");
        fs::write(directory.path.join("exact.bin"), vec![b'a'; 10_000]).unwrap();
        fs::write(directory.path.join("larger.bin"), vec![b'b'; 10_240]).unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream =
            TokenStream::new(r#"iter |> filter size equals 10kb |> select name |> to-values"#);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        present_with(
            crate::resolvers::pipeline_result_presenter::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "exact.bin\n");
        assert!(!rendered.contains("larger.bin"));
    }
}
