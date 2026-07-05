use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqlitePluginStore;
use paperclip_backend::plugins::{InstallPlugin, PluginRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-plugin-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn install(plugin_id: &str, enabled: bool) -> InstallPlugin {
    InstallPlugin {
        plugin_id: plugin_id.to_string(),
        enabled,
    }
}

/// Installed plugins persist across store instances, including the `enabled` flag.
#[test]
fn plugin_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqlitePluginStore::open(path_str);
        store.create("co-1", install("llm-wiki", false));
    }

    let store = SqlitePluginStore::open(path_str);
    let plugins = store.list_by_company("co-1");
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].plugin_id, "llm-wiki");
    assert_eq!(plugins[0].enabled, false);

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_plugins_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqlitePluginStore::open(path.to_str().unwrap());

    store.create("co-1", install("llm-wiki", true));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
