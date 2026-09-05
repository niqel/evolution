use evo_values::boolean::{AND, NOT, OR, XOR, and, not, or, xor};
use evo_values::definitions::boolean::{And, Not, Or, Xor};

// ============================================================================
// 1. Not (UC-EV-BOOL-001)
// ============================================================================

#[test]
fn test_not_semantics() {
    assert_eq!(not(false), true);
    assert_eq!(not(true), false);
}

#[test]
fn test_not_function_pointer() {
    let op: Not = NOT;
    assert_eq!(op(false), true);
    assert_eq!(op(true), false);

    let fn_item: Not = not;
    assert_eq!(fn_item(false), true);
    assert_eq!(fn_item(true), false);
}

// ============================================================================
// 2. And (UC-EV-BOOL-002)
// ============================================================================

#[test]
fn test_and_truth_table() {
    assert_eq!(and(false, false), false);
    assert_eq!(and(false, true), false);
    assert_eq!(and(true, false), false);
    assert_eq!(and(true, true), true);
}

#[test]
fn test_and_function_pointer() {
    let op: And = AND;
    assert_eq!(op(false, false), false);
    assert_eq!(op(false, true), false);
    assert_eq!(op(true, false), false);
    assert_eq!(op(true, true), true);

    let fn_item: And = and;
    assert_eq!(fn_item(false, false), false);
    assert_eq!(fn_item(false, true), false);
    assert_eq!(fn_item(true, false), false);
    assert_eq!(fn_item(true, true), true);
}

// ============================================================================
// 3. Or (UC-EV-BOOL-003)
// ============================================================================

#[test]
fn test_or_truth_table() {
    assert_eq!(or(false, false), false);
    assert_eq!(or(false, true), true);
    assert_eq!(or(true, false), true);
    assert_eq!(or(true, true), true);
}

#[test]
fn test_or_function_pointer() {
    let op: Or = OR;
    assert_eq!(op(false, false), false);
    assert_eq!(op(false, true), true);
    assert_eq!(op(true, false), true);
    assert_eq!(op(true, true), true);

    let fn_item: Or = or;
    assert_eq!(fn_item(false, false), false);
    assert_eq!(fn_item(false, true), true);
    assert_eq!(fn_item(true, false), true);
    assert_eq!(fn_item(true, true), true);
}

// ============================================================================
// 4. Xor (UC-EV-BOOL-004)
// ============================================================================

#[test]
fn test_xor_truth_table() {
    assert_eq!(xor(false, false), false);
    assert_eq!(xor(false, true), true);
    assert_eq!(xor(true, false), true);
    assert_eq!(xor(true, true), false);
}

#[test]
fn test_xor_function_pointer() {
    let op: Xor = XOR;
    assert_eq!(op(false, false), false);
    assert_eq!(op(false, true), true);
    assert_eq!(op(true, false), true);
    assert_eq!(op(true, true), false);

    let fn_item: Xor = xor;
    assert_eq!(fn_item(false, false), false);
    assert_eq!(fn_item(false, true), true);
    assert_eq!(fn_item(true, false), true);
    assert_eq!(fn_item(true, true), false);
}

// ============================================================================
// 5. Public surface verification
// ============================================================================

#[test]
fn test_public_surface_access() {
    // Verify type aliases from definitions
    let _not_alias: Not = not;
    let _and_alias: And = and;
    let _or_alias: Or = or;
    let _xor_alias: Xor = xor;

    // Verify constants
    assert_eq!(NOT(false), true);
    assert_eq!(AND(true, true), true);
    assert_eq!(OR(false, true), true);
    assert_eq!(XOR(true, false), true);

    // Verify functions
    assert_eq!(not(true), false);
    assert_eq!(and(true, false), false);
    assert_eq!(or(false, false), false);
    assert_eq!(xor(true, true), false);

    // Verify re-exports from boolean module directly
    use evo_values::boolean::{And as BAnd, Not as BNot, Or as BOr, Xor as BXor};
    let _b_not: BNot = NOT;
    let _b_and: BAnd = AND;
    let _b_or: BOr = OR;
    let _b_xor: BXor = XOR;
}
