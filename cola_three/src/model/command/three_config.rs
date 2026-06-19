// cola_three/src/model/command/three_config.rs  -- 配置命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [CMD] - 新增/更新第三方服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCommand {
    pub id: Option<i64>,
    pub type_id: i64,
    pub vendor_id: i64,
    pub name: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: String,
    pub config_json: Option<serde_json::Value>,
    pub remark: Option<String>,
    pub status: i16,
}

impl From<ConfigCommand> for cola_data::three::command::three_config::UpsertConfigCommand {
    fn from(cmd: ConfigCommand) -> Self {
        Self {
            id: cmd.id,
            type_id: cmd.type_id,
            vendor_id: cmd.vendor_id,
            name: cmd.name,
            bucket: cmd.bucket,
            access_key: cmd.access_key,
            secret_key: cmd.secret_key,
            endpoint: cmd.endpoint,
            region: cmd.region,
            config_json: cmd.config_json,
            remark: cmd.remark,
            status: cmd.status,
        }
    }
}
