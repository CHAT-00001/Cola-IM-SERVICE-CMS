// cola_three/src/model/dynamic/three_vendor.rs  -- 厂商命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [CMD] - 新增/更新厂商
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorCommand {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}

impl From<VendorCommand> for cola_data::three::command::three_vendor::UpsertVendorCommand {
    fn from(cmd: VendorCommand) -> Self {
        Self {
            id: cmd.id,
            code: cmd.code,
            name: cmd.name,
            sort: cmd.sort,
            status: cmd.status,
        }
    }
}
