// repo_adapter/src/user/friend/list.rs
// 🔌 插头 - 可乐用户 - 朋友 - 列表
// 2026/8/6 解耦: 朋友列表/配置

////////

use anyhow::Result;
use cola_data::cola_user::info::config::UserConfigInfo;

////////

/// # [ADAPTER] - 获取朋友配置
pub async fn get_config(_user_id: i64, // 用户ID
) -> Result<UserConfigInfo> {
    // 🚧 TODO: 对接 FriendService
    Err(anyhow::anyhow!("not implemented"))
}

//////// END
