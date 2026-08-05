use crate::definitions::providers::welcome_presenter::Provide;
use crate::definitions::resolvers::welcome_presenter::Resolve;
use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;
use crate::providers::welcome_presenter as provider;
use crate::resolvers::welcome_presenter as resolver;

pub fn present() -> Result<(), WelcomePresenterError> {
    let resolve: Resolve = resolver::resolve;
    let provide: Provide = provider::provide;

    present_with(resolve, provide)
}

pub(crate) fn present_with(
    resolve: Resolve,
    provide: Provide,
) -> Result<(), WelcomePresenterError> {
    resolve(provide)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::definitions::providers::welcome_presenter::Provide;
    use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;
    use crate::presentation_style::{FILE_STYLE, LOCATION_STYLE, PRIMARY_STYLE, RESET};

    use super::{present, present_with};

    #[test]
    fn present_matches_welcome_presenter_function_pointer() {
        let present: crate::WelcomePresenter = present;

        let _ = present;
    }

    #[test]
    fn present_with_delegates_to_resolver_and_provider() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);

        fn resolve(provide: Provide) -> Result<(), WelcomePresenterError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 0);
            provide(env!("CARGO_PKG_VERSION"))?;
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 2);
            Ok(())
        }

        fn provide(version: &str) -> Result<(), WelcomePresenterError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 1);
            assert_eq!(version, env!("CARGO_PKG_VERSION"));
            Ok(())
        }

        let result = present_with(resolve, provide);

        assert!(result.is_ok());
        assert_eq!(ORDER.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn resolver_uses_package_version() {
        static CAPTURED: AtomicUsize = AtomicUsize::new(0);

        fn provide(version: &str) -> Result<(), WelcomePresenterError> {
            assert_eq!(version, env!("CARGO_PKG_VERSION"));
            CAPTURED.store(1, Ordering::SeqCst);
            Ok(())
        }

        let result = crate::resolvers::welcome_presenter::resolve(provide);

        assert!(result.is_ok());
        assert_eq!(CAPTURED.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn provider_writes_expected_welcome_output() {
        let mut output = Vec::new();

        crate::providers::welcome_presenter::provide_to(&mut output, "0.1.0").unwrap();

        let expected = format!(
            "{PRIMARY_STYLE}CatarinaSoft{RESET}\n\
             {LOCATION_STYLE}evo-shell{RESET} {FILE_STYLE}0.1.0{RESET}\n\
             {FILE_STYLE}evo-shell is a life :){RESET}\n\n"
        );

        assert_eq!(String::from_utf8(output).unwrap(), expected);
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
        let result = crate::providers::welcome_presenter::provide_to(&mut writer, "0.1.0");

        assert!(matches!(result, Err(WelcomePresenterError::Io(_))));
    }
}
