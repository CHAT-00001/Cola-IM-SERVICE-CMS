// repo_adapter/src/music/like/add.rs -- 🔌 适配器 - MUSIC - 点赞 - 发布适配器
// 2026/8/23 00:20 Created.

////////

use port::cola_music::like::add::MusicLikeAddPort;
use repository::music::pg::like::add::MusicLikeAddRepo;

////////

/// # [ADAPTER] - 音乐点赞发布适配器
/// * `desc`: `COLA MUSIC - Like Add Adapter.`
pub struct MusicLikeAddAdapter;
#[async_trait::async_trait]
impl MusicLikeAddPort for MusicLikeAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加点赞
    async fn add_like(&self, uid: i64, music_id: i64) -> anyhow::Result<()> {
        // Call PG REPO ..
        MusicLikeAddRepo::save_like(uid, music_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 添加音乐点赞失败: {error}"))
    }

    ////////

    /// # 2. [ADAPTER] - 取消点赞
    async fn un_like(&self, uid: i64, music_id: i64) -> anyhow::Result<()> {
        // Call PG REPO ..
        MusicLikeAddRepo::un_like(uid, music_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 取消音乐点赞失败: {error}"))
    }

    ////////

    /// # 3. [ADAPTER] - 更新/插入点赞
    async fn upsert_like(&self, uid: i64, music_id: i64, status: i16) -> anyhow::Result<()> {
        // Call PG REPO ..
        MusicLikeAddRepo::upsert_like(uid, music_id, status)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 更新/插入音乐点赞失败: {error}"))
    }
}

//////// END
