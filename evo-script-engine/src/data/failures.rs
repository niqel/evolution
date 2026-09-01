#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexicalFailure {
    UnrecognizedCharacter(char),
    InvalidIdentifier,
    MalformedNumericLiteral,
    UnterminatedStringLiteral,
    InvalidStringEscape(char),
    PhysicalNewlineInStringLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxFailure {
    MalformedDeclaration,
    MalformedExpression,
    InvalidImportPlacement,
    MissingFinalReturn,
    InvalidReturnPlacement,
    MissingPublicFunction,
    MultiplePublicFunctions,
    EmptyEnum,
    InvalidOperationStatement,
    InvalidThisUsage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexical_failure_inventory_has_exactly_6_variants_and_payloads() {
        let all_lexical: [LexicalFailure; 6] = [
            LexicalFailure::UnrecognizedCharacter('@'),
            LexicalFailure::InvalidIdentifier,
            LexicalFailure::MalformedNumericLiteral,
            LexicalFailure::UnterminatedStringLiteral,
            LexicalFailure::InvalidStringEscape('x'),
            LexicalFailure::PhysicalNewlineInStringLiteral,
        ];

        assert_eq!(all_lexical.len(), 6);

        // Verify payloads are preserved
        assert_eq!(all_lexical[0], LexicalFailure::UnrecognizedCharacter('@'));
        assert_ne!(all_lexical[0], LexicalFailure::UnrecognizedCharacter('#'));
        assert_eq!(all_lexical[4], LexicalFailure::InvalidStringEscape('x'));
        assert_ne!(all_lexical[4], LexicalFailure::InvalidStringEscape('u'));

        // Copy and equality semantics
        let copy = all_lexical[0];
        assert_eq!(all_lexical[0], copy);
    }

    #[test]
    fn syntax_failure_inventory_has_exactly_10_variants() {
        let all_syntax: [SyntaxFailure; 10] = [
            SyntaxFailure::MalformedDeclaration,
            SyntaxFailure::MalformedExpression,
            SyntaxFailure::InvalidImportPlacement,
            SyntaxFailure::MissingFinalReturn,
            SyntaxFailure::InvalidReturnPlacement,
            SyntaxFailure::MissingPublicFunction,
            SyntaxFailure::MultiplePublicFunctions,
            SyntaxFailure::EmptyEnum,
            SyntaxFailure::InvalidOperationStatement,
            SyntaxFailure::InvalidThisUsage,
        ];

        assert_eq!(all_syntax.len(), 10);
        assert_eq!(all_syntax[0], SyntaxFailure::MalformedDeclaration);
        assert_eq!(all_syntax[9], SyntaxFailure::InvalidThisUsage);

        // Copy and equality semantics
        let copy = all_syntax[0];
        assert_eq!(all_syntax[0], copy);
    }
}
