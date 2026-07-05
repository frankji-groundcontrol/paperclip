use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteRunStore;
use paperclip_backend::runs::{CreateRun, RunRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("paperclip-run-{}-{}.sqlite", std::process::id(), n))
}

fn new_run(agent_id: &str) -> CreateRun {
    CreateRun {
        agent_id: agent_id.to_string(),
        status: "queued".to_string(),
    }
}

/// Runs written by one store instance are visible to a fresh instance opened on
/// the same database file.
#[test]
fn run_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteRunStore::open(path_str);
        store.create("co-1", new_run("a-1"));
    }

    let store = SqliteRunStore::open(path_str);
    let runs = store.list_by_company("co-1");
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].agent_id, "a-1");
    assert_eq!(runs[0].status, "queued");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_runs_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteRunStore::open(path.to_str().unwrap());

    store.create("co-1", new_run("a-1"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
