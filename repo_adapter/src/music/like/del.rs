// repo_adapter/src/music/like/del.rs -- 适配器 - MUSIC - 点赞 - 删除
// 2026/8/23 00:20 Created.

////////

use port::cola_music::like::del::MusicLikeDelPort;
use repository::music::pg::like::del::MusicLikeDelRepo;

////////
pub struct MusicLikeDelAdapter;

#[async_trait::async_trait]
impl MusicLikeDelPort for MusicLikeDelAdapter {
    //

    ////////

    /// # [ADAPTER] - 删除记录
    async fn hard_delete_expired(
        &self,
        _operator_uid: i64,
        time_range: i64,
    ) -> anyhow::Result<u64> {
        MusicLikeDelRepo::hard_delete_expired(time_range)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 清理最喜欢记录失败: {error}"))
    }
}

//////// END
