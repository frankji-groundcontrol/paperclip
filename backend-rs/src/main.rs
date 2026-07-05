use paperclip_backend::app_from_env;

/// Composition root: binds a listener and serves `app_from_env()`, which wires the
/// real Supabase (auth + data) and OpenAI gateways when SUPABASE_*/OPENAI_* env is
/// present, and falls back to disabled/in-memory otherwise. Behavior lives in the
/// library (see src/lib.rs) and is covered by tests/.
#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3100);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");
    println!("paperclip-backend listening on http://{addr}");
    axum::serve(listener, app_from_env())
        .await
        .expect("server error");
}
