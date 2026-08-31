// port/src/music/album/stat.rs
// ⏩️ 端口 - 可乐音乐 - 专辑 - 统计端口
// 2026/8/24 12:14 Created.

////////

////////

/// # [MUSIC ALBUM STAT PORTS] - 音乐专辑统计端口
#[async_trait::async_trait]
pub trait MusicAlbumStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的数量
    async fn user_count(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("用户的专辑数量"))
    }

    ////////

    /// # 1. [PORT] - 不同状态的数量
    async fn status_count(
        &self,
        uid: i64,         // 操作者 ID
        status_code: i16, // 状态码
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("不同状态的数量"))
    }

    ////////

    /// # 1. [PORT] - 总数量
    async fn total_count(
        &self,
        uid: i64, // 操作者 ID
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("总数量"))
    }
}

//////// END
