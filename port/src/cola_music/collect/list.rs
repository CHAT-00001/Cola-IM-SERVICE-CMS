// cola_port/src/music/collec/list.rs
// ⏩️ 端口 - MUSIC - 收藏 - mod
// 2026/8/24 11:41 Created.

////////

/// # [MUSIC COLLECT LIST PORT] - 音乐收藏列表端口
/// * `desc`: `收藏记录列表端口`
#[async_trait::async_trait]
pub trait MusicCollectListPort: Send + Sync {
    //

    /// # 1. [PORT] - 用户的
    /// * `desc`: `根据用户ID获取收藏记录列表`
    async fn get_collect_record_by_user_id(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("用户的收藏记录列表"))
    }

    ////////

    /// # 2. [PORT] - 音乐的
    /// * `desc`: `根据音乐ID获取被收藏的记录列表`
    async fn get_collect_record_by_music_id(
        &self,
        uid: i64,      // 操作者 ID
        music_id: i64, // 音乐 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐的收藏记录列表"))
    }
}

//////// END
