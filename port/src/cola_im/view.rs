// port/get  -- 端口 浏览
// 2026/6/10 07:13

////////

use crate::video::info::video::VideoInfo;

////////

/// # [SERVICE PORT] - 浏览 服务
#[async_trait::async_trait]
pub trait ViewPort: Send + Sync {

    ////////

    /// # [PORT] - 保存浏览记录 + 更新浏览数量
    async fn save_view_record_update_views_count(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 报告浏览完成（完播） + 更新完播数量
    async fn view_done_update_done_count(
        &self,
        uid: i64,
        video_id: i64,
        is_done: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 单个获取视频
    async fn get_video_list_by_id(
        &self,
        video_id: i64,
    ) -> anyhow::Result<(VideoInfo)>;

    ////////

    /// # [PORT] - 批量获取视频
    async fn get_video_list_by_ids(
        &self,
        video_ids: Vec<i64>,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;
}