use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteInboxStore;
use paperclip_backend::inbox::InboxRepository;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-inbox-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

/// Dismissals persist across instances, scoped by (company, user).
#[test]
fn dismissals_persist_across_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteInboxStore::open(path_str);
        store.dismiss("co-1", "u-1", "approval:apr-1", "100".to_string());
    }

    let store = SqliteInboxStore::open(path_str);
    let list = store.list("co-1", "u-1");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].item_key, "approval:apr-1");
    assert_eq!(list[0].dismissed_at, "100");
    // A different user sees nothing.
    assert!(store.list("co-1", "u-2").is_empty());

    std::fs::remove_file(&path).ok();
}

/// Dismissing the same item twice is idempotent (one row), refreshing the time.
#[test]
fn dismiss_is_idempotent_per_item() {
    let path = temp_db_path();
    let store = SqliteInboxStore::open(path.to_str().unwrap());

    store.dismiss("co-1", "u-1", "run:r-1", "100".to_string());
    store.dismiss("co-1", "u-1", "run:r-1", "200".to_string());

    let list = store.list("co-1", "u-1");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].dismissed_at, "200");

    std::fs::remove_file(&path).ok();
}
