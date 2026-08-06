use std::path::Path;

use evo_shell_engine::{
    Enter, Iter, ProjectedValue, SetFilesystemScope, enterer, iterator, scope_setter,
};

use crate::agents::exiter;
use crate::definitions::domain::entities::command::{Command, CommandArgument};
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};
use crate::definitions::use_cases::execute_pipeline::ExecutePipeline;
use crate::definitions::use_cases::exiter::Exit;
use crate::definitions::use_cases::terminal_clearer::TerminalClearer;
use crate::terminal_clearer;

pub fn resolve(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
    let clear: TerminalClearer = terminal_clearer::clear;
    let execute_pipeline: ExecutePipeline = crate::pipeline_executor::execute;

    resolve_with(shell, command, clear, execute_pipeline)
}

pub(crate) fn resolve_with(
    shell: &mut Shell,
    command: Command<'_>,
    clear: TerminalClearer,
    execute_pipeline: ExecutePipeline,
) -> Result<ExecutionResult, ExecuteError> {
    match command {
        Command::ScopeFs(path) => {
            let set_scope: SetFilesystemScope = scope_setter::set;
            let filesystem_scope = set_scope(Path::new(path)).map_err(ExecuteError::Scope)?;
            shell.replace_filesystem_scope(filesystem_scope);
            Ok(ExecutionResult::ScopeChanged)
        }
        Command::Iter => {
            let iter: Iter = iterator::iter;
            let iteration = iter(shell.filesystem_scope()).map_err(ExecuteError::Iter)?;
            Ok(ExecutionResult::FilesystemIteration(iteration))
        }
        Command::Enter(argument) => {
            let location_buf: String;
            let location: &str = match argument {
                CommandArgument::Literal(loc) => loc,
                CommandArgument::Grouped(inner) => {
                    let result = resolve_with(shell, *inner, clear, execute_pipeline)?;
                    match result {
                        ExecutionResult::Pipeline(PipelineValue::Value(val)) => {
                            location_buf = convert_projected_value_to_location(&val)?;
                            &location_buf
                        }
                        _ => return Err(ExecuteError::IncompatibleGroupedArgument),
                    }
                }
            };
            let enter: Enter = enterer::enter;
            let filesystem_scope = enter(shell.filesystem_scope(), Path::new(location))
                .map_err(ExecuteError::Scope)?;
            shell.replace_filesystem_scope(filesystem_scope);
            Ok(ExecutionResult::ScopeChanged)
        }
        Command::CopyTo {
            sources,
            destination,
        } => {
            let dest_path_buf: std::path::PathBuf = match destination {
                CommandArgument::Literal(loc) => std::path::PathBuf::from(loc),
                CommandArgument::Grouped(inner) => {
                    let result = resolve_with(shell, *inner, clear, execute_pipeline)?;
                    match result {
                        ExecutionResult::Pipeline(PipelineValue::Value(val)) => {
                            std::path::PathBuf::from(convert_projected_value_to_location(&val)?)
                        }
                        _ => return Err(ExecuteError::IncompatibleGroupedArgument),
                    }
                }
            };

            let mut resolved_sources = Vec::new();

            for src in sources {
                match src {
                    CommandArgument::Literal(loc) => {
                        resolved_sources.push(std::path::PathBuf::from(loc));
                    }
                    CommandArgument::Grouped(inner) => {
                        let result = resolve_with(shell, *inner, clear, execute_pipeline)?;
                        match result {
                            ExecutionResult::Pipeline(PipelineValue::Value(val)) => {
                                let loc = convert_projected_value_to_location(&val)?;
                                resolved_sources.push(std::path::PathBuf::from(loc));
                            }
                            ExecutionResult::Pipeline(PipelineValue::Arguments(args)) => {
                                for val in args.items() {
                                    let loc = convert_projected_value_to_location(val)?;
                                    resolved_sources.push(std::path::PathBuf::from(loc));
                                }
                            }
                            _ => return Err(ExecuteError::IncompatibleGroupedArgument),
                        }
                    }
                }
            }

            if resolved_sources.is_empty() {
                return Err(ExecuteError::MissingSource);
            }

            let source_refs: Vec<&Path> = resolved_sources.iter().map(|p| p.as_path()).collect();
            evo_shell_engine::copier::copy(shell.filesystem_scope(), &source_refs, &dest_path_buf)
                .map_err(ExecuteError::Copy)?;

            Ok(ExecutionResult::Copied)
        }
        Command::Clear => {
            clear().map_err(ExecuteError::TerminalClear)?;
            Ok(ExecutionResult::TerminalCleared)
        }
        Command::Exit => {
            let exit: Exit = exiter::exit;
            exit();
            Ok(ExecutionResult::Exit)
        }
        Command::Pipeline(pipeline) => {
            let result = execute_pipeline(shell, pipeline).map_err(ExecuteError::Pipeline)?;
            Ok(ExecutionResult::Pipeline(result))
        }
        Command::Grouped(inner) => resolve_with(shell, *inner, clear, execute_pipeline),
    }
}

fn convert_projected_value_to_location(value: &ProjectedValue) -> Result<String, ExecuteError> {
    match value {
        ProjectedValue::Name(name) => Ok(name.to_string_lossy().into_owned()),
        _ => Err(ExecuteError::IncompatibleGroupedArgument),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::pipeline_executor;
    use crate::agents::{parser, tokenizer};
    use crate::definitions::domain::entities::token_stream::TokenStream;
    use crate::definitions::domain::value_objects::pipeline::PipelineOperationKind;
    use crate::definitions::domain::value_objects::pipeline_value::{
        PipelineValue, PipelineValueKind,
    };
    use crate::definitions::use_cases::execute_pipeline::PipelineExecutionError;
    use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
    use evo_shell_engine::{FilesystemEntryKind, IterError, iteration_advancer, scope_setter};
    use std::ffi::OsStr;
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
                "evo_shell_exec_res_{name}_{}_{}",
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
    fn scope_fs_valid_path_replaces_previous_scope() {
        let initial = TestDirectory::new("scope_initial");
        let replacement = TestDirectory::new("scope_replacement");
        let mut shell = shell_from_directory(&initial);
        let input = format!("scope-fs \"{}\"", replacement.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), replacement.path.as_path());
    }

    #[test]
    fn scope_fs_invalid_path_returns_error() {
        let initial = TestDirectory::new("scope_invalid_error");
        let mut shell = shell_from_directory(&initial);
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command);

        assert!(matches!(result, Err(ExecuteError::Scope(_))));
    }

    #[test]
    fn scope_fs_error_leaves_previous_scope_intact() {
        let initial = TestDirectory::new("previous_scope");
        let mut shell = shell_from_directory(&initial);
        let previous_path = shell.filesystem_scope().path().to_path_buf();
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command);

        assert!(result.is_err());
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn enter_existing_child_replaces_scope_and_returns_scope_changed() {
        let directory = TestDirectory::new("enter_child_execute");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        let mut shell = shell_from_directory(&directory);

        let result = resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), child.as_path());
    }

    #[test]
    fn enter_parent_replaces_scope_with_parent_path() {
        let directory = TestDirectory::new("enter_parent_execute");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        let mut shell = shell_from_directory(&directory);
        resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        let result = resolve(&mut shell, Command::Enter(CommandArgument::Literal(".."))).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(
            shell.filesystem_scope().path(),
            directory.path.canonicalize().unwrap().as_path()
        );
        assert!(
            !shell
                .filesystem_scope()
                .path()
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        );
    }

    #[test]
    fn enter_two_parents_replaces_scope_with_ancestor_path() {
        let directory = TestDirectory::new("enter_two_parents_execute");
        let child = directory.path.join("child");
        let grandchild = child.join("grandchild");
        fs::create_dir(&child).unwrap();
        fs::create_dir(&grandchild).unwrap();
        let mut shell = shell_from_directory(&directory);
        resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child/grandchild")),
        )
        .unwrap();

        let result = resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("../..")),
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(
            shell.filesystem_scope().path(),
            directory.path.canonicalize().unwrap().as_path()
        );
        assert!(
            !shell
                .filesystem_scope()
                .path()
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        );
    }

    #[test]
    fn enter_missing_location_returns_error_and_keeps_previous_scope() {
        let directory = TestDirectory::new("enter_missing_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("missing")),
        );

        assert!(matches!(result, Err(ExecuteError::Scope(_))));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn scope_fs_still_replaces_scope_after_enter_changes() {
        let initial = TestDirectory::new("scope_fs_after_enter_initial");
        let replacement = TestDirectory::new("scope_fs_after_enter_replacement");
        let mut shell = shell_from_directory(&initial);
        let input = format!("scope-fs \"{}\"", replacement.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), replacement.path.as_path());
    }

    #[test]
    fn iter_still_works_after_enter() {
        let directory = TestDirectory::new("iter_after_enter");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        fs::write(child.join("inside.txt"), "inside").unwrap();
        let mut shell = shell_from_directory(&directory);
        resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        let result = resolve(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
    }

    #[test]
    fn enter_then_iter_reads_entries_from_child_scope() {
        let directory = TestDirectory::new("enter_then_iter");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        fs::write(child.join("inside.txt"), "inside").unwrap();
        let mut shell = shell_from_directory(&directory);
        resolve(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();
        let result = resolve(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        let mut found_inside = false;

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            let entry = item.entry();
            if entry.name() == OsStr::new("inside.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_inside = true;
            }
        }

        assert!(found_inside);
    }

    #[test]
    fn execute_iter_returns_filesystem_iteration() {
        let directory = TestDirectory::new("iter_execute");
        let mut shell = shell_from_directory(&directory);

        let result = resolve(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
    }

    #[test]
    fn execute_parsed_pipeline_to_value_returns_pipeline_value() {
        let directory = TestDirectory::new("pipeline_to_value");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> take 1 |> select name |> to-value");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Value(value)) = result else {
            panic!("expected typed pipeline value");
        };

        assert_eq!(value, ProjectedValue::name("only.txt"));
    }

    #[test]
    fn execute_parsed_pipeline_to_values_returns_values() {
        let directory = TestDirectory::new("pipeline_to_values");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> select name |> to-values");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Values(values)) = result else {
            panic!("expected typed values");
        };

        assert_eq!(values.len(), 1);
        assert_eq!(values.items(), &[ProjectedValue::name("only.txt")]);
    }

    #[test]
    fn execute_parsed_pipeline_to_args_returns_arguments() {
        let directory = TestDirectory::new("pipeline_to_args");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> select name |> to-args");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Arguments(arguments)) = result else {
            panic!("expected typed arguments");
        };

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments.items(), &[ProjectedValue::name("only.txt")]);
    }

    #[test]
    fn execute_parsed_semantically_invalid_pipeline_returns_pipeline_error() {
        let directory = TestDirectory::new("pipeline_invalid_transition");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> to-value");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = resolve(&mut shell, command);

        assert!(matches!(
            result,
            Err(ExecuteError::Pipeline(
                PipelineExecutionError::InvalidTransition {
                    operation: PipelineOperationKind::ToValue,
                    state: PipelineValueKind::StructuredItems,
                }
            ))
        ));
    }

    #[test]
    fn execute_clear_returns_terminal_cleared_without_changing_scope() {
        fn clear() -> Result<(), TerminalClearError> {
            Ok(())
        }

        let directory = TestDirectory::new("clear_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = resolve_with(
            &mut shell,
            Command::Clear,
            clear,
            pipeline_executor::execute,
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::TerminalCleared));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn execute_exit_returns_exit_without_changing_scope() {
        let directory = TestDirectory::new("exit_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = resolve_with(
            &mut shell,
            Command::Exit,
            terminal_clearer::clear,
            pipeline_executor::execute,
        );

        assert!(matches!(result, Ok(ExecutionResult::Exit)));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn iter_borrows_filesystem_scope_without_replacing_it() {
        let directory = TestDirectory::new("iter_borrow");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = resolve(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn iter_can_be_consumed_lazily_with_public_advance() {
        let directory = TestDirectory::new("iter_lazy");
        fs::write(directory.path.join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path.join("images")).unwrap();
        let mut shell = shell_from_directory(&directory);

        let result = resolve(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        let mut found_file = false;
        let mut found_directory = false;

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            let entry = item.entry();
            if entry.name() == OsStr::new("report.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_file = true;
            }

            if entry.name() == OsStr::new("images") {
                assert_eq!(entry.kind(), FilesystemEntryKind::Directory);
                found_directory = true;
            }
        }

        assert!(found_file);
        assert!(found_directory);
    }

    #[test]
    fn iter_error_is_converted_to_execute_error() {
        let directory = TestDirectory::new("iter_error");
        let mut shell = shell_from_directory(&directory);
        fs::remove_dir_all(&directory.path).unwrap();

        let result = resolve(&mut shell, Command::Iter);

        assert!(matches!(result, Err(ExecuteError::Iter(_))));
    }

    #[test]
    fn advance_errors_remain_iter_error() {
        let directory = TestDirectory::new("advance_error");
        let mut shell = shell_from_directory(&directory);
        fs::write(directory.path.join("report.txt"), "report").unwrap();
        let result = resolve(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        fs::remove_dir_all(&directory.path).unwrap();

        let result = iteration_advancer::advance(&mut iteration);

        if let Err(error) = result {
            assert!(matches!(
                error,
                IterError::NextEntry(_) | IterError::MaterializeEntry(_)
            ));
        }
    }
}
