// port/src/cola_music/favorites/del.rs
// ⏩️ 端口 - 可乐音乐 - 最喜欢 - 定时删除
// 2026/8/23 00:10 Created.

////////

/// # [DELETE PORT] - 最喜欢硬删除
/// * `desc`: `定时任务物理删除已软删除且超过保留期的 favorite 记录`
#[async_trait::async_trait]
pub trait MusicLikeDelPort: Send + Sync {
    async fn hard_delete_expired(
        &self,
        operator_uid: i64, // 任务操作者 ID
        time_range: i64,   // 保留时间范围
    ) -> anyhow::Result<u64> {
        Err(anyhow::anyhow!("音乐最喜欢删除适配器尚未装配"))
    }
}

//////// END