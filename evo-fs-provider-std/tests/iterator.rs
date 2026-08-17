use evo_fs_provider_std::iterator::ITERATE;
use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::in_condition::InCondition;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::new_field::NewField;
use evo_shell::definitions::structs::borrowed::selection::Selection;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::borrowed::value_expression::ValueExpression;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_shell::definitions::structs::owned::flow::Flow;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

static DIR_MUTEX: Mutex<()> = Mutex::new(());
static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

static RECORD_COUNT: AtomicUsize = AtomicUsize::new(0);
static SEEN_FILE: AtomicBool = AtomicBool::new(false);
static SEEN_DIR: AtomicBool = AtomicBool::new(false);
static FOUND_SIZE: AtomicUsize = AtomicUsize::new(0);
static FOUND_DIR_NO_SIZE: AtomicBool = AtomicBool::new(false);
static RECEIVED_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static RECEIVED_INDICES: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static RECEIVED_FIELD_NAMES: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new());
static RECEIVED_TEXT_VALUES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static RECEIVED_UNSIGNED_VALUES: Mutex<Vec<u64>> = Mutex::new(Vec::new());
static RECEIVED_SIGNED_VALUES: Mutex<Vec<i64>> = Mutex::new(Vec::new());
static RECEIVED_BOOLEAN_VALUES: Mutex<Vec<bool>> = Mutex::new(Vec::new());

struct CurrentDirGuard {
    temp_dir: PathBuf,
}

impl CurrentDirGuard {
    fn new(test_name: &str) -> (Self, std::sync::MutexGuard<'static, ()>) {
        let lock = DIR_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let base_dir = BASE_DIR
            .get_or_init(|| std::env::current_dir().expect("failed to get initial working dir"));

        std::env::set_current_dir(base_dir).expect("failed to restore base dir");

        RECORD_COUNT.store(0, Ordering::SeqCst);
        SEEN_FILE.store(false, Ordering::SeqCst);
        SEEN_DIR.store(false, Ordering::SeqCst);
        FOUND_SIZE.store(0, Ordering::SeqCst);
        FOUND_DIR_NO_SIZE.store(false, Ordering::SeqCst);
        RECEIVED_NAMES.lock().unwrap().clear();
        RECEIVED_INDICES.lock().unwrap().clear();
        RECEIVED_FIELD_NAMES.lock().unwrap().clear();
        RECEIVED_TEXT_VALUES.lock().unwrap().clear();
        RECEIVED_UNSIGNED_VALUES.lock().unwrap().clear();
        RECEIVED_SIGNED_VALUES.lock().unwrap().clear();
        RECEIVED_BOOLEAN_VALUES.lock().unwrap().clear();

        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir =
            std::env::temp_dir().join(format!("evo_fs_test_{}_{}", test_name, unique_id));
        std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");
        std::env::set_current_dir(&temp_dir).expect("failed to set current dir");
        (Self { temp_dir }, lock)
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        if let Some(base_dir) = BASE_DIR.get() {
            let _ = std::env::set_current_dir(base_dir);
        }
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

fn count_records_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Record(record) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            assert!(record.fields.len() >= 4);
        }
        Construction::Value(_) => panic!("expected record construction"),
    }
    Flow::Continue
}

fn check_record_fields_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Record(record) => {
            let has_index = record.fields.iter().any(|f| f.name == "index");
            let has_name = record.fields.iter().any(|f| f.name == "name");
            let has_path = record.fields.iter().any(|f| f.name == "path");
            let has_kind = record.fields.iter().any(|f| f.name == "kind");

            assert!(has_index);
            assert!(has_name);
            assert!(has_path);
            assert!(has_kind);

            let kind_field = record.fields.iter().find(|f| f.name == "kind").unwrap();
            match kind_field.value {
                Value::Text("file") => SEEN_FILE.store(true, Ordering::SeqCst),
                Value::Text("directory") => SEEN_DIR.store(true, Ordering::SeqCst),
                _ => {}
            }
        }
        Construction::Value(_) => panic!("expected record construction"),
    }
    Flow::Continue
}

fn check_file_size_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Record(record) => {
            let name_field = record.fields.iter().find(|f| f.name == "name").unwrap();
            if name_field.value == Value::Text("data.bin") {
                let kind_field = record.fields.iter().find(|f| f.name == "kind").unwrap();
                assert_eq!(kind_field.value, Value::Text("file"));

                let size_field = record.fields.iter().find(|f| f.name == "size");
                assert!(size_field.is_some());
                if let Value::Unsigned(size) = size_field.unwrap().value {
                    FOUND_SIZE.store(size as usize, Ordering::SeqCst);
                }
            }
        }
        Construction::Value(_) => panic!("expected record construction"),
    }
    Flow::Continue
}

fn check_dir_no_size_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Record(record) => {
            let name_field = record.fields.iter().find(|f| f.name == "name").unwrap();
            if name_field.value == Value::Text("subdir") {
                let kind_field = record.fields.iter().find(|f| f.name == "kind").unwrap();
                assert_eq!(kind_field.value, Value::Text("directory"));

                let size_field = record.fields.iter().find(|f| f.name == "size");
                if size_field.is_none() {
                    FOUND_DIR_NO_SIZE.store(true, Ordering::SeqCst);
                }
            }
        }
        Construction::Value(_) => panic!("expected record construction"),
    }
    Flow::Continue
}

fn collect_names_and_continue(construction: Construction<'_>) -> Flow {
    if let Construction::Record(record) = construction {
        RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
        if let Some(name_field) = record.fields.iter().find(|f| f.name == "name") {
            if let Value::Text(name) = name_field.value {
                RECEIVED_NAMES.lock().unwrap().push(name.to_string());
            }
        }
    }
    Flow::Continue
}

fn collect_names_and_indices_and_continue(construction: Construction<'_>) -> Flow {
    if let Construction::Record(record) = construction {
        RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
        if let Some(name_field) = record.fields.iter().find(|f| f.name == "name") {
            if let Value::Text(name) = name_field.value {
                RECEIVED_NAMES.lock().unwrap().push(name.to_string());
            }
        }
        if let Some(index_field) = record.fields.iter().find(|f| f.name == "index") {
            if let Value::Unsigned(idx) = index_field.value {
                RECEIVED_INDICES.lock().unwrap().push(idx);
            }
        }
    }
    Flow::Continue
}

fn collect_field_names_and_continue(construction: Construction<'_>) -> Flow {
    if let Construction::Record(record) = construction {
        RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
        let names: Vec<String> = record.fields.iter().map(|f| f.name.to_string()).collect();
        RECEIVED_FIELD_NAMES.lock().unwrap().push(names);
    }
    Flow::Continue
}

fn collect_text_values_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(Value::Text(text)) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            RECEIVED_TEXT_VALUES.lock().unwrap().push(text.to_string());
        }
        _ => panic!("expected text value construction"),
    }
    Flow::Continue
}

fn collect_unsigned_values_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(Value::Unsigned(val)) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            RECEIVED_UNSIGNED_VALUES.lock().unwrap().push(val);
        }
        _ => panic!("expected unsigned value construction"),
    }
    Flow::Continue
}

fn collect_signed_values_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(Value::Signed(val)) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            RECEIVED_SIGNED_VALUES.lock().unwrap().push(val);
        }
        _ => panic!("expected signed value construction"),
    }
    Flow::Continue
}

fn collect_boolean_values_and_continue(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(Value::Boolean(val)) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            RECEIVED_BOOLEAN_VALUES.lock().unwrap().push(val);
        }
        _ => panic!("expected boolean value construction"),
    }
    Flow::Continue
}

fn value_flow_stop(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(_) => {
            RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
            Flow::Stop
        }
        _ => panic!("expected value construction"),
    }
}

fn count_and_continue(_construction: Construction<'_>) -> Flow {
    RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
    Flow::Continue
}

fn count_and_stop(_construction: Construction<'_>) -> Flow {
    RECORD_COUNT.fetch_add(1, Ordering::SeqCst);
    Flow::Stop
}

fn panic_on_call(_construction: Construction<'_>) -> Flow {
    panic!("requester should not be called");
}

fn ignore_and_continue(_construction: Construction<'_>) -> Flow {
    Flow::Continue
}

#[test]
fn iterator_empty_iteration_enumerates_current_directory() {
    let (_guard, _lock) = CurrentDirGuard::new("empty_iter_enum");
    std::fs::write("file_a.txt", b"hello").unwrap();
    std::fs::write("file_b.txt", b"world").unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_records_and_continue);

    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_records_contain_required_fields() {
    let (_guard, _lock) = CurrentDirGuard::new("records_fields");
    std::fs::write("sample.txt", b"content").unwrap();
    std::fs::create_dir("sample_dir").unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, check_record_fields_and_continue);

    assert_eq!(result, Ok(()));
    assert!(SEEN_FILE.load(Ordering::SeqCst));
    assert!(SEEN_DIR.load(Ordering::SeqCst));
}

#[test]
fn iterator_regular_file_contains_size() {
    let (_guard, _lock) = CurrentDirGuard::new("file_size");
    let content = b"1234567890";
    std::fs::write("data.bin", content).unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, check_file_size_and_continue);

    assert_eq!(result, Ok(()));
    assert_eq!(FOUND_SIZE.load(Ordering::SeqCst), content.len());
}

#[test]
fn iterator_directory_does_not_contain_size() {
    let (_guard, _lock) = CurrentDirGuard::new("dir_no_size");
    std::fs::create_dir("subdir").unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, check_dir_no_size_and_continue);

    assert_eq!(result, Ok(()));
    assert!(FOUND_DIR_NO_SIZE.load(Ordering::SeqCst));
}

#[test]
fn iterator_flow_continue_allows_multiple_results() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_continue");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);

    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 3);
}

#[test]
fn iterator_flow_stop_stops_enumeration() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_stop");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_stop);

    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_non_empty_iteration_returns_provider_incompatible() {
    let operations = [IterationOperation::Count];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, ignore_and_continue);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_zero_results_empty_directory() {
    let (_guard, _lock) = CurrentDirGuard::new("empty_dir");

    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Ok(()));
}

#[test]
fn iterator_filter_simple_equal_text() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_equal_text");
    std::fs::write("one.txt", b"1").unwrap();
    std::fs::write("two.txt", b"2").unwrap();
    std::fs::write("three.txt", b"3").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("one.txt"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["one.txt"]);
}

#[test]
fn iterator_filter_kind() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_kind");
    std::fs::write("file.txt", b"file").unwrap();
    std::fs::create_dir("folder").unwrap();

    let condition = Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["file.txt"]);
}

#[test]
fn iterator_filter_contains() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_contains");
    std::fs::write("alpha.txt", b"a").unwrap();
    std::fs::write("beta.log", b"b").unwrap();
    std::fs::write("gamma.txt", b"c").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Contains,
        value: Value::Text(".txt"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let mut names = RECEIVED_NAMES.lock().unwrap().clone();
    names.sort();
    assert_eq!(names, vec!["alpha.txt", "gamma.txt"]);
}

#[test]
fn iterator_filter_starts_with() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_starts_with");
    std::fs::write("pre_one.txt", b"1").unwrap();
    std::fs::write("pre_two.txt", b"2").unwrap();
    std::fs::write("other.txt", b"3").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::StartsWith,
        value: Value::Text("pre_"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let mut names = RECEIVED_NAMES.lock().unwrap().clone();
    names.sort();
    assert_eq!(names, vec!["pre_one.txt", "pre_two.txt"]);
}

#[test]
fn iterator_filter_ends_with() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_ends_with");
    std::fs::write("data.log", b"1").unwrap();
    std::fs::write("info.log", b"2").unwrap();
    std::fs::write("app.txt", b"3").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".log"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let mut names = RECEIVED_NAMES.lock().unwrap().clone();
    names.sort();
    assert_eq!(names, vec!["data.log", "info.log"]);
}

#[test]
fn iterator_filter_size_greater_than() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_size_gt");
    std::fs::write("small.bin", b"12345").unwrap();
    std::fs::write("large.bin", b"12345678901234567890").unwrap();

    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(10),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["large.bin"]);
}

#[test]
fn iterator_filter_index_less_than() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_index_lt");
    std::fs::write("a.txt", b"a").unwrap();
    std::fs::write("b.txt", b"b").unwrap();
    std::fs::write("c.txt", b"c").unwrap();

    let condition = Condition {
        field: "index",
        operator: ConditionOperator::LessThan,
        value: Value::Unsigned(1),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_filter_between_inclusive() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_between");
    std::fs::write("a.bin", b"1234567890").unwrap();
    std::fs::write("b.bin", b"123456789012345").unwrap();
    std::fs::write("c.bin", b"12345678901234567890").unwrap();
    std::fs::write("d.bin", b"1234567890123456789012345").unwrap();

    let between = BetweenCondition {
        field: "size",
        lower: Value::Unsigned(10),
        upper: Value::Unsigned(20),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Between(between));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 3);
}

#[test]
fn iterator_filter_in() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_in");
    std::fs::write("one.txt", b"1").unwrap();
    std::fs::write("two.txt", b"2").unwrap();
    std::fs::write("three.txt", b"3").unwrap();
    std::fs::write("four.txt", b"4").unwrap();

    let candidates = [Value::Text("one.txt"), Value::Text("three.txt")];
    let in_condition = InCondition {
        field: "name",
        values: &candidates,
    };
    let filter = IterationOperation::Filter(ConditionExpression::In(in_condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let mut names = RECEIVED_NAMES.lock().unwrap().clone();
    names.sort();
    assert_eq!(names, vec!["one.txt", "three.txt"]);
}

#[test]
fn iterator_filter_not() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_not");
    std::fs::write("one.txt", b"1").unwrap();
    std::fs::write("two.txt", b"2").unwrap();

    let inner = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("one.txt"),
    });
    let filter = IterationOperation::Filter(ConditionExpression::Not(&inner));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["two.txt"]);
}

#[test]
fn iterator_filter_and() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_and");
    std::fs::write("one.txt", b"1234567890").unwrap();
    std::fs::write("two.txt", b"12").unwrap();
    std::fs::write("three.log", b"1234567890").unwrap();

    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    });
    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(5),
    });
    let children = [a, b];
    let filter = IterationOperation::Filter(ConditionExpression::And(&children));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["one.txt"]);
}

#[test]
fn iterator_filter_or() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_or");
    std::fs::write("one.txt", b"1").unwrap();
    std::fs::write("two.txt", b"2").unwrap();
    std::fs::write("three.txt", b"3").unwrap();

    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("one.txt"),
    });
    let b = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("two.txt"),
    });
    let children = [a, b];
    let filter = IterationOperation::Filter(ConditionExpression::Or(&children));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let mut names = RECEIVED_NAMES.lock().unwrap().clone();
    names.sort();
    assert_eq!(names, vec!["one.txt", "two.txt"]);
}

#[test]
fn iterator_filter_and_empty_is_true() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_and_empty");
    std::fs::write("file.txt", b"1").unwrap();

    let children: [ConditionExpression<'_>; 0] = [];
    let filter = IterationOperation::Filter(ConditionExpression::And(&children));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_filter_or_empty_is_false() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_or_empty");
    std::fs::write("file.txt", b"1").unwrap();

    let children: [ConditionExpression<'_>; 0] = [];
    let filter = IterationOperation::Filter(ConditionExpression::Or(&children));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 0);
}

#[test]
fn iterator_multiple_filters() {
    let (_guard, _lock) = CurrentDirGuard::new("multiple_filters");
    std::fs::write("one.txt", b"1234567890").unwrap();
    std::fs::write("two.txt", b"12").unwrap();
    std::fs::write("three.log", b"1234567890").unwrap();

    let filter_1 = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    }));
    let filter_2 = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(5),
    }));
    let operations = [filter_1, filter_2];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, collect_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_NAMES.lock().unwrap(), vec!["one.txt"]);
}

#[test]
fn iterator_filter_field_not_found() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_field_not_found");
    std::fs::write("file.txt", b"1").unwrap();

    let condition = Condition {
        field: "unknown",
        operator: ConditionOperator::Equal,
        value: Value::Text("x"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(
        result,
        Err(iterate_contract::Error::FieldNotFound("unknown"))
    );
}

#[test]
fn iterator_filter_size_absent_on_directory_returns_field_not_found() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_size_absent_dir");
    std::fs::create_dir("subdir").unwrap();

    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(0),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Err(iterate_contract::Error::FieldNotFound("size")));
}

#[test]
fn iterator_filter_type_mismatch() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_type_mismatch");
    std::fs::write("file.txt", b"1").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(1),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(
        result,
        Err(iterate_contract::Error::ComparisonTypeMismatch("name"))
    );
}

#[test]
fn iterator_filter_signed_unsigned_no_coercion() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_signed_unsigned_mismatch");
    std::fs::write("file.txt", b"1234567890").unwrap();

    let condition = Condition {
        field: "size",
        operator: ConditionOperator::Equal,
        value: Value::Signed(10),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(
        result,
        Err(iterate_contract::Error::ComparisonTypeMismatch("size"))
    );
}

#[test]
fn iterator_filter_and_non_filter_returns_provider_incompatible_before_running() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("file.txt"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter, IterationOperation::Count];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_filter_flow_stop_after_filter() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_flow_stop");
    std::fs::write("a.txt", b"a").unwrap();
    std::fs::write("b.txt", b"b").unwrap();
    std::fs::write("c.txt", b"c").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let filter = IterationOperation::Filter(ConditionExpression::Condition(condition));
    let operations = [filter];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_stop);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_skip_zero() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_zero");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let operations = [IterationOperation::Skip(0)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_skip_basic() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_basic");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let operations = [IterationOperation::Skip(2)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_skip_greater_than_available() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_gt_avail");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let operations = [IterationOperation::Skip(10)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 0);
}

#[test]
fn iterator_take_zero() {
    let (_guard, _lock) = CurrentDirGuard::new("take_zero");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let operations = [IterationOperation::Take(0)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 0);
}

#[test]
fn iterator_take_basic() {
    let (_guard, _lock) = CurrentDirGuard::new("take_basic");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let operations = [IterationOperation::Take(2)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_take_greater_than_available() {
    let (_guard, _lock) = CurrentDirGuard::new("take_gt_avail");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let operations = [IterationOperation::Take(10)];
    let iteration = Iteration {
        operations: &operations,
    };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_skip_then_take_vs_take_then_skip() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_take_order");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    // First observe the exact order read_dir produces for this directory
    let empty_ops: [IterationOperation<'_>; 0] = [];
    let empty_iteration = Iteration {
        operations: &empty_ops,
    };
    ITERATE(empty_iteration, collect_names_and_continue).unwrap();
    let initial_names = RECEIVED_NAMES.lock().unwrap().clone();
    assert_eq!(initial_names.len(), 4);

    // Test Skip(1) |> Take(2): drops 1st item, takes next 2 -> should have 2 elements [initial_names[1], initial_names[2]]
    RECEIVED_NAMES.lock().unwrap().clear();
    RECORD_COUNT.store(0, Ordering::SeqCst);
    let skip_take_ops = [IterationOperation::Skip(1), IterationOperation::Take(2)];
    let skip_take_iter = Iteration {
        operations: &skip_take_ops,
    };
    let res1 = ITERATE(skip_take_iter, collect_names_and_continue);
    assert_eq!(res1, Ok(()));
    let skip_take_names = RECEIVED_NAMES.lock().unwrap().clone();
    assert_eq!(skip_take_names.len(), 2);
    assert_eq!(skip_take_names, initial_names[1..3]);

    // Test Take(2) |> Skip(1): takes first 2 items [0, 1], then skips 1st -> should have 1 element [initial_names[1]]
    RECEIVED_NAMES.lock().unwrap().clear();
    RECORD_COUNT.store(0, Ordering::SeqCst);
    let take_skip_ops = [IterationOperation::Take(2), IterationOperation::Skip(1)];
    let take_skip_iter = Iteration {
        operations: &take_skip_ops,
    };
    let res2 = ITERATE(take_skip_iter, collect_names_and_continue);
    assert_eq!(res2, Ok(()));
    let take_skip_names = RECEIVED_NAMES.lock().unwrap().clone();
    assert_eq!(take_skip_names.len(), 1);
    assert_eq!(take_skip_names, vec![initial_names[1].clone()]);

    assert_ne!(skip_take_names, take_skip_names);
}

#[test]
fn iterator_filter_then_skip() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_then_skip");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.log", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
        IterationOperation::Skip(1),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_skip_then_filter() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_then_filter");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Skip(2),
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_filter_then_take() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_then_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.log", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
        IterationOperation::Take(2),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_take_then_filter() {
    let (_guard, _lock) = CurrentDirGuard::new("take_then_filter");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Take(1),
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_multiple_skips() {
    let (_guard, _lock) = CurrentDirGuard::new("multiple_skips");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let ops = [IterationOperation::Skip(1), IterationOperation::Skip(2)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_multiple_takes() {
    let (_guard, _lock) = CurrentDirGuard::new("multiple_takes");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let ops = [IterationOperation::Take(3), IterationOperation::Take(2)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_filter_skip_take_pipeline() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_skip_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.log", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();
    std::fs::write("5.txt", b"5").unwrap();

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
        IterationOperation::Skip(1),
        IterationOperation::Take(2),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_index_preserved_after_skip_and_take() {
    let (_guard, _lock) = CurrentDirGuard::new("index_preserved");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let ops = [IterationOperation::Skip(1), IterationOperation::Take(2)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_names_and_indices_and_continue);
    assert_eq!(result, Ok(()));
    let indices = RECEIVED_INDICES.lock().unwrap().clone();
    assert_eq!(indices.len(), 2);
    // Indices must be original enumeration indices [1, 2], not renumerated [0, 1]
    assert_eq!(indices, vec![1, 2]);
}

#[test]
fn iterator_flow_stop_after_skip_and_take() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_stop_skip_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let ops = [IterationOperation::Skip(1), IterationOperation::Take(2)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_stop);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_filter_take_and_count_returns_provider_incompatible() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    };
    let ops = [
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
        IterationOperation::Take(2),
        IterationOperation::Count,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_simple() {
    let (_guard, _lock) = CurrentDirGuard::new("select_simple");
    std::fs::write("1.txt", b"1").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(field_names, vec![vec!["name".to_string()]]);
}

#[test]
fn iterator_select_multiple() {
    let (_guard, _lock) = CurrentDirGuard::new("select_multiple");
    std::fs::write("1.txt", b"1").unwrap();

    let selections = [
        Selection::Field("name"),
        Selection::Field("kind"),
        Selection::Field("index"),
    ];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec![
            "name".to_string(),
            "kind".to_string(),
            "index".to_string()
        ]]
    );
}

#[test]
fn iterator_select_field_order() {
    let (_guard, _lock) = CurrentDirGuard::new("select_order");
    std::fs::write("1.txt", b"1").unwrap();

    let selections = [
        Selection::Field("kind"),
        Selection::Field("name"),
        Selection::Field("index"),
    ];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec![
            "kind".to_string(),
            "name".to_string(),
            "index".to_string()
        ]]
    );
}

#[test]
fn iterator_select_field_not_found() {
    let (_guard, _lock) = CurrentDirGuard::new("select_not_found");
    std::fs::write("1.txt", b"1").unwrap();

    let selections = [Selection::Field("unknown")];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(
        result,
        Err(iterate_contract::Error::FieldNotFound("unknown"))
    );
}

#[test]
fn iterator_filter_then_select() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_then_select");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::create_dir("folder").unwrap();

    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let selections = [Selection::Field("name")];
    let select = IterationOperation::Select(&selections);
    let ops = [filter, select];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(field_names, vec![vec!["name".to_string()]]);
}

#[test]
fn iterator_select_then_filter_valid() {
    let (_guard, _lock) = CurrentDirGuard::new("select_then_filter_valid");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::create_dir("folder").unwrap();

    let selections = [Selection::Field("name"), Selection::Field("kind")];
    let select = IterationOperation::Select(&selections);
    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let ops = [select, filter];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string(), "kind".to_string()]]
    );
}

#[test]
fn iterator_select_then_filter_removed_field_returns_field_not_found() {
    let (_guard, _lock) = CurrentDirGuard::new("select_then_filter_removed");
    std::fs::write("1.txt", b"1").unwrap();

    let selections = [Selection::Field("name")];
    let select = IterationOperation::Select(&selections);
    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let ops = [select, filter];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::FieldNotFound("kind")));
}

#[test]
fn iterator_filter_field_before_remove_succeeds() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_before_remove");
    std::fs::write("1.txt", b"1").unwrap();

    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let selections = [Selection::Field("name")];
    let select = IterationOperation::Select(&selections);
    let ops = [filter, select];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_multiple_selects() {
    let (_guard, _lock) = CurrentDirGuard::new("multiple_selects");
    std::fs::write("1.txt", b"1").unwrap();

    let s1 = [
        Selection::Field("index"),
        Selection::Field("name"),
        Selection::Field("path"),
    ];
    let s2 = [Selection::Field("name"), Selection::Field("path")];
    let s3 = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&s1),
        IterationOperation::Select(&s2),
        IterationOperation::Select(&s3),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(field_names, vec![vec!["name".to_string()]]);
}

#[test]
fn iterator_skip_then_select() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_then_select");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Skip(1),
        IterationOperation::Select(&selections),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string()], vec!["name".to_string()]]
    );
}

#[test]
fn iterator_select_then_skip() {
    let (_guard, _lock) = CurrentDirGuard::new("select_then_skip");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::Skip(1),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string()], vec!["name".to_string()]]
    );
}

#[test]
fn iterator_take_then_select() {
    let (_guard, _lock) = CurrentDirGuard::new("take_then_select");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Take(2),
        IterationOperation::Select(&selections),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string()], vec!["name".to_string()]]
    );
}

#[test]
fn iterator_select_then_take() {
    let (_guard, _lock) = CurrentDirGuard::new("select_then_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::Take(2),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string()], vec!["name".to_string()]]
    );
}

#[test]
fn iterator_filter_skip_select_take_pipeline() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_skip_select_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.log", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();
    std::fs::write("5.txt", b"5").unwrap();

    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::Text(".txt"),
    }));
    let skip = IterationOperation::Skip(1);
    let selections = [Selection::Field("name")];
    let select = IterationOperation::Select(&selections);
    let take = IterationOperation::Take(2);
    let ops = [filter, skip, select, take];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string()], vec!["name".to_string()]]
    );
}

#[test]
fn iterator_index_preserved_in_select() {
    let (_guard, _lock) = CurrentDirGuard::new("index_preserved_select");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();
    std::fs::write("4.txt", b"4").unwrap();

    let selections = [Selection::Field("index"), Selection::Field("name")];
    let ops = [
        IterationOperation::Skip(1),
        IterationOperation::Take(2),
        IterationOperation::Select(&selections),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_names_and_indices_and_continue);
    assert_eq!(result, Ok(()));
    let indices = RECEIVED_INDICES.lock().unwrap().clone();
    assert_eq!(indices, vec![1, 2]);
}

#[test]
fn iterator_flow_stop_after_select() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_stop_select");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_stop);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_select_name_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("select_name_to_value");
    std::fs::write("file.txt", b"1").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_TEXT_VALUES.lock().unwrap(), vec!["file.txt"]);
}

#[test]
fn iterator_select_index_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("select_index_to_value");
    std::fs::write("file.txt", b"1").unwrap();

    let selections = [Selection::Field("index")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![0]);
}

#[test]
fn iterator_select_size_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("select_size_to_value");
    let content = b"12345678";
    std::fs::write("file.bin", content).unwrap();

    let selections = [Selection::Field("size")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(
        *RECEIVED_UNSIGNED_VALUES.lock().unwrap(),
        vec![content.len() as u64]
    );
}

#[test]
fn iterator_to_value_on_original_record_returns_single_field_error() {
    let (_guard, _lock) = CurrentDirGuard::new("to_value_original_record");
    std::fs::write("file.txt", b"1").unwrap();

    let ops = [IterationOperation::ToValue];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(
        result,
        Err(iterate_contract::Error::ToValueRequiresSingleField)
    );
}

#[test]
fn iterator_select_multiple_fields_to_value_returns_single_field_error() {
    let (_guard, _lock) = CurrentDirGuard::new("select_multiple_to_value");
    std::fs::write("file.txt", b"1").unwrap();

    let selections = [Selection::Field("name"), Selection::Field("kind")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(
        result,
        Err(iterate_contract::Error::ToValueRequiresSingleField)
    );
}

#[test]
fn iterator_filter_then_select_then_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_select_to_value");
    std::fs::write("file.txt", b"1").unwrap();
    std::fs::create_dir("subfolder").unwrap();

    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let selections = [Selection::Field("name")];
    let select = IterationOperation::Select(&selections);
    let to_value = IterationOperation::ToValue;
    let ops = [filter, select, to_value];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_TEXT_VALUES.lock().unwrap(), vec!["file.txt"]);
}

#[test]
fn iterator_skip_then_select_then_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_select_to_value");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Skip(1),
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_take_then_select_then_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("take_select_to_value");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Take(2),
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_select_then_to_value_then_take() {
    let (_guard, _lock) = CurrentDirGuard::new("select_to_value_take");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
        IterationOperation::Take(2),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_select_then_to_value_then_skip() {
    let (_guard, _lock) = CurrentDirGuard::new("select_to_value_skip");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
        IterationOperation::Skip(1),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn iterator_skip_then_select_index_to_value_preserves_original_index() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_select_index_to_value");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let selections = [Selection::Field("index")];
    let ops = [
        IterationOperation::Skip(1),
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![1, 2]);
}

#[test]
fn iterator_flow_stop_after_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_stop_to_value");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, value_flow_stop);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_to_value_then_filter_returns_provider_incompatible_before_running() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("file.txt"),
    };
    let ops = [
        IterationOperation::ToValue,
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_to_value_then_select_returns_provider_incompatible_before_running() {
    let selections1 = [Selection::Field("name")];
    let selections2 = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections1),
        IterationOperation::ToValue,
        IterationOperation::Select(&selections2),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_to_value_then_to_value_returns_to_value_requires_record_before_running() {
    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ToValueRequiresRecord));
}

#[test]
fn iterator_first_returns_provider_incompatible() {
    let ops = [IterationOperation::First];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_last_returns_provider_incompatible() {
    let ops = [IterationOperation::Last];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_count_returns_provider_incompatible() {
    let ops = [IterationOperation::Count];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_empty_returns_provider_incompatible() {
    let selections: [Selection<'_>; 0] = [];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_plus_count_returns_provider_incompatible() {
    let selections = [Selection::Field("name")];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::Count,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_literal_text() {
    let (_guard, _lock) = CurrentDirGuard::new("new_literal_text");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "label",
        expression: ValueExpression::Literal(Value::Text("hello")),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_TEXT_VALUES.lock().unwrap(), vec!["hello"]);
}

#[test]
fn iterator_new_literal_unsigned() {
    let (_guard, _lock) = CurrentDirGuard::new("new_literal_unsigned");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "count",
        expression: ValueExpression::Literal(Value::Unsigned(42)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![42]);
}

#[test]
fn iterator_new_literal_signed() {
    let (_guard, _lock) = CurrentDirGuard::new("new_literal_signed");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "delta",
        expression: ValueExpression::Literal(Value::Signed(-5)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_signed_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_SIGNED_VALUES.lock().unwrap(), vec![-5]);
}

#[test]
fn iterator_new_literal_boolean() {
    let (_guard, _lock) = CurrentDirGuard::new("new_literal_boolean");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "flag",
        expression: ValueExpression::Literal(Value::Boolean(true)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_boolean_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_BOOLEAN_VALUES.lock().unwrap(), vec![true]);
}

#[test]
fn iterator_field_plus_new() {
    let (_guard, _lock) = CurrentDirGuard::new("field_plus_new");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "enabled",
        expression: ValueExpression::Literal(Value::Boolean(true)),
    };
    let selections = [Selection::Field("name"), Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["name".to_string(), "enabled".to_string()]]
    );
}

#[test]
fn iterator_new_pipeline_name() {
    let (_guard, _lock) = CurrentDirGuard::new("new_pipeline_name");
    std::fs::write("sample.txt", b"1").unwrap();

    let inner_sels = [Selection::Field("name")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy_name",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_text_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_TEXT_VALUES.lock().unwrap(), vec!["sample.txt"]);
}

#[test]
fn iterator_new_pipeline_index() {
    let (_guard, _lock) = CurrentDirGuard::new("new_pipeline_index");
    std::fs::write("sample.txt", b"1").unwrap();

    let inner_sels = [Selection::Field("index")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy_index",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![0]);
}

#[test]
fn iterator_new_pipeline_size() {
    let (_guard, _lock) = CurrentDirGuard::new("new_pipeline_size");
    let content = b"123456789";
    std::fs::write("sample.bin", content).unwrap();

    let inner_sels = [Selection::Field("size")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy_size",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(
        *RECEIVED_UNSIGNED_VALUES.lock().unwrap(),
        vec![content.len() as u64]
    );
}

#[test]
fn iterator_new_pipeline_field_not_found() {
    let (_guard, _lock) = CurrentDirGuard::new("new_pipeline_not_found");
    std::fs::write("sample.txt", b"1").unwrap();

    let inner_sels = [Selection::Field("missing")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(
        result,
        Err(iterate_contract::Error::FieldNotFound("missing"))
    );
}

#[test]
fn iterator_new_pipeline_multi_field_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("new_pipeline_multi_field");
    std::fs::write("sample.txt", b"1").unwrap();

    let inner_sels = [Selection::Field("name"), Selection::Field("kind")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(
        result,
        Err(iterate_contract::Error::ToValueRequiresSingleField)
    );
}

#[test]
fn iterator_new_pipeline_empty_returns_provider_incompatible_before_running() {
    let inner_ops: [IterationOperation<'_>; 0] = [];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_filter_returns_provider_incompatible_before_running() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("x"),
    };
    let inner_ops = [
        IterationOperation::Filter(ConditionExpression::Condition(condition)),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_skip_returns_provider_incompatible_before_running() {
    let inner_ops = [IterationOperation::Skip(1), IterationOperation::ToValue];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_take_returns_provider_incompatible_before_running() {
    let inner_ops = [IterationOperation::Take(1), IterationOperation::ToValue];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_first_returns_provider_incompatible_before_running() {
    let inner_ops = [IterationOperation::First];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_last_returns_provider_incompatible_before_running() {
    let inner_ops = [IterationOperation::Last];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_count_returns_provider_incompatible_before_running() {
    let inner_ops = [IterationOperation::Count];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_pipeline_select_new_returns_provider_incompatible_before_running() {
    let deep_new = NewField {
        name: "deep",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let inner_sels = [Selection::New(deep_new)];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_field = NewField {
        name: "copy",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_new_concat_returns_provider_incompatible_before_running() {
    let expressions = [ValueExpression::Literal(Value::Text("a"))];
    let new_field = NewField {
        name: "label",
        expression: ValueExpression::Concat(&expressions),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::ProviderIncompatible));
}

#[test]
fn iterator_select_solo_new() {
    let (_guard, _lock) = CurrentDirGuard::new("select_solo_new");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(field_names, vec![vec!["a".to_string()]]);
}

#[test]
fn iterator_select_new_then_to_value() {
    let (_guard, _lock) = CurrentDirGuard::new("select_new_to_value");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![1]);
}

#[test]
fn iterator_filter_then_select_new() {
    let (_guard, _lock) = CurrentDirGuard::new("filter_then_select_new");
    std::fs::write("file.txt", b"1").unwrap();
    std::fs::create_dir("folder").unwrap();

    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "kind",
        operator: ConditionOperator::Equal,
        value: Value::Text("file"),
    }));
    let new_field = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let selections = [Selection::New(new_field)];
    let select = IterationOperation::Select(&selections);
    let ops = [filter, select];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(field_names, vec![vec!["a".to_string()]]);
}

#[test]
fn iterator_select_new_then_filter_new_field() {
    let (_guard, _lock) = CurrentDirGuard::new("select_new_filter_new_field");
    std::fs::write("file.txt", b"1").unwrap();

    let new_field = NewField {
        name: "flag",
        expression: ValueExpression::Literal(Value::Boolean(true)),
    };
    let selections = [Selection::Field("name"), Selection::New(new_field)];
    let select = IterationOperation::Select(&selections);
    let filter = IterationOperation::Filter(ConditionExpression::Condition(Condition {
        field: "flag",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    }));
    let ops = [select, filter];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_same_select_new_cannot_see_previous_new() {
    let (_guard, _lock) = CurrentDirGuard::new("same_select_new_no_cross_ref");
    std::fs::write("file.txt", b"1").unwrap();

    let new_a = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let inner_sels = [Selection::Field("a")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_b = NewField {
        name: "b",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let selections = [Selection::New(new_a), Selection::New(new_b)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, panic_on_call);
    assert_eq!(result, Err(iterate_contract::Error::FieldNotFound("a")));
}

#[test]
fn iterator_second_select_can_see_new_from_first_select() {
    let (_guard, _lock) = CurrentDirGuard::new("second_select_sees_previous_new");
    std::fs::write("file.txt", b"1").unwrap();

    let new_a = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let sel_a = [Selection::New(new_a)];

    let inner_sels = [Selection::Field("a")];
    let inner_ops = [
        IterationOperation::Select(&inner_sels),
        IterationOperation::ToValue,
    ];
    let new_b = NewField {
        name: "b",
        expression: ValueExpression::Pipeline(&inner_ops),
    };
    let sel_b = [Selection::New(new_b)];

    let ops = [
        IterationOperation::Select(&sel_a),
        IterationOperation::Select(&sel_b),
        IterationOperation::ToValue,
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_unsigned_values_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(*RECEIVED_UNSIGNED_VALUES.lock().unwrap(), vec![1]);
}

#[test]
fn iterator_multiple_new_preserve_selection_order() {
    let (_guard, _lock) = CurrentDirGuard::new("multiple_new_preserve_order");
    std::fs::write("file.txt", b"1").unwrap();

    let new_a = NewField {
        name: "a",
        expression: ValueExpression::Literal(Value::Unsigned(1)),
    };
    let new_b = NewField {
        name: "b",
        expression: ValueExpression::Literal(Value::Unsigned(2)),
    };
    let selections = [
        Selection::New(new_a),
        Selection::Field("name"),
        Selection::New(new_b),
    ];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, collect_field_names_and_continue);
    assert_eq!(result, Ok(()));
    let field_names = RECEIVED_FIELD_NAMES.lock().unwrap().clone();
    assert_eq!(
        field_names,
        vec![vec!["a".to_string(), "name".to_string(), "b".to_string()]]
    );
}

#[test]
fn iterator_flow_stop_after_select_new() {
    let (_guard, _lock) = CurrentDirGuard::new("flow_stop_select_new");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();

    let new_field = NewField {
        name: "custom",
        expression: ValueExpression::Literal(Value::Text("test")),
    };
    let selections = [Selection::New(new_field)];
    let ops = [IterationOperation::Select(&selections)];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_stop);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn iterator_skip_take_with_select_new() {
    let (_guard, _lock) = CurrentDirGuard::new("skip_take_select_new");
    std::fs::write("1.txt", b"1").unwrap();
    std::fs::write("2.txt", b"2").unwrap();
    std::fs::write("3.txt", b"3").unwrap();

    let new_field = NewField {
        name: "custom",
        expression: ValueExpression::Literal(Value::Unsigned(100)),
    };
    let selections = [Selection::New(new_field)];
    let ops = [
        IterationOperation::Skip(1),
        IterationOperation::Select(&selections),
        IterationOperation::Take(1),
    ];
    let iteration = Iteration { operations: &ops };

    let result = ITERATE(iteration, count_and_continue);
    assert_eq!(result, Ok(()));
    assert_eq!(RECORD_COUNT.load(Ordering::SeqCst), 1);
}
