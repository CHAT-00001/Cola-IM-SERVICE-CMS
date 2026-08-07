// cola_video/port/recommend/del.rs
// 视频 - port - 弹幕 - 发布
// 2026/8/5 01:49 Created.

////////

////////

/// # [DEL SERVICE] - 删除
/// `desc`: `视频弹幕删除服务端口`
#[async_trait::async_trait]
pub trait DanmakuDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户单个软删除弹幕记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
        id: i64,       // 目标ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 保存
    /// * `desc`: `用户批量软删除弹幕记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
        ids: Vec<i64>, // 目标IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
