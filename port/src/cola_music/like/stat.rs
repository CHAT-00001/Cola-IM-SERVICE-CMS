// port/src/cola_music/favorites/stat.rs
// ⏩️ 端口 - 可乐音乐 - 最喜欢 - 统计
// 2026/8/23 00:10 Created.

////////

/// # [STAT PORT] - 最喜欢统计
/// * `desc`: `统计用户有效 favorite 数量与音乐被 favorite 的用户数量`
#[async_trait::async_trait]
pub trait MusicLikeStatPort: Send + Sync {
    async fn count_valid_by_user_id(
        &self,
        operator_uid: i64, // 操作者 ID
        user_id: i64,      // 用户 ID
    ) -> anyhow::Result<u64> {
        Err(anyhow::anyhow!("音乐最喜欢用户统计适配器尚未装配"))
    }

    async fn count_valid_by_music_id(
        &self,
        operator_uid: i64, // 操作者 ID
        music_id: i64,     // 音乐 ID
    ) -> anyhow::Result<u64> {
        Err(anyhow::anyhow!("音乐最喜欢音乐统计适配器尚未装配"))
    }
}

//////// END