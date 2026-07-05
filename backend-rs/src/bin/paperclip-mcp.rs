//! `paperclip-mcp` — stdio MCP server exposing Paperclip tools to agents
//! (Claude Code / Codex / Hermes). Wraps the broker `/api/paperclip/*` endpoints.
//! Auth via `PAPERCLIP_API_KEY`; target via `PAPERCLIP_SERVER`.
//!
//! Minimal but complete JSON-RPC 2.0 over newline-delimited stdio:
//! `initialize` -> serverInfo + tools capability; `tools/list`; `tools/call`;
//! notifications (e.g. `notifications/initialized`) are accepted and ignored.

use std::io::{BufRead, Write};

use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2024-11-05";

fn server() -> String {
    std::env::var("PAPERCLIP_SERVER").unwrap_or_else(|_| "http://127.0.0.1:8787".to_string())
}
fn api_key() -> Option<String> {
    std::env::var("PAPERCLIP_API_KEY").ok().filter(|k| !k.trim().is_empty())
}

fn tools() -> Value {
    json!([
        { "name": "paperclip_create_company",
          "description": "Form a Paperclip company (a team-scoped workspace). Returns { companyId }.",
          "inputSchema": { "type": "object", "properties": { "name": { "type": "string" } }, "required": ["name"] } },
        { "name": "paperclip_list_companies",
          "description": "List the companies visible to your API key's team.",
          "inputSchema": { "type": "object", "properties": {} } },
        { "name": "paperclip_run_job",
          "description": "Run a job on a company: an LLM-backed task. Returns { jobId, status, result, usage }.",
          "inputSchema": { "type": "object", "properties": {
              "companyId": { "type": "string" }, "prompt": { "type": "string" }, "model": { "type": "string" } },
              "required": ["companyId", "prompt"] } },
        { "name": "paperclip_list_jobs",
          "description": "List jobs for a company. Returns { jobs }.",
          "inputSchema": { "type": "object", "properties": { "companyId": { "type": "string" } }, "required": ["companyId"] } }
    ])
}

async fn http(method: &str, path: &str, body: Option<Value>) -> anyhow::Result<Value> {
    let key = api_key().ok_or_else(|| anyhow::anyhow!("PAPERCLIP_API_KEY is not set"))?;
    let client = reqwest::Client::new();
    let url = format!("{}{}", server(), path);
    let mut req = if method == "POST" { client.post(url) } else { client.get(url) }.bearer_auth(key);
    if let Some(body) = body {
        req = req.json(&body);
    }
    let resp = req.send().await?;
    let status = resp.status();
    let value: Value = resp.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        anyhow::bail!("paperclip request failed ({status})");
    }
    Ok(value)
}

async fn call_tool(name: &str, args: &Value) -> anyhow::Result<Value> {
    match name {
        "paperclip_create_company" => {
            let n = args.get("name").cloned().unwrap_or(Value::Null);
            http("POST", "/api/paperclip/companies", Some(json!({ "name": n }))).await
        }
        "paperclip_list_companies" => http("GET", "/api/paperclip/companies", None).await,
        "paperclip_run_job" => {
            let company = args.get("companyId").and_then(Value::as_str).unwrap_or("");
            let mut body = json!({ "prompt": args.get("prompt").cloned().unwrap_or(Value::Null) });
            if let Some(m) = args.get("model") {
                body["model"] = m.clone();
            }
            http("POST", &format!("/api/paperclip/companies/{company}/jobs"), Some(body)).await
        }
        "paperclip_list_jobs" => {
            let company = args.get("companyId").and_then(Value::as_str).unwrap_or("");
            http("GET", &format!("/api/paperclip/companies/{company}/jobs"), None).await
        }
        other => anyhow::bail!("unknown tool: {other}"),
    }
}

fn reply(id: &Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}
fn error(id: &Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");
        let id = req.get("id").cloned();
        // Notifications have no id → never reply.
        let response: Option<Value> = match (method, id.clone()) {
            ("initialize", Some(id)) => Some(reply(
                &id,
                json!({
                    "protocolVersion": PROTOCOL_VERSION,
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "paperclip-mcp", "version": env!("CARGO_PKG_VERSION") }
                }),
            )),
            ("tools/list", Some(id)) => Some(reply(&id, json!({ "tools": tools() }))),
            ("tools/call", Some(id)) => {
                let params = req.get("params").cloned().unwrap_or(Value::Null);
                let name = params.get("name").and_then(Value::as_str).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                match rt.block_on(call_tool(name, &args)) {
                    Ok(v) => Some(reply(
                        &id,
                        json!({ "content": [{ "type": "text", "text": serde_json::to_string(&v).unwrap_or_default() }] }),
                    )),
                    Err(e) => Some(error(&id, -32000, &e.to_string())),
                }
            }
            (_, Some(id)) => Some(error(&id, -32601, "method not found")),
            (_, None) => None, // notification: accept and ignore
        };
        if let Some(response) = response {
            let _ = writeln!(stdout, "{response}");
            let _ = stdout.flush();
        }
    }
}
