// like/manage.rs
// 视频 - port - 点赞 - 管理
// 2026/8/5 00:04 Created.

////////

/// # [STAT PORT] - 管理
/// * `desc`: `视频点赞管理端口`
#[async_trait::async_trait]
pub trait LikeManagePort: Send + Sync {
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
