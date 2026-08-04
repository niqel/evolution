mod agents;
mod definitions;
mod resolvers;

pub use agents::{executor, parser, tokenizer};
pub use definitions::domain::entities::command::Command;
pub use definitions::domain::entities::token::Token;
pub use definitions::domain::entities::token_stream::TokenStream;
pub use definitions::use_cases::execute::{Execute, ExecuteError};
pub use definitions::use_cases::parse::{Parse, ParseError};
pub use definitions::use_cases::tokenize::{Tokenize, TokenizeError};

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::SystemTime;

    use evo_shell_engine::{FilesystemScope, SetFilesystemScope, scope_setter};

    use crate::resolvers::token;
    use crate::{
        Command, Execute, ExecuteError, Parse, ParseError, Token, TokenStream, Tokenize,
        TokenizeError, executor, parser, tokenizer,
    };

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
                "evo_shell_{name}_{}_{}",
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

    #[test]
    fn token_stream_borrows_input_and_starts_at_initial_position() {
        let input = "scope-fs \"/tmp\"";
        let stream = TokenStream::new(input);

        assert_eq!(stream.input(), input);
        assert_eq!(stream.position(), 0);
    }

    #[test]
    fn token_resolver_recognizes_scope_fs_as_word() {
        let mut stream = TokenStream::new("scope-fs \"/tmp\"");

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }

    #[test]
    fn token_resolver_recognizes_quoted_path_as_borrowed_string() {
        let input = "scope-fs \"/home/user/documents\"";
        let mut stream = TokenStream::new(input);
        token::resolve(&mut stream).unwrap();

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::String("/home/user/documents")));
        let Token::String(path) = token.unwrap() else {
            panic!("expected string token");
        };
        let expected = &input[10..30];
        assert!(std::ptr::eq(path.as_ptr(), expected.as_ptr()));
    }

    #[test]
    fn tokenization_returns_none_at_end() {
        let mut stream = TokenStream::new("scope-fs");

        assert!(token::resolve(&mut stream).unwrap().is_some());
        assert_eq!(token::resolve(&mut stream).unwrap(), None);
    }

    #[test]
    fn unterminated_quote_returns_tokenize_error() {
        let mut stream = TokenStream::new("scope-fs \"/tmp");
        token::resolve(&mut stream).unwrap();

        let result = token::resolve(&mut stream);

        assert!(matches!(result, Err(TokenizeError::UnterminatedString)));
    }

    #[test]
    fn parser_resolves_scope_fs_command() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::ScopeFs("/ruta"));
    }

    #[test]
    fn parser_rejects_missing_path() {
        let mut stream = TokenStream::new("scope-fs");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::ExpectedPath)));
    }

    #[test]
    fn parser_rejects_extra_token() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\" extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_unknown_command() {
        let mut stream = TokenStream::new("unknown-command \"/ruta\"");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnknownCommand("unknown-command"))
        ));
    }

    #[test]
    fn parser_rejects_empty_input() {
        let mut stream = TokenStream::new("");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::ExpectedCommand)));
    }

    #[test]
    fn parser_consumes_tokens_incrementally_without_token_vec() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(stream.position(), stream.input().len());
    }

    #[test]
    fn tokenizer_matches_tokenize_function_pointer() {
        let tokenize: Tokenize = tokenizer::tokenize;
        let mut stream = TokenStream::new("scope-fs");

        let token = tokenize(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }

    #[test]
    fn parser_matches_parse_function_pointer() {
        let parse: Parse = parser::parse;
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        let command = parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::ScopeFs("/ruta"));
    }

    #[test]
    fn executor_matches_execute_function_pointer() {
        let directory = TestDirectory::new("execute_pointer");
        let input = format!("scope-fs \"{}\"", directory.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let execute: Execute = executor::execute;

        let scope = execute(command).unwrap();

        assert_eq!(scope.path(), directory.path.as_path());
    }

    #[test]
    fn execution_with_valid_temporary_directory_returns_filesystem_scope() {
        let directory = TestDirectory::new("valid_execute");
        let input = format!("scope-fs \"{}\"", directory.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let scope = executor::execute(command).unwrap();

        assert_eq!(scope.path(), directory.path.as_path());
    }

    #[test]
    fn engine_error_is_converted_to_execute_error() {
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(command);

        assert!(matches!(result, Err(ExecuteError::Engine(_))));
    }

    #[test]
    fn previous_scope_can_remain_owned_by_caller_when_new_execution_fails() {
        let directory = TestDirectory::new("previous_scope");
        let set_scope: SetFilesystemScope = scope_setter::set;
        let previous_scope: FilesystemScope = set_scope(directory.path.as_path()).unwrap();
        let previous_path = previous_scope.path().to_path_buf();
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(command);

        assert!(result.is_err());
        assert_eq!(previous_scope.path(), previous_path.as_path());
    }
}
