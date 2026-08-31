// cola_port/src/music/collect/manage.rs
// ⏩️ 端口 - MUSIC - 收藏 - mod
// 2026/8/24 11:41 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC COLLECT MANAGE PORTS] - 音乐 收藏 管理 端口
/// * `desc`: `收藏记录列表端口`
#[async_trait::async_trait]
pub trait MusicCollectManagePort: Send + Sync {
    //

    /// # 1. [PORT] - 用户的
    /// * `desc`: `根据用户ID获取收藏记录列表`
    async fn get_admin_list(
        &self,
        uid: i64,              // 操作者 ID
        user_id: Option<i64>,  // 用户 ID
        music_id: Option<i64>, // 音乐 ID
        limit: i64,            // 数量
        offset: i64,           // 页码
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐收藏新增适配器尚未装配"))
    }
}


//////// END
