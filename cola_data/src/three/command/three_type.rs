// cola_data/src/three/command/three_type.rs  -- THREE - 类型命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [COMMAND] - 新增/更新服务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertTypeCommand {
    pub id: Option<i64>,        // None=新增, Some=更新
    pub code: String,
    pub name: String,
    pub sort: i16,
    pub status: i16,
}
