// repo_adapter/src/music/collect/check.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/24 16:04 Created.

////////

use port::cola_music::collect::check::MusicCollectCheckPort;

////////

/// # [MUSIC COLLECT CHECK ADAPTER] - 音乐收藏检查适配器
pub struct MusicCollectCheckAdapter;
#[async_trait::async_trait]
impl MusicCollectCheckPort for MusicCollectCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏记录
    async fn collect_music(
        &self,
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 取消收藏记录
    async fn uncollect_music(
        &self,
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
