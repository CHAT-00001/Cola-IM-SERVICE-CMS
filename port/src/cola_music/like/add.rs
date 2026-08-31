// port/src/cola_music/like/add.rs -- 端口 - MUSIC - 点赞 - 发布端口
// 2026/8/23 00:10 Created.

////////

/// # [ADD PORT] - 音乐点赞发布端口
/// * `desc`: `新增、恢复或取消用户的音乐最喜欢关系`
#[async_trait::async_trait]
pub trait MusicLikeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 添加点赞
    async fn add_like(
        &self,
        uid: i64,      // 用户 ID
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐最喜欢新增适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 取消点赞
    async fn un_like(
        &self,
        uid: i64,      // 用户 ID
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐最喜欢取消适配器尚未装配"))
    }

    ////////

    /// # 3. [PORT] - 插入更新点赞
    async fn upsert_like(
        &self,
        uid: i64,         // 操作者 ID
        music_id: i64,    // 音乐 ID
        status: i16,      // 状态: 1有效, 0取消
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐最喜欢新增适配器尚未装配"))
    }
}

//////// END