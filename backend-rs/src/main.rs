use paperclip_backend::app;

/// Composition root: binds a listener and serves the tested `app()` router.
/// Behavior lives in `app()` (see src/lib.rs) and is covered by tests/.
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
    axum::serve(listener, app()).await.expect("server error");
}
