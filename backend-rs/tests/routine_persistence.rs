use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteRoutineStore;
use paperclip_backend::routines::{CreateRoutine, RoutineRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-routine-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_routine(title: &str) -> CreateRoutine {
    CreateRoutine {
        title: title.to_string(),
        status: "active".to_string(),
    }
}

/// Routines written by one store instance are visible to a fresh instance opened
/// on the same database file.
#[test]
fn routine_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteRoutineStore::open(path_str);
        store.create("co-1", new_routine("Daily standup"));
    }

    let store = SqliteRoutineStore::open(path_str);
    let routines = store.list_by_company("co-1");
    assert_eq!(routines.len(), 1);
    assert_eq!(routines[0].title, "Daily standup");
    assert_eq!(routines[0].status, "active");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_routines_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteRoutineStore::open(path.to_str().unwrap());

    store.create("co-1", new_routine("secret"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
