// cola_auth/src/kits/token.rs
// core - 验证中心 - kits - Token 生成工具
// 2026-07-19 11:10

////////

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # [PAYLOAD] - JWT 载荷（含设备 ID，支持多端登录）
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i64,          // 用户 ID (uid)
    pub device_id: String, // 设备唯一标识
    pub exp: usize,        // 过期时间 (秒级时间戳)
    pub iat: usize,        // 签发时间
    pub jti: String,       // JWT ID (防重放)
}

////////

/// # 生产级签名密钥
/// * `pub`: 供网关 middleware (JwtAuth) 复用同一密钥进行解码验证
pub fn kit_get_jwt_secret() -> Vec<u8> {
    b"cola_cms_jwt_secret_key_2026_secure_v2!@#".to_vec()
}

/// # AES-256 密钥 (32 字节)
/// 生产环境应从环境变量或配置文件读取
fn get_aes_key() -> [u8; 32] {
    let key_str = std::env::var("COLA_AES_KEY")
        .unwrap_or_else(|_| "cola_cms_aes256_key_20260719_v1!!@".to_string());
    let mut key = [0u8; 32];
    let bytes = key_str.as_bytes();
    let len = bytes.len().min(32);
    key[..len].copy_from_slice(&bytes[..len]);
    key
}

////////

/// # 1. [KITS] - 生成 access_token (JWT)，带 device_id
/// * 有效期: 由 `SessionCommand::ACCESS_TOKEN_TTL_DAYS` 统一控制（默认 10 天）
pub fn kit_generate_access_token(uid: i64, device_id: &str) -> Result<(String, i64), anyhow::Error> {
    let now = Utc::now();
    let ttl_days = cola_data::cola_auth::command::session::SessionCommand::ACCESS_TOKEN_TTL_DAYS;
    let exp = now + Duration::days(ttl_days);
    let exp_ts = exp.timestamp() as usize;

    let claims = JwtClaims {
        sub: uid,
        device_id: device_id.to_string(),
        exp: exp_ts,
        iat: now.timestamp() as usize,
        jti: Uuid::new_v4().to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&kit_get_jwt_secret()),
    )?;

    Ok((token, exp.timestamp()))
}

////////

/// # 2. [KITS] - 生成 access_token (JWT) - 自定义过期时间，带 device_id
pub fn kit_generate_access_token_with_ttl(
    uid: i64,
    device_id: &str,
    ttl_minutes: i64,
) -> Result<(String, i64), anyhow::Error> {
    let now = Utc::now();
    let exp = now + Duration::minutes(ttl_minutes);
    let exp_ts = exp.timestamp() as usize;

    let claims = JwtClaims {
        sub: uid,
        device_id: device_id.to_string(),
        exp: exp_ts,
        iat: now.timestamp() as usize,
        jti: Uuid::new_v4().to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&kit_get_jwt_secret()),
    )?;

    Ok((token, exp.timestamp()))
}

////////

/// # 3. [KITS] - 生成 128 位 refresh_token (安全随机令牌)
/// * 有效期: 由 `SessionCommand::REFRESH_TOKEN_TTL_DAYS` 统一控制（默认 180 天）
/// * 方案: 64 字节随机数 → 128 hex 字符
pub fn kit_generate_refresh_token() -> Result<(String, i64), anyhow::Error> {
    let now = Utc::now();
    let ttl_days = cola_data::cola_auth::command::session::SessionCommand::REFRESH_TOKEN_TTL_DAYS;
    let exp = now + Duration::days(ttl_days);
    let exp_ts = exp.timestamp();

    // 64 字节随机数 → 128 hex 字符
    let mut random_bytes = [0u8; 64];
    rand::thread_rng().fill(&mut random_bytes);
    let raw = format!("{}", hex_encode(&random_bytes));

    Ok((raw, exp_ts))
}

////////

/// # 4. [KITS] - AES-256-GCM 加密 refresh_token（用于入库）
/// * 返回 base64 编码的密文（nonce + ciphertext）
pub fn kit_encrypt_refresh_token(raw: &str) -> Result<String, anyhow::Error> {
    let key = get_aes_key();
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("AES key init failed: {}", e))?;

    // 12 字节随机 nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, raw.as_bytes())
        .map_err(|e| anyhow::anyhow!("AES encrypt failed: {}", e))?;

    // nonce + ciphertext 的 base64
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(base64_encode(&combined))
}

////////

/// # 5. [KITS] - AES-256-GCM 解密 refresh_token
/// * 输入: base64 编码的密文
/// * 返回: 明文字符串
pub fn kit_decrypt_refresh_token(encrypted_b64: &str) -> Result<String, anyhow::Error> {
    let key = get_aes_key();
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("AES key init failed: {}", e))?;

    let combined = base64_decode(encrypted_b64)?;
    if combined.len() < 12 {
        return Err(anyhow::anyhow!("Invalid encrypted data: too short"));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("AES decrypt failed: {}", e))?;

    Ok(String::from_utf8(plaintext)
        .map_err(|e| anyhow::anyhow!("Decrypted data not valid UTF-8: {}", e))?)
}

////////

/// # 6. [KITS] - 验证 refresh_token 是否匹配密文
/// * 解密数据库密文后与客户端明文 token 比对（AES-GCM nonce 随机，不能直接比对密文）
pub fn kit_verify_refresh_token(raw: &str, encrypted_b64: &str) -> bool {
    match kit_decrypt_refresh_token(encrypted_b64) {
        Ok(plaintext) => plaintext == raw,
        Err(_) => false,
    }
}

////////

/// # 7. [KITS] - 构建 SessionCommand（统一组装 token + 过期时间，带 device_id）
/// * refresh_token 入库前先 AES-256-GCM 加密
/// * 过期时间: 统一由 `SessionCommand` 配置常量控制，与 JWT 生成保持一致
pub fn kit_build_session_cmd(
    uid: i64,
    phone: &str,
    platform: &str,
    device_id: &str,
) -> Result<(cola_data::cola_auth::command::session::SessionCommand, String, String), anyhow::Error> {
    use cola_data::cola_auth::command::session::SessionCommand;

    let (access_token, access_exp) = kit_generate_access_token(uid, device_id)?;
    let (refresh_token_raw, refresh_exp) = kit_generate_refresh_token()?;

    // AES-256-GCM 加密后入库
    let refresh_token_encrypted = kit_encrypt_refresh_token(&refresh_token_raw)?;

    let cmd = SessionCommand {
        access_token: access_token.clone(),
        refresh_token: refresh_token_encrypted,
        access_expires_at: DateTime::from_timestamp(access_exp, 0)
            .unwrap_or_else(Utc::now),
        refresh_expires_at: DateTime::from_timestamp(refresh_exp, 0)
            .unwrap_or_else(Utc::now),
        last_active_at: Utc::now(),
        client_id: phone.to_string(),
        device_id: device_id.to_string(),
    };

    Ok((cmd, access_token, refresh_token_raw))
}

////////

/// 辅助: hex 编码
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// 辅助: base64 编码
fn base64_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

////////

/// 辅助: base64 解码
fn base64_decode(s: &str) -> Result<Vec<u8>, anyhow::Error> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))
}

////////

//////// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_token_128() {
        let (token, exp) = kit_generate_refresh_token().unwrap();
        assert_eq!(token.len(), 128, "refresh_token must be 128 chars");
        assert!(exp > Utc::now().timestamp());
        println!("Refresh Token ({} chars): {}", token.len(), token);
    }

    #[test]
    fn test_aes_encrypt_decrypt() {
        let raw = "test_refresh_token_1234567890abcdef1234567890abcdef";
        let encrypted = kit_encrypt_refresh_token(raw).unwrap();
        assert!(!encrypted.is_empty());

        let decrypted = kit_decrypt_refresh_token(&encrypted).unwrap();
        assert_eq!(decrypted, raw);
        println!("AES encrypt/decrypt OK: {} -> {} -> {}",
            raw.len(), encrypted.len(), decrypted.len());
    }

    #[test]
    fn test_verify_refresh_token() {
        let raw = "test_refresh_token_1234567890abcdef1234567890abcdef";
        let encrypted = kit_encrypt_refresh_token(raw).unwrap();
        assert!(kit_verify_refresh_token(raw, &encrypted));
        assert!(!kit_verify_refresh_token("wrong_token", &encrypted));
    }
}

//////// END