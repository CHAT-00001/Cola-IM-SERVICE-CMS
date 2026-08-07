// data/cola_video/port/view/manage.rs
// 数据中心 - VIDEO - port - 浏览 管理
// 2026/8/4 22:11 Created.

////////

use crate::cola_video::info::video::VideoInfo;
use crate::cola_video::info::view::VideoViewInfo;

////////

/// # [MANAGE PORT]
/// * `desc`: `视频浏览管理端口`
#[async_trait::async_trait]
pub trait VideoViewManagePort: Send + Sync {
    ////////

    /// # [PORT] - 发布
    /// * `desc`: `保存浏览记录 + 更新浏览数量`
    async fn save_view_record_update_views_count(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 更新
    /// * `desc`: `报告浏览完成（完播） + 更新完播数量`
    async fn view_done_update_done_count(
        &self,
        uid: i64,
        video_id: i64,
        is_done: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 单个删除
    /// * ``: `用户单个删除浏览记录`
    async fn single_del_view_record_by_id(
        &self,
        id: i64, // 记录ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # [PORT] - 批量删除
    /// `desc`: `管理员批量删除浏览记录`
    async fn batch_del_view_record_by_ids(
        &self,
        ids: Vec<i64>, // 记录IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # [PORT] - 批量获取视频
    async fn get_video_list_by_ids(
        &self,
        video_ids: Vec<i64>, // 视频IDs
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # [PORT] - 我的
    /// * `desc`: `获取我的浏览视频列表`
    async fn get_my_viewed_list(
        &self,
        uid: i64,    // 用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # [PORT] - 她的
    /// * `desc`: `获取她的浏览视频列表`
    async fn get_here_viewed_list(
        &self,
        uid: i64,    // 用户ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # [PORT] - 她的
    /// * `desc`: `获取她的浏览视频列表`
    async fn get_video_viewed_list(
        &self,
        video_id: i64, // 视频ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoViewInfo>)>;
}

//////// END