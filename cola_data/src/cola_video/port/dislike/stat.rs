// cola_video/port/dislike/stat.rs
// 视频 - port - 不喜欢 - 统计
// 2026/8/5 16:00 Created.

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `视频不喜欢统计端口`
#[async_trait::async_trait]
pub trait VideoDislikeStatPort {
    //

    ////////

    /// # [PORT] - 发布
    /// * `desc`: `保存浏览记录 + 更新浏览数量`
    async fn save_view(&self, uid: i64, video_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 更新
    /// * `desc`: `报告浏览完成（完播） + 更新完播数量`
    async fn update_done_count(&self, uid: i64, video_id: i64, is_done: bool)
    -> anyhow::Result<()>;
}

//////// END
