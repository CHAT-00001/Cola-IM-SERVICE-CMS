// repo_adapter/src/music/collect/get.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/24 16:03 Created.

////////

use port::cola_music::collect::get::MusicCollectGetPort;

////////

/// # [MUSIC COLLECT GET ADAPTER] - 音乐收藏获取适配器
pub struct MusicCollectGetAdapter;
#[async_trait::async_trait]
impl MusicCollectGetPort for MusicCollectGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏记录
    async fn get_music_ids_by_user_id(
        &self,
        user_id: i64,
        album_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(i64)> {
        todo!()
    }
}

//////// END
