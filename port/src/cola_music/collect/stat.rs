// cola_port/src/music/collec/stat.rs
// ⏩️ 端口 - MUSIC - 收藏 - mod
// 2026/8/24 11:41 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC COLLECT STAT PORTS] - 音乐 收藏 统计端口
#[async_trait::async_trait]
pub trait MusicCollectStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的收藏记录数量
    async fn stat_collect_count_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("统计用户收藏音乐的数量"))
    }

    ////////

    /// # 2. [PORT] - 音乐被收藏的记录数量
    async fn stat_collect_count_by_music_id(
        &self,
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("统计音乐被多少人收藏了"))
    }

    ////////

    /// # 3. [PORT] - 全部收藏的记录数量
    /// * `desc`: `根据状态码统计所有的收藏记录数量`
    async fn stat_collect_total_count_by_status_id(
        &self,
        status_code: i16, // 状态码
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("统计所有的收藏记录数量"))
    }

    ////////
}

//////// END
