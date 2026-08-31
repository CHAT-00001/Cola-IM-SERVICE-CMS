// repo_adapter/src/music/like/get.rs -- 🔌 适配器 - MUSIC - 点赞 - 获取适配器
// 2026/8/23 00:20 Created.

////////

use port::cola_music::like::get::MusicLikeGetPort;
use repository::music::pg::like::get::MusicLikeGetRepo;

////////

/// # [GET ADAPTER] - 音乐点赞获取适配器
/// * `desc`: `COLA MUSIC - LIKE GET Adapter`
pub struct MusicLikeGetAdapter;

#[async_trait::async_trait]
impl MusicLikeGetPort for MusicLikeGetAdapter {

    /// # [ADAPTER] - 根据用户ID获取点赞的音乐IDs
    async fn get_music_ids_by_user_id(
        &self,
        _operator_uid: i64,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<i64>> {
        // Call REPO ..
        MusicLikeGetRepo::find_music_ids_by_user_id(user_id, limit, offset)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 查询最喜欢音乐失败: {error}"))
    }
}

//////// END
