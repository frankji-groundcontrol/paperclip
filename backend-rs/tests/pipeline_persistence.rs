use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqlitePipelineStore;
use paperclip_backend::pipelines::{CreatePipeline, PipelineRepository, UpdatePipeline};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-pipeline-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_pipeline(key: &str, name: &str) -> CreatePipeline {
    CreatePipeline {
        key: key.to_string(),
        name: name.to_string(),
        description: None,
        project_id: None,
        enforce_transitions: false,
    }
}

/// Pipelines persist across instances with all core fields preserved.
#[test]
fn pipeline_persists_across_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqlitePipelineStore::open(path_str);
        store.create(
            "co-1",
            CreatePipeline {
                description: Some("intake flow".to_string()),
                project_id: Some("proj-1".to_string()),
                enforce_transitions: true,
                ..new_pipeline("intake", "Intake")
            },
        );
    }

    let store = SqlitePipelineStore::open(path_str);
    let list = store.list_by_company("co-1");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].key, "intake");
    assert_eq!(list[0].description.as_deref(), Some("intake flow"));
    assert_eq!(list[0].project_id.as_deref(), Some("proj-1"));
    assert!(list[0].enforce_transitions);
    assert!(!list[0].archived);

    std::fs::remove_file(&path).ok();
}

/// Company scoping + get-by-id hold at the persistence layer.
#[test]
fn persisted_pipelines_scoped_and_gettable() {
    let path = temp_db_path();
    let store = SqlitePipelineStore::open(path.to_str().unwrap());

    let created = store.create("co-1", new_pipeline("intake", "Intake"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.get("co-1", &created.id).unwrap().key, "intake");
    assert!(store.get("co-2", &created.id).is_none());

    std::fs::remove_file(&path).ok();
}

/// Update (archive) round-trips through the database.
#[test]
fn persisted_pipeline_update() {
    let path = temp_db_path();
    let store = SqlitePipelineStore::open(path.to_str().unwrap());

    let created = store.create("co-1", new_pipeline("intake", "Intake"));
    let updated = store
        .update(
            "co-1",
            &created.id,
            UpdatePipeline {
                name: None,
                description: None,
                enforce_transitions: None,
                archived: Some(true),
            },
        )
        .expect("pipeline exists");
    assert!(updated.archived);
    assert_eq!(updated.key, "intake");

    // Re-open: the archive flag persisted.
    let store = SqlitePipelineStore::open(path.to_str().unwrap());
    assert!(store.get("co-1", &created.id).unwrap().archived);

    std::fs::remove_file(&path).ok();
}
