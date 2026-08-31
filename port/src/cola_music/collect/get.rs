// cola_port/src/music/collect/get.rs -- 端口 - MUSIC - 收藏 - mod
// 2026/8/24 11:41 Created.

////////

////////

/// # [MUSIC COLLECT GET PORT] - 音乐收藏获取端口
/// * `desc`: `COLA MUSIC - Collect Get Ports.`
#[async_trait::async_trait]
pub trait MusicCollectGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的
    /// * `desc`: `根据用户ID / 专辑ID 查找收藏的音乐IDs`
    async fn get_music_ids_by_user_id(
        &self,
        user_id: i64,          // 用户 ID
        album_id: Option<i64>, // 专辑 ID
        limit: i64,            // 数量
        offset: i64,           // 页码
    ) -> anyhow::Result<(i64)> {
        Err(anyhow::anyhow!("音乐收藏新增适配器尚未装配"))
    }
}

//////// END
