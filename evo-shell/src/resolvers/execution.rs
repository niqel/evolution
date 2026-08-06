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
        Command::Clear(mode) => {
            clear(mode).map_err(ExecuteError::TerminalClear)?;
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
