use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiKeyParts {
    pub prefix: String,
    pub key_hash: String,
}

pub fn parse_api_key(full: &str) -> Option<ApiKeyParts> {
    let rest = full.strip_prefix("paperclip_")?;
    if !rest.starts_with("pc_") {
        return None;
    }

    let secret_separator = rest[3..].find('_').map(|index| index + 3)?;
    if secret_separator == 0 || secret_separator + 1 >= rest.len() {
        return None;
    }

    Some(ApiKeyParts {
        prefix: rest[..secret_separator].to_string(),
        key_hash: hash_full_key(full),
    })
}

pub fn hash_full_key(full: &str) -> String {
    hex::encode(Sha256::digest(full.as_bytes()))
}
