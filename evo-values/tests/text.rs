extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use evo_values::definitions::text::{
    Concat, Contains, EndsWith, Find, IsEmpty, Join, Len, ReceiveTextSegment, Replace, Split,
    StartsWith, Substring, Trim,
};
use evo_values::text::{
    CONCAT, CONTAINS, ENDS_WITH, FIND, IS_EMPTY, JOIN, LEN, REPLACE, STARTS_WITH, SUBSTRING, TRIM,
    concat, contains, ends_with, find, is_empty, join, len, replace, split, starts_with, substring,
    trim,
};
use evo_values::{ProductionControl, TextLength, TextOperationFailure, TextPosition};

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

// ============================================================================
// 9. Concat
// ============================================================================

#[test]
fn test_concat_zero_elements() {
    let res = concat(&[]);
    assert_eq!(res, "");
    assert!(matches!(res, Cow::Borrowed(_)));
}

#[test]
fn test_concat_single_element_preserves_borrow() {
    let input = "Evo";
    let res = concat(&[input]);
    assert_eq!(res, "Evo");
    match res {
        Cow::Borrowed(slice) => {
            assert_eq!(slice.as_ptr(), input.as_ptr());
        }
        Cow::Owned(_) => panic!("expected Cow::Borrowed without allocation"),
    }
}

#[test]
fn test_concat_multiple_elements() {
    let res = concat(&["Evo", "-", "Values"]);
    assert_eq!(res, "Evo-Values");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_concat_empty_elements_preserved() {
    let res = concat(&["a", "", "b"]);
    assert_eq!(res, "ab");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_concat_unicode() {
    let res = concat(&["Mé", "xi", "co"]);
    assert_eq!(res, "México");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_concat_function_pointer() {
    let op: Concat = CONCAT;
    let res = op(&["Hello", " ", "World"]);
    assert_eq!(res, "Hello World");
    assert!(matches!(res, Cow::Owned(_)));
}

// ============================================================================
// 10. Join
// ============================================================================

#[test]
fn test_join_zero_elements() {
    let res = join(&[], ",");
    assert_eq!(res, "");
    assert!(matches!(res, Cow::Borrowed(_)));
}

#[test]
fn test_join_single_element_preserves_borrow() {
    let input = "Evo";
    let res = join(&[input], ",");
    assert_eq!(res, "Evo");
    match res {
        Cow::Borrowed(slice) => {
            assert_eq!(slice.as_ptr(), input.as_ptr());
        }
        Cow::Owned(_) => panic!("expected Cow::Borrowed without allocation"),
    }
}

#[test]
fn test_join_two_elements() {
    let res = join(&["a", "b"], ",");
    assert_eq!(res, "a,b");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_join_multiple_elements() {
    let res = join(&["a", "b", "c"], ",");
    assert_eq!(res, "a,b,c");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_join_empty_elements() {
    let res1 = join(&["a", "", "b"], ",");
    assert_eq!(res1, "a,,b");
    assert!(matches!(res1, Cow::Owned(_)));

    let res2 = join(&["", "a", ""], ",");
    assert_eq!(res2, ",a,");
    assert!(matches!(res2, Cow::Owned(_)));
}

#[test]
fn test_join_empty_separator() {
    let res = join(&["a", "b"], "");
    assert_eq!(res, "ab");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_join_multi_scalar_separator() {
    let res1 = join(&["a", "b"], "---");
    assert_eq!(res1, "a---b");

    let res2 = join(&["a", "b"], "🦀");
    assert_eq!(res2, "a🦀b");
}

#[test]
fn test_join_unicode() {
    let res = join(&["México", "lindo"], " 🇲🇽 ");
    assert_eq!(res, "México 🇲🇽 lindo");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_join_function_pointer() {
    let op: Join = JOIN;
    let res = op(&["x", "y", "z"], "-");
    assert_eq!(res, "x-y-z");
    assert!(matches!(res, Cow::Owned(_)));
}

// ============================================================================
// 11. Replace
// ============================================================================

#[test]
fn test_replace_one_match() {
    let res = replace("hello world", "world", "rust").unwrap();
    assert_eq!(res, "hello rust");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_multiple_matches() {
    let res = replace("one two one", "one", "1").unwrap();
    assert_eq!(res, "1 two 1");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_no_match_preserves_borrow() {
    let input = "hello world";
    let res = replace(input, "abc", "123").unwrap();
    assert_eq!(res, "hello world");
    match res {
        Cow::Borrowed(slice) => {
            assert_eq!(slice.as_ptr(), input.as_ptr());
        }
        Cow::Owned(_) => panic!("expected Cow::Borrowed without allocation"),
    }
}

#[test]
fn test_replace_pattern_equals_replacement_preserves_borrow() {
    let input = "hello world";
    let res = replace(input, "world", "world").unwrap();
    assert_eq!(res, "hello world");
    match res {
        Cow::Borrowed(slice) => {
            assert_eq!(slice.as_ptr(), input.as_ptr());
        }
        Cow::Owned(_) => panic!("expected Cow::Borrowed without allocation"),
    }
}

#[test]
fn test_replace_to_empty() {
    let res = replace("Gustavo", "avo", "").unwrap();
    assert_eq!(res, "Gust");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_empty_pattern_error() {
    assert_eq!(
        replace("hello", "", "world"),
        Err(TextOperationFailure::EmptyPattern)
    );
}

#[test]
fn test_replace_case_sensitive() {
    let res = replace("Case CASE case", "case", "word").unwrap();
    assert_eq!(res, "Case CASE word");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_unicode() {
    let res = replace("México lindo y querido", "México", "MÉXICO").unwrap();
    assert_eq!(res, "MÉXICO lindo y querido");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_non_overlapping() {
    let res = replace("aaaa", "aa", "x").unwrap();
    assert_eq!(res, "xx");
    assert!(matches!(res, Cow::Owned(_)));
}

#[test]
fn test_replace_function_pointer() {
    let op: Replace = REPLACE;
    let res = op("one two one", "one", "1").unwrap();
    assert_eq!(res, "1 two 1");
    assert!(matches!(res, Cow::Owned(_)));
}

// ============================================================================
// 12. Split
// ============================================================================

#[derive(Default, Debug, PartialEq, Eq)]
struct TestCollectorState {
    segments: Vec<String>,
    call_count: usize,
}

fn test_collect_all(state: &mut TestCollectorState, segment: &str) -> ProductionControl {
    state.call_count += 1;
    state.segments.push(String::from(segment));
    ProductionControl::Continue
}

#[test]
fn test_split_normal() {
    let mut state = TestCollectorState::default();
    let res = split("a,b,c", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["a", "b", "c"]);
    assert_eq!(state.call_count, 3);
}

#[test]
fn test_split_separator_not_found() {
    let mut state = TestCollectorState::default();
    let res = split("abc", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["abc"]);
    assert_eq!(state.call_count, 1);
}

#[test]
fn test_split_empty_text() {
    let mut state = TestCollectorState::default();
    let res = split("", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, [""]);
    assert_eq!(state.call_count, 1);
}

#[test]
fn test_split_initial_empty_segment() {
    let mut state = TestCollectorState::default();
    let res = split(",a", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["", "a"]);
    assert_eq!(state.call_count, 2);
}

#[test]
fn test_split_trailing_empty_segment() {
    let mut state1 = TestCollectorState::default();
    let res1 = split("a,", ",", &mut state1, test_collect_all);
    assert_eq!(res1, Ok(()));
    assert_eq!(state1.segments, ["a", ""]);
    assert_eq!(state1.call_count, 2);

    let mut state2 = TestCollectorState::default();
    let res2 = split(",a,", ",", &mut state2, test_collect_all);
    assert_eq!(res2, Ok(()));
    assert_eq!(state2.segments, ["", "a", ""]);
    assert_eq!(state2.call_count, 3);
}

#[test]
fn test_split_consecutive_separators() {
    let mut state1 = TestCollectorState::default();
    let res1 = split("a,,b", ",", &mut state1, test_collect_all);
    assert_eq!(res1, Ok(()));
    assert_eq!(state1.segments, ["a", "", "b"]);
    assert_eq!(state1.call_count, 3);

    let mut state2 = TestCollectorState::default();
    let res2 = split("a----b", "--", &mut state2, test_collect_all);
    assert_eq!(res2, Ok(()));
    assert_eq!(state2.segments, ["a", "", "b"]);
    assert_eq!(state2.call_count, 3);
}

#[test]
fn test_split_multiple_empty_segments() {
    let mut state = TestCollectorState::default();
    let res = split(",,,", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["", "", "", ""]);
    assert_eq!(state.call_count, 4);
}

#[test]
fn test_split_multi_scalar_separator() {
    let mut state = TestCollectorState::default();
    let res = split("a--b--c", "--", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["a", "b", "c"]);
    assert_eq!(state.call_count, 3);
}

#[test]
fn test_split_unicode_separator() {
    let mut state = TestCollectorState::default();
    let res = split("a🦀b🦀c", "🦀", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["a", "b", "c"]);
    assert_eq!(state.call_count, 3);
}

#[test]
fn test_split_unicode_content() {
    let mut state = TestCollectorState::default();
    let res = split("árbol,niño,café", ",", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["árbol", "niño", "café"]);
    assert_eq!(state.call_count, 3);
}

#[test]
fn test_split_empty_separator_mandatory() {
    let mut state = TestCollectorState::default();
    let res = split("abc", "", &mut state, test_collect_all);
    assert_eq!(res, Err(TextOperationFailure::EmptySeparator));
    assert_eq!(state.call_count, 0);
    assert!(state.segments.is_empty());
}

#[test]
fn test_split_continue_until_exhaustion() {
    let mut state = TestCollectorState::default();
    let res = split("1;2;3;4;5", ";", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["1", "2", "3", "4", "5"]);
    assert_eq!(state.call_count, 5);
}

#[test]
fn test_split_stop_after_first() {
    struct StopFirstState {
        segments: Vec<String>,
        calls: usize,
    }

    fn receiver(state: &mut StopFirstState, segment: &str) -> ProductionControl {
        state.calls += 1;
        state.segments.push(String::from(segment));
        ProductionControl::Stop
    }

    let mut state = StopFirstState {
        segments: Vec::new(),
        calls: 0,
    };
    let res = split("a,b,c", ",", &mut state, receiver);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["a"]);
    assert_eq!(state.calls, 1);
}

#[test]
fn test_split_stop_after_multiple_mandatory() {
    struct StopAtBState {
        segments: Vec<String>,
        calls: usize,
    }

    fn receiver(state: &mut StopAtBState, segment: &str) -> ProductionControl {
        state.calls += 1;
        state.segments.push(String::from(segment));
        if segment == "b" {
            ProductionControl::Stop
        } else {
            ProductionControl::Continue
        }
    }

    let mut state = StopAtBState {
        segments: Vec::new(),
        calls: 0,
    };
    let res = split("a,b,c,d", ",", &mut state, receiver);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["a", "b"]);
    assert_eq!(state.calls, 2);
}

#[test]
fn test_split_stores_borrowed_segments_mandatory() {
    struct BorrowState<'text> {
        segments: Vec<&'text str>,
    }

    fn collect_borrowed<'text>(
        state: &mut BorrowState<'text>,
        segment: &'text str,
    ) -> ProductionControl {
        state.segments.push(segment);
        ProductionControl::Continue
    }

    let source = String::from("alpha,beta,gamma");
    let mut state = BorrowState {
        segments: Vec::new(),
    };

    let res = split(source.as_str(), ",", &mut state, collect_borrowed);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["alpha", "beta", "gamma"]);
}

#[test]
fn test_split_function_pointer_split() {
    let operation: Split<TestCollectorState> = split::<TestCollectorState>;
    let mut state = TestCollectorState::default();
    let res = operation("hello world", " ", &mut state, test_collect_all);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["hello", "world"]);
    assert_eq!(state.call_count, 2);

    struct BorrowState<'text> {
        segments: Vec<&'text str>,
    }

    fn collect_borrowed<'text>(
        state: &mut BorrowState<'text>,
        segment: &'text str,
    ) -> ProductionControl {
        state.segments.push(segment);
        ProductionControl::Continue
    }

    let borrow_op: Split<'_, BorrowState<'_>> = split::<BorrowState<'_>>;
    let source = String::from("one,two,three");
    let mut borrow_state = BorrowState {
        segments: Vec::new(),
    };
    let res = borrow_op(source.as_str(), ",", &mut borrow_state, collect_borrowed);
    assert_eq!(res, Ok(()));
    assert_eq!(borrow_state.segments, ["one", "two", "three"]);
}

#[test]
fn test_split_function_pointer_receive_text_segment() {
    let receiver: ReceiveTextSegment<TestCollectorState> = test_collect_all;
    let mut state = TestCollectorState::default();
    let res = split("x:y:z", ":", &mut state, receiver);
    assert_eq!(res, Ok(()));
    assert_eq!(state.segments, ["x", "y", "z"]);
    assert_eq!(state.call_count, 3);

    struct BorrowState<'text> {
        segments: Vec<&'text str>,
    }

    fn collect_borrowed<'text>(
        state: &mut BorrowState<'text>,
        segment: &'text str,
    ) -> ProductionControl {
        state.segments.push(segment);
        ProductionControl::Continue
    }

    let borrow_receiver: ReceiveTextSegment<'_, BorrowState<'_>> = collect_borrowed;
    let source = String::from("m:n");
    let mut borrow_state = BorrowState {
        segments: Vec::new(),
    };
    let res = split(source.as_str(), ":", &mut borrow_state, borrow_receiver);
    assert_eq!(res, Ok(()));
    assert_eq!(borrow_state.segments, ["m", "n"]);
}

#[test]
fn test_split_borrow_invariants() {
    let source = "alpha,beta,gamma";

    struct BorrowState<'a> {
        source: &'a str,
        all_borrowed: bool,
    }

    fn check_borrow<'a>(state: &mut BorrowState<'a>, segment: &str) -> ProductionControl {
        let seg_start = segment.as_ptr() as usize;
        let seg_end = seg_start + segment.len();
        let src_start = state.source.as_ptr() as usize;
        let src_end = src_start + state.source.len();

        if seg_start < src_start || seg_end > src_end {
            state.all_borrowed = false;
        }
        ProductionControl::Continue
    }

    let mut state = BorrowState {
        source,
        all_borrowed: true,
    };
    let res = split(source, ",", &mut state, check_borrow);
    assert_eq!(res, Ok(()));
    assert!(state.all_borrowed);
}

#[test]
fn test_split_custom_consumer_state() {
    #[derive(Default)]
    struct CustomConsumerState {
        total_segment_len: usize,
        seen_empty: bool,
    }

    fn analyze(state: &mut CustomConsumerState, segment: &str) -> ProductionControl {
        state.total_segment_len += segment.len();
        if segment.is_empty() {
            state.seen_empty = true;
        }
        ProductionControl::Continue
    }

    let mut state = CustomConsumerState::default();
    let res = split("apple,,banana", ",", &mut state, analyze);
    assert_eq!(res, Ok(()));
    assert_eq!(state.total_segment_len, 5 + 0 + 6);
    assert!(state.seen_empty);
}
