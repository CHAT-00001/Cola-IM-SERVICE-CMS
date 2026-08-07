// cola_dynamic/port/view/stat.rs
// 动态 - port - 浏览 - 统计
// 2026/8/4 22:11 Created.

////////

/// # [STAT PORTS] 浏览 统计
/// * `desc`: `动态浏览统计端口`
#[async_trait::async_trait]
pub trait ViewStatPort:  Send + Sync {
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
