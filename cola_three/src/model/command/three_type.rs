// cola_three/src/model/dynamic/three_type.rs  -- 类型命令
// 2026/6/18

use serde::{Deserialize, Serialize};

//////

/// # [CMD] - 新增/更新服务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCommand {
    pub id: Option<i64>, // ID
    pub code: String,    // 代码
    pub name: String,    // 名称
    pub sort: i16,       // 排序
    pub status: i16,     // 默认 1
}


//
impl From<TypeCommand> for cola_data::three::command::three_type::UpsertTypeCommand {
    fn from(cmd: TypeCommand) -> Self {
        Self {
            id: cmd.id,
            code: cmd.code,
            name: cmd.name,
            sort: cmd.sort,
            status: cmd.status,
        }
    }
}
