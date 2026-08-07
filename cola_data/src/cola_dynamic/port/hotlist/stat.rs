// cola_dynamic/port/hotlist/stat.rs
// 动态 - port - 上热门 - 统计
// 2026/8/5 00:08 Created.

////////

/// # [STAT PORT] - 统计
/// * `desc`: `动态上热门统计端口`
#[async_trait::async_trait]
pub trait HotlistStatPort: Send + Sync {
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
