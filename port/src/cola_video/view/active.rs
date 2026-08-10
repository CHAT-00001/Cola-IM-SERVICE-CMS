// cola_video/port/view/active.rs
// 视频 - 端口 - 浏览 - 发布
// 2026/8/4 22:11 Created.

////////

////////

/// # [VIEW ADD] 浏览发布
/// * `desc`: `浏览记录发布`
#[async_trait::async_trait]
pub trait VideoViewActivePort:  Send + Sync {
    //

    ////////

    /// # [PORT] - 发布
    /// * `desc`: `保存浏览记录 + 更新浏览数量`
    async fn save_view(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;


    ////////

    /// # [PORT] - 更新
    /// * `desc`: `报告浏览完成（完播） + 更新完播数量`
    async fn update_done_count(
        &self,
        uid: i64,
        video_id: i64,
        is_done: bool,
    ) -> anyhow::Result<()>;
}

//////// END