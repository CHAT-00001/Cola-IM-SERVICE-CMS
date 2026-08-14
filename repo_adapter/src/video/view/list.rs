// repo_adapter/src/user/ban/del.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 删除服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::view::VideoViewInfo;
use port::cola_video::view::list::VideoViewListPort;
////////

/// # [DEL ADAPTER] - 删除
/// * `desc`: `用户浏览删除服务`
pub struct ViewListService;

// 构造实现
#[async_trait]
impl VideoViewListPort for ViewListService {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的主动浏览记录
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 视频的被动浏览记录
    async fn get_view_infos_by_video_id(
        &self,
        video_id: i64, // 视频 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }
}

//////// END
