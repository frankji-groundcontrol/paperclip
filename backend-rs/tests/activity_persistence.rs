use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::activity::{ActivityFilters, ActivityRepository, CreateActivity};
use paperclip_backend::db::SqliteActivityStore;
use serde_json::json;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-activity-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn event(action: &str, entity_type: &str, entity_id: &str) -> CreateActivity {
    CreateActivity {
        actor_type: "system".to_string(),
        actor_id: "u-1".to_string(),
        action: action.to_string(),
        entity_type: entity_type.to_string(),
        entity_id: entity_id.to_string(),
        agent_id: None,
        details: None,
    }
}

/// Events persist across instances, with the JSON `details` round-tripping.
#[test]
fn activity_persists_across_instances_with_details() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteActivityStore::open(path_str);
        store.create(
            "co-1",
            CreateActivity {
                details: Some(json!({ "title": "Fix bug" })),
                ..event("issue.created", "issue", "i-1")
            },
        );
    }

    let store = SqliteActivityStore::open(path_str);
    let events = store.list("co-1", &ActivityFilters::default());
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "issue.created");
    assert_eq!(events[0].details, Some(json!({ "title": "Fix bug" })));

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_activity_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteActivityStore::open(path.to_str().unwrap());

    store.create("co-1", event("a", "issue", "i-1"));

    assert!(store.list("co-2", &ActivityFilters::default()).is_empty());
    assert_eq!(store.list("co-1", &ActivityFilters::default()).len(), 1);

    std::fs::remove_file(&path).ok();
}

/// Filters and limit apply at the persistence layer, newest-first.
#[test]
fn persisted_activity_filters_and_limit() {
    let path = temp_db_path();
    let store = SqliteActivityStore::open(path.to_str().unwrap());

    store.create("co-1", event("a", "issue", "i-1"));
    store.create("co-1", event("b", "goal", "g-1"));
    store.create("co-1", event("c", "issue", "i-2"));

    let only_goals = store.list(
        "co-1",
        &ActivityFilters {
            entity_type: Some("goal".to_string()),
            ..Default::default()
        },
    );
    assert_eq!(only_goals.len(), 1);
    assert_eq!(only_goals[0].entity_type, "goal");

    let limited = store.list(
        "co-1",
        &ActivityFilters {
            limit: Some(2),
            ..Default::default()
        },
    );
    assert_eq!(limited.len(), 2);
    // Newest-first: the most recently inserted event comes first.
    assert_eq!(limited[0].action, "c");

    std::fs::remove_file(&path).ok();
}
