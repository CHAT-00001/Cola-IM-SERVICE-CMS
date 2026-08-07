// repo_adapter/src/user/ban/del.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 删除服务
// 2026/8/7 05:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::info::view::VideoViewInfo;
use cola_data::video::port::view::list::VideoViewListPort;

////////

/// # [DEL SERVICE] - 删除
/// * `desc`: `用户浏览删除服务`
pub struct ViewListService;

// 构造实现
#[async_trait]
impl VideoViewListPort for ViewListService {
    //

    ////////

    /// # 1. [SERVICE] - 列表
    /// * `desc`: `获取浏览记录列表`
    async fn get_view_list(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }
}

//////// END
