// cola_data/src/three/entity/sign.rs  -- -- THREE - 第三方登录配置实体
// 2026/6/30 03:10

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 第三方登录服务配置
/// * `table name`: `three_sign`
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct ThreeSignEntity {
    pub id: i64,
    pub type_id: i64,                           // FK → three_type.id
    pub vendor_id: i64,                         // FK → three_vendor.id
    pub name: String,                           // 服务名称，如"微信登录"、"Google登录"
    pub client_id: String,                      // 客户端ID (AppID/ClientId)
    pub client_secret: String,                  // 客户端密钥（加密存储）
    pub redirect_uri: String,                   // 回调地址
    pub scope: String,                          // 权限范围（如 email, profile, openid）
    pub config_json: Option<serde_json::Value>, // 厂商特有配置（如微信的授权类型、公钥、私钥路径等）
    pub remark: Option<String>,                 // 备注
    pub status: i16,                            // 1启用 0禁用
    pub created_at: Option<DateTime<Utc>>,      // 创建时间 - 人类
    pub updated_at: Option<DateTime<Utc>>,      // 更新时间 - 人类
}

/// # 查询字段常量
pub const THREE_SIGN_COLUMNS: &str = r#"
    id, type_id, vendor_id, name, client_id, client_secret,
    redirect_uri, scope, config_json, remark, status, created_at, updated_at
"#;
