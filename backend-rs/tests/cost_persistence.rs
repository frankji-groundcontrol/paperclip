use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::costs::{CostRepository, CreateCostEvent};
use paperclip_backend::db::SqliteCostStore;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-cost-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_event(cost_cents: i64) -> CreateCostEvent {
    CreateCostEvent {
        agent_id: "a-1".to_string(),
        issue_id: None,
        provider: "anthropic".to_string(),
        biller: None,
        billing_type: "unknown".to_string(),
        model: "claude".to_string(),
        input_tokens: 0,
        cached_input_tokens: 0,
        output_tokens: 0,
        cost_cents,
        occurred_at: "2026-07-01T00:00:00Z".to_string(),
    }
}

/// Cost events persist across instances; `biller` defaults to `provider`.
#[test]
fn cost_events_persist_across_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteCostStore::open(path_str);
        let created = store.create("co-1", new_event(150));
        assert_eq!(created.biller, "anthropic"); // defaulted from provider
    }

    let store = SqliteCostStore::open(path_str);
    assert_eq!(store.spend_cents("co-1"), 150);

    std::fs::remove_file(&path).ok();
}

/// Spend is summed and company-scoped at the persistence layer.
#[test]
fn persisted_spend_is_summed_and_scoped() {
    let path = temp_db_path();
    let store = SqliteCostStore::open(path.to_str().unwrap());

    store.create("co-1", new_event(150));
    store.create("co-1", new_event(350));
    store.create("co-2", new_event(999));

    assert_eq!(store.spend_cents("co-1"), 500);
    assert_eq!(store.spend_cents("co-2"), 999);
    assert_eq!(store.spend_cents("co-3"), 0);

    std::fs::remove_file(&path).ok();
}
