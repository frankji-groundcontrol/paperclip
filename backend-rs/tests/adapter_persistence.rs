use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::adapters::{AdapterRepository, ConfigureAdapter};
use paperclip_backend::db::SqliteAdapterStore;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-adapter-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn configure(adapter_type: &str, enabled: bool) -> ConfigureAdapter {
    ConfigureAdapter {
        adapter_type: adapter_type.to_string(),
        enabled,
    }
}

/// Configured adapters persist across store instances, including `enabled`.
#[test]
fn adapter_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteAdapterStore::open(path_str);
        store.create("co-1", configure("claude_local", false));
    }

    let store = SqliteAdapterStore::open(path_str);
    let adapters = store.list_by_company("co-1");
    assert_eq!(adapters.len(), 1);
    assert_eq!(adapters[0].adapter_type, "claude_local");
    assert_eq!(adapters[0].enabled, false);

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_adapters_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteAdapterStore::open(path.to_str().unwrap());

    store.create("co-1", configure("codex", true));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
