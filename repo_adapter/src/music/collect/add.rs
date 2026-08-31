// repo_adapter/src/music/collect/add.rs -- 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/23 00:20 Created.

////////

use port::cola_music::collect::add::MusicCollectAddPort;
use repository::music::pg::collect::add::MusicCollectAddRepo;

////////

/// # [MUSIC COLLECT ADD ADAPTER] - 音乐收藏发布适配器
pub struct MusicCollectAddAdapter;

#[async_trait::async_trait]
impl MusicCollectAddPort for MusicCollectAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏
    async fn save_collect(
        &self,
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> anyhow::Result<(bool)> {
        // Call REPOSITORY ..
        MusicCollectAddRepo::save(uid, music_id, album_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 保存音乐收藏失败: {error}"))
    }

    ////////

    /// # 2. [ADAPTER] - 取消收藏
    async fn un_collect(
        &self,
        uid: i64,
        music_id: i64,
        album_id: Option<i64>,
    ) -> anyhow::Result<(bool)> {
        // Call REPOSITORY ..
        MusicCollectAddRepo::delete(uid, music_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 更新音乐收藏失败: {error}"))
    }

    ////////

    /// # 3. [ADAPTER] - 取消收藏
    async fn get_collect_ids_user_id(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
