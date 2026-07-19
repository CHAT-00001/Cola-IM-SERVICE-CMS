// cola_auth/src/kits/token.rs  -- 可乐验证中心 - Token 生成工具
// 2026/6/18

////////

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # JWT 载荷（含设备 ID，支持多端登录）
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i64,        // 用户 ID (uid)
    pub device_id: String, // 设备唯一标识
    pub exp: usize,      // 过期时间 (秒级时间戳)
    pub iat: usize,      // 签发时间
    pub jti: String,     // JWT ID (防重放)
}

/// # 生产级签名密钥
fn get_jwt_secret() -> Vec<u8> {
    b"cola_cms_jwt_secret_key_2026_secure_v2!@#".to_vec()
}

////////

/// # 1. [KITS] - 生成 access_token (JWT)，带 device_id
/// * 有效期: 15 分钟
pub fn kit_generate_access_token(uid: i64, device_id: &str) -> Result<(String, i64), anyhow::Error> {
    let now = Utc::now();
    let exp = now + Duration::minutes(15);
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
        &EncodingKey::from_secret(&get_jwt_secret()),
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
        &EncodingKey::from_secret(&get_jwt_secret()),
    )?;

    Ok((token, exp.timestamp()))
}

////////

/// # 3. [KITS] - 生成 refresh_token (安全随机令牌)
/// * 有效期: 30 天
/// * 方案: 32 字节随机数 + 时间戳 + UUID，高熵防猜测
pub fn kit_generate_refresh_token() -> Result<(String, i64), anyhow::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(30);
    let exp_ts = exp.timestamp();

    let mut random_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut random_bytes);

    let raw = format!(
        "{}{:08x}{}",
        hex_encode(&random_bytes),
        now.timestamp(),
        Uuid::new_v4().to_string().replace("-", "")
    );

    Ok((raw, exp_ts))
}

////////

/// # 4. [KITS] - 生成 refresh_token 的哈希（用于入库校验）
pub fn kit_hash_refresh_token(raw: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    raw.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

////////

/// # 5. [KITS] - 构建 SessionCommand（统一组装 token + 过期时间，带 device_id）
pub fn kit_build_session_cmd(
    uid: i64,
    phone: &str,
    platform: &str,
    device_id: &str,
) -> Result<(cola_data::auth::command::session::SessionCommand, String, String), anyhow::Error> {
    let (access_token, access_exp) = kit_generate_access_token(uid, device_id)?;
    let (refresh_token_raw, refresh_exp) = kit_generate_refresh_token()?;

    let cmd = cola_data::auth::command::session::SessionCommand {
        access_token: access_token.clone(),
        refresh_token: kit_hash_refresh_token(&refresh_token_raw),
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

/// 辅助: hex 编码
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

//////// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token() {
        let (token, exp) = kit_generate_access_token(10086, "test_device").unwrap();
        assert!(!token.is_empty());
        assert!(exp > Utc::now().timestamp());
        println!("Access Token ({} chars): {}...", token.len(), &token[..20]);
    }

    #[test]
    fn test_refresh_token() {
        let (token, exp) = kit_generate_refresh_token().unwrap();
        assert!(!token.is_empty());
        assert!(exp > Utc::now().timestamp());
        println!("Refresh Token ({} chars): {}...", token.len(), &token[..20]);
        println!("Hash: {}", kit_hash_refresh_token(&token));
    }
}
//////// END
