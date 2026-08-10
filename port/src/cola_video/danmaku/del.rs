// port/src/cola_video/danmaku/del.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 发布
// 2026/8/5 01:49 Created.

////////

////////

/// # [DELETE PORTS] - 删除
/// `desc`: `视频弹幕删除服务端口`
#[async_trait::async_trait]
pub trait VideoDanmakuDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户单个软删除弹幕记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 保存
    /// * `desc`: `用户批量软删除弹幕记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,              // UID
        danmaku_ids: Vec<i64>, // 弹幕 IDs
    ) -> anyhow::Result<(u64)>;
}

//////// END
