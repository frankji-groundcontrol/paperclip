use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::approvals::{ApprovalRepository, CreateApproval};
use paperclip_backend::db::SqliteApprovalStore;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-approval-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_approval(issue_id: &str) -> CreateApproval {
    CreateApproval {
        issue_id: issue_id.to_string(),
        status: "pending".to_string(),
    }
}

/// Approvals written by one store instance are visible to a fresh instance
/// opened on the same database file.
#[test]
fn approval_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteApprovalStore::open(path_str);
        store.create("co-1", new_approval("i-1"));
    }

    let store = SqliteApprovalStore::open(path_str);
    let approvals = store.list_by_company("co-1");
    assert_eq!(approvals.len(), 1);
    assert_eq!(approvals[0].issue_id, "i-1");
    assert_eq!(approvals[0].status, "pending");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_approvals_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteApprovalStore::open(path.to_str().unwrap());

    store.create("co-1", new_approval("i-1"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
