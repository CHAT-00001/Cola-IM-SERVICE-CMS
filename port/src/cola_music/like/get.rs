// port/src/cola_music/favorites/get.rs
// ⏩️ 端口 - 可乐音乐 - 最喜欢 - 获取
// 2026/8/23 00:10 Created.

////////

/// # [GET PORT] - 最喜欢
/// * `desc`: `获取指定用户有效 favorite 关系中的音乐对象 ID`
#[async_trait::async_trait]
pub trait MusicLikeGetPort: Send + Sync {
    async fn get_music_ids_by_user_id(
        &self,
        operator_uid: i64, // 操作者 ID
        user_id: i64,      // 目标用户 ID
        limit: i64,        // 数量
        offset: i64,       // 偏移
    ) -> anyhow::Result<Vec<i64>> {
        Err(anyhow::anyhow!("音乐最喜欢获取适配器尚未装配"))
    }
}

//////// END