// cola_port/src/music/collect/add.rs -- 端口 - MUSIC - 收藏 - 发布端口
// 2026/8/22 23:44 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC COLLECT ADD PORTS] - 音乐收藏发布端口
#[async_trait::async_trait]
pub trait MusicCollectAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 添加收藏
    async fn save_collect(
        &self,
        uid: i64,              // 操作者 ID
        music_id: i64,         // 音乐 ID
        album_id: Option<i64>, // 可选目标专辑 ID
    ) -> anyhow::Result<(bool)> {
        Err(anyhow::anyhow!("音乐收藏适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 取消收藏
    async fn un_collect(
        &self,
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> anyhow::Result<(bool)> {
        Err(anyhow::anyhow!("音乐收藏适配器尚未装配"))
    }

    ////////

    /// # 3. [PORT] - 🆔 👤 根据用户ID获取收藏的IDs
    async fn get_collect_ids_user_id(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        Err(anyhow::anyhow!("音乐收藏适配器尚未装配"))
    }
}

//////// END
