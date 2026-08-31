// cola_port/src/music/collect/check.rs
// ⏩️ 端口 - MUSIC - 收藏 - 检查端口
// 2026/8/24 12:11 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC COLLECT CHECK PORTS] - 音乐 收藏 检查端口
#[async_trait::async_trait]
pub trait MusicCollectCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 收藏音乐
    /// * `desc`: `收藏音乐，并可选加入指定专辑`
    async fn collect_music(
        &self,
        uid: i64,              // 用户 ID
        music_id: i64,         // 音乐 ID
        album_id: Option<i64>, // 专辑 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐收藏新增适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 取消收藏音乐
    /// * `desc`: `软删除用户与音乐的收藏关系`
    async fn uncollect_music(
        &self,
        uid: i64,              // 用户 ID
        music_id: i64,         // 音乐 ID
        album_id: Option<i64>, // 专辑 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐收藏取消适配器尚未装配"))
    }
}

//////// END
