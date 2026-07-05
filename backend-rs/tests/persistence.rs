use std::sync::atomic::{AtomicU32, Ordering};

use paperclip_backend::companies::{CompanyRepository, CreateCompany};
use paperclip_backend::db::SqliteCompanyStore;

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_db_path() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "paperclip-test-{}-{}.sqlite",
        std::process::id(),
        n
    ))
}

fn new_company(name: &str) -> CreateCompany {
    CreateCompany {
        name: name.to_string(),
        description: None,
        budget_monthly_cents: 0,
    }
}

/// The point of persistence: data written by one store instance is visible to a
/// fresh instance opened on the same database file.
#[test]
fn company_data_persists_across_store_instances() {
    let path = temp_db_path();
    let path_str = path.to_str().unwrap();

    {
        let store = SqliteCompanyStore::open(path_str);
        store.create(new_company("Acme"));
    }

    let store = SqliteCompanyStore::open(path_str);
    let companies = store.list();
    assert_eq!(companies.len(), 1);
    assert_eq!(companies[0].name, "Acme");

    std::fs::remove_file(&path).ok();
}

#[test]
fn gets_persisted_company_by_id_or_none() {
    let path = temp_db_path();
    let store = SqliteCompanyStore::open(path.to_str().unwrap());

    let created = store.create(new_company("Beta"));

    assert_eq!(
        store.get(&created.id).map(|c| c.name),
        Some("Beta".to_string())
    );
    assert!(store.get("does-not-exist").is_none());

    std::fs::remove_file(&path).ok();
}
