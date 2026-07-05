//! `paperclip` — CLI for agent users: form a company and run jobs on it.
//! Wraps the broker's `/api/paperclip/*` endpoints; auth via `PAPERCLIP_API_KEY`.
//! Config: `PAPERCLIP_SERVER` (default http://127.0.0.1:8787), `PAPERCLIP_API_KEY`.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::time::{sleep, Duration};

struct LoginMaterial {
    device_secret: String,
    user_code: String,
    prefix: String,
    full_key: String,
}

fn login_material(
    device_secret: String,
    user_code: String,
    prefix: String,
    key_secret: String,
) -> LoginMaterial {
    let full_key = format!("paperclip_{prefix}_{key_secret}");
    LoginMaterial {
        device_secret,
        user_code,
        prefix,
        full_key,
    }
}

struct LoginOptions {
    name: Option<String>,
    team_id: Option<String>,
    server: String,
}

fn parse_login_options(args: &[String]) -> anyhow::Result<LoginOptions> {
    let mut name = None;
    let mut team_id = None;
    let mut server_url = server();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                let value = args
                    .get(i + 1)
                    .filter(|value| !value.starts_with("--"))
                    .ok_or_else(|| anyhow::anyhow!("missing --name <device>"))?;
                name = Some(value.clone());
                i += 2;
            }
            "--team" => {
                let value = args
                    .get(i + 1)
                    .filter(|value| !value.starts_with("--"))
                    .ok_or_else(|| anyhow::anyhow!("missing --team <TEAM_ID>"))?;
                team_id = Some(value.clone());
                i += 2;
            }
            "--server" => {
                let value = args
                    .get(i + 1)
                    .filter(|value| !value.starts_with("--"))
                    .ok_or_else(|| anyhow::anyhow!("missing --server URL"))?;
                server_url = value.clone();
                i += 2;
            }
            other => anyhow::bail!("unknown login option {other}"),
        }
    }
    Ok(LoginOptions {
        name,
        team_id,
        server: server_url,
    })
}

fn sha256_hex(input: &str) -> String {
    hex::encode(Sha256::digest(input.as_bytes()))
}

fn login_start_body(material: &LoginMaterial, name: Option<&str>, team_id: Option<&str>) -> Value {
    let pending_key_name = name.unwrap_or("paperclip CLI");
    let mut body = json!({
        "secretHash": sha256_hex(&material.device_secret),
        "userCodeHash": sha256_hex(&material.user_code),
        "pendingKeyPrefix": material.prefix,
        "pendingKeyHash": sha256_hex(&material.full_key),
        "pendingKeyName": pending_key_name,
        "requestedAccess": "team",
    });
    if let Some(name) = name {
        body["deviceName"] = json!(name);
    }
    if let Some(team_id) = team_id {
        body["teamId"] = json!(team_id);
    }
    body
}

fn random_url_secret() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn random_hex(byte_len: usize) -> String {
    let mut bytes = vec![0_u8; byte_len];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn generate_login_material() -> LoginMaterial {
    login_material(
        random_url_secret(),
        random_hex(4).to_uppercase(),
        format!("pc_{}", random_hex(4)),
        random_url_secret(),
    )
}

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

fn save_key(key: &str) -> anyhow::Result<std::path::PathBuf> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&path, json!({ "api_key": key }).to_string())?;
    Ok(path)
}

fn api_key() -> anyhow::Result<String> {
    std::env::var("PAPERCLIP_API_KEY")
        .ok()
        .filter(|k| !k.trim().is_empty())
        .or_else(stored_key)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no API key: set PAPERCLIP_API_KEY or run `paperclip config set-key <KEY>`"
            )
        })
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
            value
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("error")
        );
    }
    Ok(value)
}

async fn post_public(
    client: &reqwest::Client,
    base: &str,
    path: &str,
    body: Value,
) -> anyhow::Result<Value> {
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let resp = client.post(url).json(&body).send().await?;
    let status = resp.status();
    let value: Value = resp.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        anyhow::bail!(
            "request failed ({}): {}",
            status,
            value
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("error")
        );
    }
    Ok(value)
}

async fn login(options: LoginOptions) -> anyhow::Result<()> {
    const POLL_ATTEMPTS: usize = 150;

    let material = generate_login_material();
    let client = reqwest::Client::new();
    let start_body = login_start_body(
        &material,
        options.name.as_deref(),
        options.team_id.as_deref(),
    );
    post_public(&client, &options.server, "/api/cli/start", start_body).await?;

    println!("Approve this device with code: {}", material.user_code);
    println!(
        "Run `paperclip approve {}` while signed in, or open the web approve page. Waiting...",
        material.user_code
    );

    let secret_hash = sha256_hex(&material.device_secret);
    for attempt in 0..POLL_ATTEMPTS {
        let poll = post_public(
            &client,
            &options.server,
            "/api/cli/poll",
            json!({ "secretHash": secret_hash }),
        )
        .await?;
        match poll.get("status").and_then(Value::as_str) {
            Some("approved") => {
                if poll.get("prefix").and_then(Value::as_str) != Some(material.prefix.as_str()) {
                    anyhow::bail!("approved device login returned a different key prefix");
                }
                let path = save_key(&material.full_key)?;
                println!("login complete; saved key to {}", path.display());
                return Ok(());
            }
            Some("expired") | Some("cancelled") => {
                anyhow::bail!(
                    "device login {}",
                    poll["status"].as_str().unwrap_or("failed")
                );
            }
            _ => {
                if attempt + 1 < POLL_ATTEMPTS {
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    anyhow::bail!("device login timed out");
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  paperclip login [--name <device>] [--team <TEAM_ID>] [--server URL]\n  \
paperclip config set-key <API_KEY>\n  paperclip company create \"<name>\"\n  \
paperclip company list\n  paperclip job run --company <ID> \"<prompt>\" [--model <M>]\n  \
paperclip job list --company <ID>"
    );
    std::process::exit(2);
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
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
        Some("login") => {
            let options = parse_login_options(&args[1..])?;
            login(options).await?;
        }
        Some("config") if args.get(1).map(String::as_str) == Some("set-key") => {
            let key = args
                .get(2)
                .ok_or_else(|| anyhow::anyhow!("missing API key"))?;
            let path = save_key(key)?;
            println!("saved key to {}", path.display());
        }
        Some("company") => match args.get(1).map(String::as_str) {
            Some("create") => {
                let name = args
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("missing company name"))?;
                let v = call(
                    "POST",
                    "/api/paperclip/companies",
                    Some(json!({ "name": name })),
                )
                .await?;
                println!(
                    "{}",
                    v.get("companyId").and_then(Value::as_str).unwrap_or("")
                );
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
                    anyhow::bail!(
                        "expected exactly one prompt argument, got {}",
                        positionals.len()
                    );
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
                let v = call(
                    "GET",
                    &format!("/api/paperclip/companies/{company}/jobs"),
                    None,
                )
                .await?;
                println!("{}", serde_json::to_string_pretty(&v)?);
            }
            _ => usage(),
        },
        _ => usage(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_start_body_hashes_only_plaintext_and_uses_camel_case_keys() {
        let material = LoginMaterial {
            device_secret: "device-secret".to_string(),
            user_code: "A1B2C3D4".to_string(),
            prefix: "pc_12345678".to_string(),
            full_key: "paperclip_pc_12345678_key-secret".to_string(),
        };

        let body = login_start_body(&material, Some("Frank laptop"), Some("team-1"));

        assert_eq!(body["secretHash"], sha256_hex("device-secret"));
        assert_eq!(body["userCodeHash"], sha256_hex("A1B2C3D4"));
        assert_eq!(body["pendingKeyPrefix"], "pc_12345678");
        assert_eq!(
            body["pendingKeyHash"],
            sha256_hex("paperclip_pc_12345678_key-secret")
        );
        assert_eq!(body["pendingKeyName"], "Frank laptop");
        assert_eq!(body["deviceName"], "Frank laptop");
        assert_eq!(body["requestedAccess"], "team");
        assert_eq!(body["teamId"], "team-1");

        let encoded = body.to_string();
        assert!(!encoded.contains("device-secret"));
        assert!(!encoded.contains("A1B2C3D4"));
        assert!(!encoded.contains("key-secret"));
        assert!(!encoded.contains("paperclip_pc_12345678_key-secret"));
    }

    #[test]
    fn parse_login_options_reads_name_team_and_server_flags() {
        let args = vec![
            "--name".to_string(),
            "Frank laptop".to_string(),
            "--team".to_string(),
            "team-1".to_string(),
            "--server".to_string(),
            "http://localhost:3100".to_string(),
        ];

        let options = parse_login_options(&args).unwrap();

        assert_eq!(options.name.as_deref(), Some("Frank laptop"));
        assert_eq!(options.team_id.as_deref(), Some("team-1"));
        assert_eq!(options.server, "http://localhost:3100");
    }

    #[test]
    fn login_material_formats_full_local_key_from_parts() {
        let material = login_material(
            "device-secret".to_string(),
            "A1B2C3D4".to_string(),
            "pc_12345678".to_string(),
            "key-secret".to_string(),
        );

        assert_eq!(material.device_secret, "device-secret");
        assert_eq!(material.user_code, "A1B2C3D4");
        assert_eq!(material.prefix, "pc_12345678");
        assert_eq!(material.full_key, "paperclip_pc_12345678_key-secret");
    }
}
