// repo_adapter/src/video/collect/manage.rs
// 🔌 适配器 - ▶ 可乐视频 - Collect - Manage 实现
// 2026/8/9 01:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::collect::VideoCollectInfo;
use port::cola_video::collect::manage::VideoCollectManagePort;

////////

/// # [MANAGE ADAPTER] - 收藏
/// * `desc`: `收藏记录管理适配器`
#[derive(Debug, Default, Clone)]
pub struct CollectManageAdapter;

// 构造实现
#[async_trait]
impl VideoCollectManagePort for CollectManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_collects_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCollectInfo), u64> {
        todo!()
    }
}

//////// END
