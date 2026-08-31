// port/src/music/music/del.rs
// ⏩️ 端口 - 可乐音乐 - 音乐 - 列表
// 2026/8/22 23:30 Created.

////////

/// # [DELETE PORTS] - 逻辑删除
#[async_trait::async_trait]
pub trait MusicDelPort: Send + Sync {
    //

    ////////

    /// # 3. [PORT] - ❌️ 👤 用户批量软删除音乐(支持批量)
    async fn user_delete_by_music_ids(
        &self,
        uid: i64,            // 操作者 ID
        music_ids: Vec<i64>, // 音乐 IDs
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐删除适配器尚未装配"))
    }

    ////////

    /// # 3. [PORT] - ❌️ ⏰️ 自动任务硬删除过期的项目
    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐删除适配器尚未装配"))
    }
}

//////// END
