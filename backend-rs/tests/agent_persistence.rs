use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::agents::{AgentRepository, CreateAgent};
use paperclip_backend::db::SqliteAgentStore;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-agent-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_agent(name: &str) -> CreateAgent {
    CreateAgent {
        name: name.to_string(),
        role: "general".to_string(),
        adapter_type: "process".to_string(),
    }
}

/// Agents written by one store instance are visible to a fresh instance opened
/// on the same database file, with server-set status persisted.
#[test]
fn agent_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteAgentStore::open(path_str);
        store.create("co-1", new_agent("Ada"));
    }

    let store = SqliteAgentStore::open(path_str);
    let agents = store.list_by_company("co-1");
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "Ada");
    assert_eq!(agents[0].status, "idle");
    assert_eq!(agents[0].adapter_type, "process");

    std::fs::remove_file(&path).ok();
}

/// Company scoping holds at the persistence layer.
#[test]
fn persisted_agents_are_scoped_by_company() {
    let path = temp_db_path();
    let store = SqliteAgentStore::open(path.to_str().unwrap());

    store.create("co-1", new_agent("secret"));

    assert!(store.list_by_company("co-2").is_empty());
    assert_eq!(store.list_by_company("co-1").len(), 1);

    std::fs::remove_file(&path).ok();
}
