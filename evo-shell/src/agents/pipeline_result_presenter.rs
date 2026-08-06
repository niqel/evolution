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
}
