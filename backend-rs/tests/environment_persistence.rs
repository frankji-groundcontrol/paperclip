use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteEnvironmentStore;
use paperclip_backend::environments::{
    CreateEnvironment, EnvironmentRepository, UpdateEnvironment,
};
use serde_json::json;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!("paperclip-env-{}-{}.sqlite", std::process::id(), n))
}

fn new_env(name: &str) -> CreateEnvironment {
    CreateEnvironment {
        name: name.to_string(),
        description: None,
        driver: "local".to_string(),
        status: "active".to_string(),
        config: json!({}),
        env_vars: json!({}),
        metadata: None,
    }
}

/// Environments persist across instances, with JSON config/description preserved.
#[test]
fn environment_persists_across_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteEnvironmentStore::open(path_str);
        store.create(
            "co-1",
            CreateEnvironment {
                description: Some("primary".to_string()),
                config: json!({ "image": "ubuntu" }),
                ..new_env("prod")
            },
        );
    }

    let store = SqliteEnvironmentStore::open(path_str);
    let envs = store.list_by_company("co-1");
    assert_eq!(envs.len(), 1);
    assert_eq!(envs[0].name, "prod");
    assert_eq!(envs[0].driver, "local");
    assert_eq!(envs[0].description.as_deref(), Some("primary"));
    assert_eq!(envs[0].config, json!({ "image": "ubuntu" }));

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_environments_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteEnvironmentStore::open(path.to_str().unwrap());

    store.create("co-1", new_env("prod"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}

/// Update and delete round-trip through the database.
#[test]
fn persisted_environments_update_and_delete() {
    let path = temp_db_path();
    let store = SqliteEnvironmentStore::open(path.to_str().unwrap());

    let created = store.create("co-1", new_env("prod"));
    let updated = store
        .update(
            "co-1",
            &created.id,
            UpdateEnvironment {
                name: None,
                description: None,
                driver: None,
                status: Some("archived".to_string()),
                config: None,
                env_vars: None,
                metadata: None,
            },
        )
        .expect("environment exists");
    assert_eq!(updated.status, "archived");
    assert_eq!(updated.name, "prod");

    assert!(store.delete("co-1", &created.id));
    assert!(store.list_by_company("co-1").is_empty());

    std::fs::remove_file(&path).ok();
}
