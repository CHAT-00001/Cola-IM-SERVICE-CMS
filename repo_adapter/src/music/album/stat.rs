// repo_adapter/src/music/album/stat.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 统计适配器
// 2026/8/24 13:32 Created.

////////

use port::cola_music::album::stat::MusicAlbumStatPort;

////////

/// # [STAT ADAPTER] - 音乐专辑统计适配器
pub struct MusicAlbumStatAdapter;

#[async_trait::async_trait]
impl MusicAlbumStatPort for MusicAlbumStatAdapter {
    //

    ////////

    /// # [ADAPTER] - 用户数量
    async fn user_count(&self, uid: i64, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 状态数量
    async fn status_count(&self, uid: i64, status_code: i16) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 总数量
    async fn total_count(&self, uid: i64) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
