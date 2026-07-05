use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use paperclip_backend::auth::AgentKeyStore;
use paperclip_backend::{app_with, Repositories};
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let request = match body {
        Some(b) => builder
            .header("content-type", "application/json")
            .body(Body::from(b.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    };
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, json)
}

async fn create(app: &Router, body: Value) -> Value {
    let (status, s) = send(app, "POST", "/api/companies/co-1/skills", None, Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);
    s
}

// Ports server/src/routes/company-skills.ts core: a company's skill library
// (create/list/filter/get + star counter). Versions/comments/catalog deferred.
#[tokio::test]
async fn creates_skill_with_defaults() {
    let app = app_with(Repositories::default());
    let s = create(&app, json!({ "key": "pdf-tools", "name": "PDF Tools" })).await;

    assert_eq!(s["key"], "pdf-tools");
    assert_eq!(s["name"], "PDF Tools");
    assert_eq!(s["companyId"], "co-1");
    assert_eq!(s["starCount"], 0);
    assert_eq!(s["categories"], json!([]));
    assert_eq!(s["description"], Value::Null);
    assert!(s["id"].as_str().is_some_and(|x| !x.is_empty()));
}

#[tokio::test]
async fn lists_and_filters_skills() {
    let app = app_with(Repositories::default());
    create(
        &app,
        json!({ "key": "pdf", "name": "PDF Tools", "categories": ["docs"] }),
    )
    .await;
    create(
        &app,
        json!({ "key": "sql", "name": "SQL Helper", "categories": ["data"] }),
    )
    .await;

    let (status, all) = send(&app, "GET", "/api/companies/co-1/skills", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(all.as_array().unwrap().len(), 2);

    let (_, docs) = send(
        &app,
        "GET",
        "/api/companies/co-1/skills?category=data",
        None,
        None,
    )
    .await;
    assert_eq!(docs.as_array().unwrap().len(), 1);
    assert_eq!(docs[0]["key"], "sql");

    let (_, q) = send(&app, "GET", "/api/companies/co-1/skills?q=pdf", None, None).await;
    assert_eq!(q.as_array().unwrap().len(), 1);
    assert_eq!(q[0]["key"], "pdf");
}

#[tokio::test]
async fn get_unknown_skill_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(&app, "GET", "/api/companies/co-1/skills/nope", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Skill not found" }));
}

#[tokio::test]
async fn stars_and_unstars_a_skill() {
    let app = app_with(Repositories::default());
    let s = create(&app, json!({ "key": "pdf", "name": "PDF Tools" })).await;
    let id = s["id"].as_str().unwrap();

    let (status, starred) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/skills/{id}/star"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(starred["starCount"], 1);

    let (_, twice) = send(
        &app,
        "POST",
        &format!("/api/companies/co-1/skills/{id}/star"),
        None,
        None,
    )
    .await;
    assert_eq!(twice["starCount"], 2);

    let (status, unstarred) = send(
        &app,
        "DELETE",
        &format!("/api/companies/co-1/skills/{id}/star"),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(unstarred["starCount"], 1);
}

#[tokio::test]
async fn star_unknown_skill_is_404() {
    let app = app_with(Repositories::default());
    let (status, body) = send(
        &app,
        "POST",
        "/api/companies/co-1/skills/nope/star",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({ "error": "Skill not found" }));
}

#[tokio::test]
async fn agent_cannot_access_other_company_skills() {
    let keys = AgentKeyStore::default();
    keys.insert("k", "co-1");
    let app = app_with(Repositories {
        agent_keys: keys,
        ..Default::default()
    });
    let (status, body) = send(&app, "GET", "/api/companies/co-2/skills", Some("k"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, json!({ "error": "forbidden" }));
}
