use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteProjectStore;
use paperclip_backend::projects::{CreateProject, ProjectRepository};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-project-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_project(name: &str) -> CreateProject {
    CreateProject {
        name: name.to_string(),
        status: "backlog".to_string(),
    }
}

/// Projects written by one store instance are visible to a fresh instance opened
/// on the same database file.
#[test]
fn project_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteProjectStore::open(path_str);
        store.create("co-1", new_project("Website"));
    }

    let store = SqliteProjectStore::open(path_str);
    let projects = store.list_by_company("co-1");
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Website");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_projects_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteProjectStore::open(path.to_str().unwrap());

    store.create("co-1", new_project("secret"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
