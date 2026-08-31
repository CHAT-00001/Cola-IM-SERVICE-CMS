// repo_adapter/src/music/favorites/stat.rs -- 🔌 适配器 - 可乐音乐 - 点赞 - 统计适配器
// 2026/8/23 00:20 Created.

////////

use port::cola_music::like::stat::MusicLikeStatPort;
use repository::music::pg::like::stat::MusicLikeStatRepo;

////////
pub struct MusicLikeStatAdapter;
#[async_trait::async_trait]
impl MusicLikeStatPort for MusicLikeStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 统计用户点赞的数量
    async fn count_valid_by_user_id(
        &self,
        _operator_uid: i64, // 操作者 ID
        user_id: i64,       // 用户 ID
    ) -> anyhow::Result<u64> {

        // Call PG REPO ..
        MusicLikeStatRepo::count_valid_by_user_id(user_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 统计用户点赞的数量失败: {error}"))
    }

    ////////

    /// # 2. [ADAPTER] - 统计音乐的点赞的数量
    async fn count_valid_by_music_id(
        &self,
        _operator_uid: i64, // 操作者 ID
        music_id: i64,      // 音乐 ID
    ) -> anyhow::Result<u64> {

        // Call PG REPO ..
        MusicLikeStatRepo::count_valid_by_music_id(music_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 统计音乐被点赞的数量失败: {error}"))
    }
}

//////// END
