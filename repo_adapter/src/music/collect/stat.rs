// repo_adapter/src/music/collect/stat.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/24 16:04 Created.

////////

use port::cola_music::collect::check::MusicCollectCheckPort;
use port::cola_music::collect::stat::MusicCollectStatPort;
////////

/// # [MUSIC COLLECT STAT ADAPTER] - 音乐收藏统计适配器
pub struct MusicCollectStatAdapter;
#[async_trait::async_trait]
impl MusicCollectStatPort for MusicCollectStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的收藏数量
    async fn stat_collect_count_by_user_id(&self, user_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 音乐ID被收藏数量
    async fn stat_collect_count_by_music_id(&self, music_id: i64) -> anyhow::Result<(u64)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 总收藏数量
    async fn stat_collect_total_count_by_status_id(
        &self,
        status_code: i16,
    ) -> anyhow::Result<(u64)> {
        todo!()
    }
}

//////// END
