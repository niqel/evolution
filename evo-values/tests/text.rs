use evo_values::definitions::text::{
    Contains, EndsWith, Find, IsEmpty, Len, StartsWith, Substring, Trim,
};
use evo_values::text::{
    CONTAINS, ENDS_WITH, FIND, IS_EMPTY, LEN, STARTS_WITH, SUBSTRING, TRIM, contains, ends_with,
    find, is_empty, len, starts_with, substring, trim,
};
use evo_values::{TextLength, TextOperationFailure, TextPosition};

// ============================================================================
// 1. Len
// ============================================================================

#[test]
fn test_len_empty() {
    let result: TextLength = len("");
    assert_eq!(result, TextLength(0));
}

#[test]
fn test_len_ascii() {
    let result: TextLength = len("Gustavo");
    assert_eq!(result, TextLength(7));
}

#[test]
fn test_len_unicode_multibyte() {
    let result: TextLength = len("México");
    assert_eq!(result, TextLength(6));
    assert_ne!(result.0, "México".len()); // 6 chars vs 7 UTF-8 bytes
}

#[test]
fn test_len_function_pointer() {
    let op: Len = LEN;
    assert_eq!(op("México"), TextLength(6));
}

// ============================================================================
// 2. IsEmpty
// ============================================================================

#[test]
fn test_is_empty_empty() {
    assert!(is_empty(""));
}

#[test]
fn test_is_empty_whitespace_not_empty() {
    assert!(!is_empty(" "));
    assert!(!is_empty("\n"));
    assert!(!is_empty("\t"));
    assert!(!is_empty("\r\n"));
}

#[test]
fn test_is_empty_ascii() {
    assert!(!is_empty("abc"));
}

#[test]
fn test_is_empty_unicode() {
    assert!(!is_empty("ñ"));
    assert!(!is_empty("🦀"));
}

#[test]
fn test_is_empty_function_pointer() {
    let op: IsEmpty = IS_EMPTY;
    assert!(op(""));
    assert!(!op(" "));
    assert!(!op("abc"));
}

// ============================================================================
// 3. Substring
// ============================================================================

#[test]
fn test_substring_ascii() {
    assert_eq!(
        substring("Gustavo", TextPosition(0), TextLength(3)),
        Ok("Gus")
    );
    assert_eq!(
        substring("Gustavo", TextPosition(3), TextLength(4)),
        Ok("tavo")
    );
}

#[test]
fn test_substring_unicode_multibyte() {
    assert_eq!(
        substring("México", TextPosition(1), TextLength(3)),
        Ok("éxi")
    );
    assert_eq!(
        substring("🦀 Rust 🚀", TextPosition(2), TextLength(4)),
        Ok("Rust")
    );
}

#[test]
fn test_substring_entire_text() {
    assert_eq!(
        substring("México", TextPosition(0), TextLength(6)),
        Ok("México")
    );
}

#[test]
fn test_substring_zero_length_beginning() {
    assert_eq!(substring("Gustavo", TextPosition(0), TextLength(0)), Ok(""));
}

#[test]
fn test_substring_zero_length_end() {
    assert_eq!(substring("Gustavo", TextPosition(7), TextLength(0)), Ok(""));
}

#[test]
fn test_substring_start_out_of_bounds() {
    assert_eq!(
        substring("Gustavo", TextPosition(8), TextLength(0)),
        Err(TextOperationFailure::OutOfBounds)
    );
    assert_eq!(
        substring("Gustavo", TextPosition(8), TextLength(1)),
        Err(TextOperationFailure::OutOfBounds)
    );
}

#[test]
fn test_substring_length_out_of_bounds() {
    assert_eq!(
        substring("Gustavo", TextPosition(5), TextLength(3)),
        Err(TextOperationFailure::OutOfBounds)
    );
}

#[test]
fn test_substring_overflow_safe() {
    assert_eq!(
        substring("Gustavo", TextPosition(usize::MAX), TextLength(1)),
        Err(TextOperationFailure::OutOfBounds)
    );
    assert_eq!(
        substring("Gustavo", TextPosition(1), TextLength(usize::MAX)),
        Err(TextOperationFailure::OutOfBounds)
    );
}

#[test]
fn test_substring_empty_text_zero_length() {
    assert_eq!(substring("", TextPosition(0), TextLength(0)), Ok(""));
}

#[test]
fn test_substring_empty_text_invalid() {
    assert_eq!(
        substring("", TextPosition(1), TextLength(0)),
        Err(TextOperationFailure::OutOfBounds)
    );
    assert_eq!(
        substring("", TextPosition(0), TextLength(1)),
        Err(TextOperationFailure::OutOfBounds)
    );
}

#[test]
fn test_substring_function_pointer() {
    let op: Substring = SUBSTRING;
    assert_eq!(op("México", TextPosition(0), TextLength(3)), Ok("Méx"));
}

// ============================================================================
// 4. Contains
// ============================================================================

#[test]
fn test_contains_match() {
    assert!(contains("Hello World", "World"));
    assert!(contains("Hello World", "Hello"));
    assert!(contains("Hello World", "lo W"));
}

#[test]
fn test_contains_no_match() {
    assert!(!contains("Hello World", "world"));
    assert!(!contains("Hello World", "Foo"));
}

#[test]
fn test_contains_empty_pattern() {
    assert!(contains("Hello World", ""));
    assert!(contains("", ""));
}

#[test]
fn test_contains_case_sensitive() {
    assert!(!contains("México", "méxico"));
    assert!(contains("México", "México"));
}

#[test]
fn test_contains_unicode() {
    assert!(contains("¡Viva México!", "México"));
    assert!(contains("🦀 Rust", "🦀"));
}

#[test]
fn test_contains_function_pointer() {
    let op: Contains = CONTAINS;
    assert!(op("Hello World", "World"));
    assert!(!op("Hello World", "world"));
}

// ============================================================================
// 5. StartsWith
// ============================================================================

#[test]
fn test_starts_with_match() {
    assert!(starts_with("Hello World", "Hello"));
    assert!(starts_with("Hello World", "H"));
}

#[test]
fn test_starts_with_no_match() {
    assert!(!starts_with("Hello World", "World"));
    assert!(!starts_with("Hello World", "hello"));
}

#[test]
fn test_starts_with_empty_prefix() {
    assert!(starts_with("Hello World", ""));
    assert!(starts_with("", ""));
}

#[test]
fn test_starts_with_case_sensitive() {
    assert!(!starts_with("México", "méx"));
    assert!(starts_with("México", "Méx"));
}

#[test]
fn test_starts_with_unicode() {
    assert!(starts_with("¡Hola!", "¡Hola"));
    assert!(starts_with("🦀 Rust", "🦀"));
}

#[test]
fn test_starts_with_function_pointer() {
    let op: StartsWith = STARTS_WITH;
    assert!(op("Hello World", "Hello"));
    assert!(!op("Hello World", "world"));
}

// ============================================================================
// 6. EndsWith
// ============================================================================

#[test]
fn test_ends_with_match() {
    assert!(ends_with("Hello World", "World"));
    assert!(ends_with("Hello World", "d"));
}

#[test]
fn test_ends_with_no_match() {
    assert!(!ends_with("Hello World", "Hello"));
    assert!(!ends_with("Hello World", "world"));
}

#[test]
fn test_ends_with_empty_suffix() {
    assert!(ends_with("Hello World", ""));
    assert!(ends_with("", ""));
}

#[test]
fn test_ends_with_case_sensitive() {
    assert!(!ends_with("México", "XICO"));
    assert!(ends_with("México", "ico"));
}

#[test]
fn test_ends_with_unicode() {
    assert!(ends_with("¡Viva México!", "México!"));
    assert!(ends_with("Rust 🦀", "🦀"));
}

#[test]
fn test_ends_with_function_pointer() {
    let op: EndsWith = ENDS_WITH;
    assert!(op("Hello World", "World"));
    assert!(!op("Hello World", "Hello"));
}

// ============================================================================
// 7. Find
// ============================================================================

#[test]
fn test_find_match_at_start() {
    assert_eq!(find("Hello World", "Hello"), Some(TextPosition(0)));
}

#[test]
fn test_find_match_in_middle() {
    assert_eq!(find("Hello World", "World"), Some(TextPosition(6)));
}

#[test]
fn test_find_first_of_multiple_matches() {
    assert_eq!(find("ab ab ab", "ab"), Some(TextPosition(0)));
    assert_eq!(find("banana", "an"), Some(TextPosition(1)));
}

#[test]
fn test_find_no_match() {
    assert_eq!(find("Hello World", "404"), None);
}

#[test]
fn test_find_empty_pattern() {
    assert_eq!(find("Hello World", ""), Some(TextPosition(0)));
    assert_eq!(find("", ""), Some(TextPosition(0)));
}

#[test]
fn test_find_case_sensitive() {
    assert_eq!(find("Hello World", "world"), None);
    assert_eq!(find("Hello World", "World"), Some(TextPosition(6)));
}

#[test]
fn test_find_unicode_multibyte_before_match_mandatory_case() {
    // Mandatory case: find("México", "x") -> Some(TextPosition(2))
    // 'M' = 1 byte, 'é' = 2 bytes (bytes 1..3).
    // byte offset of 'x' is 3, but Unicode scalar position is 2.
    assert_eq!(find("México", "x"), Some(TextPosition(2)));
}

#[test]
fn test_find_unicode_distinguishes_byte_offset_from_scalar_position() {
    // "🦀🦀🦀abc": each emoji is 4 bytes (total 12 bytes before 'a').
    // byte offset = 12, scalar position = 3.
    let text = "🦀🦀🦀abc";
    assert_eq!(find(text, "a"), Some(TextPosition(3)));
    assert_ne!(find(text, "a"), Some(TextPosition(12)));
}

#[test]
fn test_find_function_pointer() {
    let op: Find = FIND;
    assert_eq!(op("México", "x"), Some(TextPosition(2)));
    assert_eq!(op("México", "notfound"), None);
}

// ============================================================================
// 8. Trim
// ============================================================================

#[test]
fn test_trim_empty() {
    assert_eq!(trim(""), "");
}

#[test]
fn test_trim_no_whitespace() {
    assert_eq!(trim("Evo"), "Evo");
}

#[test]
fn test_trim_leading() {
    assert_eq!(trim("   Evo"), "Evo");
}

#[test]
fn test_trim_trailing() {
    assert_eq!(trim("Evo   "), "Evo");
}

#[test]
fn test_trim_both_ends() {
    assert_eq!(trim("  Evo  "), "Evo");
    assert_eq!(trim("\n\tEvo \r"), "Evo");
}

#[test]
fn test_trim_interior_preserved() {
    assert_eq!(trim("  Evo   Language  "), "Evo   Language");
}

#[test]
fn test_trim_all_whitespace() {
    assert_eq!(trim("   "), "");
    assert_eq!(trim("\t\n\r "), "");
}

#[test]
fn test_trim_unicode_whitespace() {
    assert_eq!(trim("\u{00A0}Evo\u{3000}"), "Evo");
}

#[test]
fn test_trim_returns_borrowed_slice() {
    let input: &str = "  hello world  ";
    let output: &str = trim(input);
    assert_eq!(output, "hello world");
    // Prove it's a subslice of input (&str)
    assert!(output.as_ptr() >= input.as_ptr());
    assert!(output.as_ptr() <= unsafe { input.as_ptr().add(input.len()) });
}

#[test]
fn test_trim_function_pointer() {
    let op: Trim = TRIM;
    assert_eq!(op("  Evo  "), "Evo");
}
