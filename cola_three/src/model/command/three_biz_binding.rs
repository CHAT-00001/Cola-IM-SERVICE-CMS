// cola_three/src/model/command/three_biz_binding.rs  -- 绑定命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [CMD] - 新增/更新业务绑定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingCommand {
    pub id: Option<i64>,
    pub three_config_id: i64,
    pub biz_module: String,
    pub biz_type: String,
    pub status: i16,
}

impl From<BindingCommand> for cola_data::three::command::three_biz_binding::UpsertBindingCommand {
    fn from(cmd: BindingCommand) -> Self {
        Self {
            id: cmd.id,
            three_config_id: cmd.three_config_id,
            biz_module: cmd.biz_module,
            biz_type: cmd.biz_type,
            status: cmd.status,
        }
    }
}
