// service/src/cola_video/collect/get.rs
// 👤 服务 - ▶ 可乐视频 - 收藏记录 - 获取
// 2026/8/2 17:06 Created.

////////

use cola_data::cola_video::entity::collect::VideoCollectEntity;
use repository::cola_video::pg::collect::get::CollectGetRepo;

////////

/// # [COLLECT GET SERVICE] - 获取
/// * `desc`: `▶ 可乐视频 - 收藏记录获取服务`
pub struct VideoCollectGetService;

// 构造实现
impl VideoCollectGetService {
    //

    ////////

    /// # 1. [SERVICE] - IDs
    /// * `desc`: `根据用户ID 获取她收藏的视频IDs`
    pub async fn get_collect_ids_by_user_id(
        _uid: i64,    // 操作者 ID（预留）
        user_id: i64, // 目标用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<i64>, anyhow::Error> {
        // 调用 Repo 获取 Vec<i64> 并修正参数顺序
        let collect_ids = CollectGetRepo::find_video_ids_by_user_id(
            user_id,
            limit,
            offset,
        )
            .await?;

        Ok(collect_ids)
    }

    ////////

    /// # 2. [SERVICE] - 记录列表
    /// * `desc`: `根据视频ID 获取收藏记录列表`
    pub async fn get_collect_records_by_video_id(
        _uid: i64,    // 操作者 ID（预留）
        video_id: i64,// 视频 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<VideoCollectEntity>, anyhow::Error> {
        let records = CollectGetRepo::find_collect_records_by_video_id(
            video_id,
            limit,
            offset,
        )
            .await?;

        Ok(records)
    }
}

//////// END