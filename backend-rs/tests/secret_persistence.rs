use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteSecretStore;
use paperclip_backend::secrets::{CreateSecret, SecretRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-secret-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_secret(name: &str, value: &str) -> CreateSecret {
    CreateSecret {
        name: name.to_string(),
        provider: "local".to_string(),
        value: value.to_string(),
    }
}

/// Secret references persist across store instances; the value column is written
/// but never read back into a `SecretRef`.
#[test]
fn secret_refs_persist_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteSecretStore::open(path_str);
        store.create("co-1", new_secret("API_KEY", "super-secret-value"));
    }

    let store = SqliteSecretStore::open(path_str);
    let secrets = store.list_by_company("co-1");
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets[0].name, "API_KEY");
    assert_eq!(secrets[0].provider, "local");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_secrets_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteSecretStore::open(path.to_str().unwrap());

    store.create("co-1", new_secret("API_KEY", "super-secret-value"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
