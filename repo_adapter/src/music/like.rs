// repo_adapter/src/music/like.rs
// 🔌 适配器 - MUSIC - LIKE
// 2026/8/10 20:23 Created.

////////

use async_trait::async_trait;
use port::cola_music::like::LikeRepo;

////////

/// # [LIKE ADAPTER] - 音乐点赞
/// * `desc`: `🎶 音乐点赞适配器`
pub struct MusicLikePortAdapter;

#[async_trait]
impl LikeRepo for MusicLikePortAdapter {
    ////////

    /// # [PORT] - 点赞
    async fn add_like(&self, _uid: i64, _music_id: i64, _status: i16) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 不喜欢
    async fn add_unlike(&self, _uid: i64, _music_id: i64, _status: i16) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 获取点赞的IDs
    async fn get_like_ids(&self, _uid: i64, _limit: i64, _offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # [PORT] - 获取不喜欢的IDs
    async fn get_unlike_ids(&self, _uid: i64, _limit: i64, _offset: i64) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END