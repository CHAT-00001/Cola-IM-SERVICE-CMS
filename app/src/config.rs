use serde::Deserialize;

/// ---------------------------
/// 应用全局配置
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub name: String,
    pub debug: bool,
    pub signature: String,
    pub enable_protobuf: bool,
}

/// ---------------------------
/// API / WS / Health / Gateway
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ws {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Health {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Gateway {
    pub host: String,
    pub port: u16,
}

/// ---------------------------
/// PostgreSQL 配置
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct Pg {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: Option<u32>,

}

/// ---------------------------
/// Redis 配置
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub db: Option<u32>,
}

/// ---------------------------
/// MongoDB 配置
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct Mongodb {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: String,
}

/// ---------------------------
/// 全局 AppConfig
/// ---------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub api: Api,
    pub ws: Ws,
    pub health: Health,
    pub gateway: Gateway,
    pub pg: Pg,
    pub redis: Redis,
    pub mongodb: Mongodb,
}
