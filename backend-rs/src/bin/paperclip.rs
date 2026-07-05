//! `paperclip` — CLI for agent users: form a company and run jobs on it.
//! Wraps the broker's `/api/paperclip/*` endpoints; auth via `PAPERCLIP_API_KEY`.
//! Config: `PAPERCLIP_SERVER` (default http://127.0.0.1:8787), `PAPERCLIP_API_KEY`.

use serde_json::{json, Value};

fn server() -> String {
    std::env::var("PAPERCLIP_SERVER").unwrap_or_else(|_| "http://127.0.0.1:8787".to_string())
}

fn config_path() -> std::path::PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".config")
        });
    base.join("paperclip").join("credentials.json")
}

fn stored_key() -> Option<String> {
    let raw = std::fs::read_to_string(config_path()).ok()?;
    serde_json::from_str::<Value>(&raw)
        .ok()?
        .get("api_key")?
        .as_str()
        .map(ToString::to_string)
}

fn api_key() -> anyhow::Result<String> {
    std::env::var("PAPERCLIP_API_KEY")
        .ok()
        .filter(|k| !k.trim().is_empty())
        .or_else(stored_key)
        .ok_or_else(|| anyhow::anyhow!("no API key: set PAPERCLIP_API_KEY or run `paperclip config set-key <KEY>`"))
}

async fn call(method: &str, path: &str, body: Option<Value>) -> anyhow::Result<Value> {
    let key = api_key()?;
    let client = reqwest::Client::new();
    let url = format!("{}{}", server(), path);
    let mut req = match method {
        "POST" => client.post(url),
        _ => client.get(url),
    }
    .bearer_auth(key);
    if let Some(body) = body {
        req = req.json(&body);
    }
    let resp = req.send().await?;
    let status = resp.status();
    let value: Value = resp.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        anyhow::bail!(
            "request failed ({}): {}",
            status,
            value.get("error").and_then(Value::as_str).unwrap_or("error")
        );
    }
    Ok(value)
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  paperclip config set-key <API_KEY>\n  paperclip company create \"<name>\"\n  \
paperclip company list\n  paperclip job run --company <ID> \"<prompt>\" [--model <M>]\n  \
paperclip job list --company <ID>"
    );
    std::process::exit(2);
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = run(&args).await;
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

async fn run(args: &[String]) -> anyhow::Result<()> {
    match args.first().map(String::as_str) {
        Some("config") if args.get(1).map(String::as_str) == Some("set-key") => {
            let key = args.get(2).ok_or_else(|| anyhow::anyhow!("missing API key"))?;
            let path = config_path();
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            std::fs::write(&path, json!({ "api_key": key }).to_string())?;
            println!("saved key to {}", path.display());
        }
        Some("company") => match args.get(1).map(String::as_str) {
            Some("create") => {
                let name = args.get(2).ok_or_else(|| anyhow::anyhow!("missing company name"))?;
                let v = call("POST", "/api/paperclip/companies", Some(json!({ "name": name }))).await?;
                println!("{}", v.get("companyId").and_then(Value::as_str).unwrap_or(""));
            }
            Some("list") => {
                let v = call("GET", "/api/paperclip/companies", None).await?;
                println!("{}", serde_json::to_string_pretty(&v)?);
            }
            _ => usage(),
        },
        Some("job") => match args.get(1).map(String::as_str) {
            Some("run") => {
                // Index-based parse: consume --company/--model + their values, treat a lone
                // `--` as end-of-flags, and require exactly one remaining positional (the prompt).
                let sub = &args[2..];
                let mut company: Option<String> = None;
                let mut model: Option<String> = None;
                let mut positionals: Vec<String> = Vec::new();
                let mut i = 0;
                while i < sub.len() {
                    match sub[i].as_str() {
                        "--company" => {
                            if let Some(v) = sub.get(i + 1).filter(|v| !v.starts_with("--")) {
                                company = Some(v.clone());
                                i += 2;
                            } else {
                                i += 1;
                            }
                        }
                        "--model" => {
                            if let Some(v) = sub.get(i + 1).filter(|v| !v.starts_with("--")) {
                                model = Some(v.clone());
                                i += 2;
                            } else {
                                i += 1;
                            }
                        }
                        "--" => {
                            positionals.extend(sub[i + 1..].iter().cloned());
                            break;
                        }
                        _ => {
                            positionals.push(sub[i].clone());
                            i += 1;
                        }
                    }
                }
                let company = company.ok_or_else(|| anyhow::anyhow!("missing --company <ID>"))?;
                if positionals.len() != 1 {
                    anyhow::bail!("expected exactly one prompt argument, got {}", positionals.len());
                }
                let mut body = json!({ "prompt": positionals[0] });
                if let Some(m) = model {
                    body["model"] = json!(m);
                }
                let v = call(
                    "POST",
                    &format!("/api/paperclip/companies/{company}/jobs"),
                    Some(body),
                )
                .await?;
                // print the result text for humans; full JSON on stderr for scripts
                if let Some(result) = v.get("result").and_then(Value::as_str) {
                    println!("{result}");
                } else {
                    println!("{}", serde_json::to_string_pretty(&v)?);
                }
            }
            Some("list") => {
                let company = arg_value(args, "--company")
                    .ok_or_else(|| anyhow::anyhow!("missing --company <ID>"))?;
                let v = call("GET", &format!("/api/paperclip/companies/{company}/jobs"), None).await?;
                println!("{}", serde_json::to_string_pretty(&v)?);
            }
            _ => usage(),
        },
        _ => usage(),
    }
    Ok(())
}
