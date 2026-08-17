use evo_fs_provider_std::iterator::ITERATE;
use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DIR_MUTEX: Mutex<()> = Mutex::new(());
static RECORD_COUNT: AtomicUsize = AtomicUsize::new(0);
static SEEN_FILE: AtomicBool = AtomicBool::new(false);
static SEEN_DIR: AtomicBool = AtomicBool::new(false);
static FOUND_SIZE: AtomicUsize = AtomicUsize::new(0);
static FOUND_DIR_NO_SIZE: AtomicBool = AtomicBool::new(false);

struct CurrentDirGuard {
    original_dir: std::path::PathBuf,
    temp_dir: std::path::PathBuf,
}

impl CurrentDirGuard {
    fn new(test_name: &str) -> (Self, std::sync::MutexGuard<'static, ()>) {
        let lock = DIR_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        RECORD_COUNT.store(0, Ordering::SeqCst);
        SEEN_FILE.store(false, Ordering::SeqCst);
        SEEN_DIR.store(false, Ordering::SeqCst);
        FOUND_SIZE.store(0, Ordering::SeqCst);
        FOUND_DIR_NO_SIZE.store(false, Ordering::SeqCst);

        let original_dir = std::env::current_dir().expect("failed to get current dir");
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir =
            std::env::temp_dir().join(format!("evo_fs_test_{}_{}", test_name, unique_id));
        std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");
        std::env::set_current_dir(&temp_dir).expect("failed to set current dir");
        (
            Self {
                original_dir,
                temp_dir,
            },
            lock,
        )
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.original_dir);
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
