use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteIssueStore;
use paperclip_backend::issues::{CreateIssue, IssueRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-issue-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_issue(title: &str) -> CreateIssue {
    CreateIssue {
        title: title.to_string(),
        status: "backlog".to_string(),
    }
}

/// Issues written by one store instance are visible to a fresh instance opened
/// on the same database file.
#[test]
fn issue_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteIssueStore::open(path_str);
        store.create("co-1", new_issue("Fix bug"));
    }

    let store = SqliteIssueStore::open(path_str);
    let issues = store.list_by_company("co-1");
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].title, "Fix bug");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer: another company sees nothing.
#[test]
fn persisted_issues_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteIssueStore::open(path.to_str().unwrap());

    store.create("co-1", new_issue("secret"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
