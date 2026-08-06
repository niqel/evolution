use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};
use crate::definitions::use_cases::execute_pipeline::ExecutePipeline;
use crate::definitions::use_cases::terminal_clearer::TerminalClearer;
use crate::resolvers::execution;
use crate::terminal_clearer;

pub fn execute(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
    let clear: TerminalClearer = terminal_clearer::clear;
    let execute_pipeline: ExecutePipeline = crate::pipeline_executor::execute;

    execute_with(shell, command, clear, execute_pipeline)
}

pub(crate) fn execute_with(
    shell: &mut Shell,
    command: Command<'_>,
    clear: TerminalClearer,
    execute_pipeline: ExecutePipeline,
) -> Result<ExecutionResult, ExecuteError> {
    execution::resolve_with(shell, command, clear, execute_pipeline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenStream;
    use crate::definitions::domain::value_objects::pipeline::{Pipeline, PipelineOperation};
    use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;
    use crate::definitions::use_cases::execute::Execute;
    use crate::definitions::use_cases::execute_pipeline::PipelineExecutionError;
    use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
    use crate::parser;
    use crate::tokenizer;
    use evo_shell_engine::{ProjectedValue, scope_setter};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
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
                "evo_shell_exec_agent_{name}_{}_{}",
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

    #[test]
    fn executor_matches_execute_function_pointer() {
        let directory = TestDirectory::new("execute_pointer");
        let input = format!("scope-fs \"{}\"", directory.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let mut shell = shell_from_directory(&directory);
        let execute_fn: Execute = execute;

        let result = execute_fn(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), directory.path.as_path());
    }

    #[test]
    fn executor_delegates_pipeline_execution_and_returns_typed_pipeline_result() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        fn clear() -> Result<(), TerminalClearError> {
            Ok(())
        }

        fn execute_pipeline(
            shell: &Shell,
            pipeline: Pipeline,
        ) -> Result<PipelineValue, PipelineExecutionError> {
            CALLS.fetch_add(1, Ordering::SeqCst);
            assert!(shell.filesystem_scope().path().is_dir());
            assert_eq!(pipeline.operations(), &[PipelineOperation::Iter]);

            Ok(PipelineValue::Value(ProjectedValue::name("delegated.txt")))
        }

        let directory = TestDirectory::new("executor_pipeline_delegate");
        let mut shell = shell_from_directory(&directory);
        let command = Command::Pipeline(Pipeline::new(vec![PipelineOperation::Iter]));

        let result = execute_with(&mut shell, command, clear, execute_pipeline).unwrap();

        assert_eq!(CALLS.load(Ordering::SeqCst), 1);
        let ExecutionResult::Pipeline(PipelineValue::Value(value)) = result else {
            panic!("expected delegated pipeline value");
        };

        assert_eq!(value, ProjectedValue::name("delegated.txt"));
    }

    #[test]
    fn executor_propagates_pipeline_execution_error() {
        fn clear() -> Result<(), TerminalClearError> {
            Ok(())
        }

        fn execute_pipeline(
            _shell: &Shell,
            _pipeline: Pipeline,
        ) -> Result<PipelineValue, PipelineExecutionError> {
            Err(PipelineExecutionError::EmptyPipeline)
        }

        let directory = TestDirectory::new("executor_pipeline_error");
        let mut shell = shell_from_directory(&directory);
        let command = Command::Pipeline(Pipeline::new(vec![PipelineOperation::Iter]));

        let result = execute_with(&mut shell, command, clear, execute_pipeline);

        assert!(matches!(
            result,
            Err(ExecuteError::Pipeline(
                PipelineExecutionError::EmptyPipeline
            ))
        ));
    }
}
