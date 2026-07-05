use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::db::SqliteGoalStore;
use paperclip_backend::goals::{CreateGoal, GoalRepository, UpdateGoal};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-goal-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_goal(title: &str) -> CreateGoal {
    CreateGoal {
        title: title.to_string(),
        description: None,
        level: "task".to_string(),
        status: "planned".to_string(),
        parent_id: None,
        owner_agent_id: None,
    }
}

/// Goals written by one store instance are visible to a fresh instance opened on
/// the same database file, with all core fields round-tripping.
#[test]
fn goal_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteGoalStore::open(path_str);
        store.create(
            "co-1",
            CreateGoal {
                title: "Grow revenue".to_string(),
                description: Some("north star".to_string()),
                level: "company".to_string(),
                status: "active".to_string(),
                parent_id: Some("g-parent".to_string()),
                owner_agent_id: None,
            },
        );
    }

    let store = SqliteGoalStore::open(path_str);
    let goals = store.list_by_company("co-1");
    assert_eq!(goals.len(), 1);
    assert_eq!(goals[0].title, "Grow revenue");
    assert_eq!(goals[0].description.as_deref(), Some("north star"));
    assert_eq!(goals[0].level, "company");
    assert_eq!(goals[0].status, "active");
    assert_eq!(goals[0].parent_id.as_deref(), Some("g-parent"));

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_goals_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteGoalStore::open(path.to_str().unwrap());

    store.create("co-1", new_goal("secret"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}

/// Update and delete round-trip through the database.
#[test]
fn persisted_goals_update_and_delete() {
    let path = temp_db_path();
    let store = SqliteGoalStore::open(path.to_str().unwrap());

    let created = store.create("co-1", new_goal("Ship v2"));
    let updated = store
        .update(
            "co-1",
            &created.id,
            UpdateGoal {
                title: None,
                description: None,
                level: None,
                status: Some("achieved".to_string()),
                parent_id: None,
                owner_agent_id: None,
            },
        )
        .expect("goal exists");
    assert_eq!(updated.status, "achieved");
    assert_eq!(updated.title, "Ship v2");

    assert!(store.delete("co-1", &created.id));
    assert!(store.list_by_company("co-1").is_empty());
    assert!(!store.delete("co-1", &created.id)); // second delete is a no-op

    std::fs::remove_file(&path).ok();
}
