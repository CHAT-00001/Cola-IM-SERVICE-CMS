// service/src/cola_video/buy/manage.rs
// 仓储 - ▶ 可乐视频 - 购买记录 - 管理
// 2026/8/2 16:49 Created.

////////

use cola_data::cola_video::entity::buy::VideoBuyEntity;
use repository::cola_video::pg::buy::manage::VideoBuyManageRepo;

////////

/// # [MANAGE SERVICE] - 管理
/// * `desc`: `▶ 可乐视频 - 👤 购买管理服务`
pub struct VideoBuyManageService;

impl VideoBuyManageService {
    //

    ////////

    /// # 1. [SERVICE] - 管理员查看综合列表
    /// * `desc`: `获取视频购买记录综合列表`
    pub async fn get_all_list_at_admin(
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        keyword: Option<String>, // 关键词
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        video_id: Option<i64>,   // 视频 ID
        status: Option<i16>,     // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<Vec<VideoBuyEntity>, anyhow::Error> {
        let list = VideoBuyManageRepo::find_all_record_at_admin(
            uid,
            user_id,
            keyword,
            start_time,
            end_time,
            video_id,
            status,
            limit,
            offset,
        )
            .await?;

        Ok(list)
    }

    ////////

    /// # 2. [SERVICE] - 管理员设置单个状态
    /// * `desc`: `更新单个购买记录状态`
    pub async fn set_record_status_by_id(
        uid: i64,   // 操作者 ID
        buy_id: i64, // 购买 ID
        status: i16, // 新状态码
    ) -> Result<u64, anyhow::Error> {
        let rows_affected = VideoBuyManageRepo::reset_record_status_by_id(
            uid,
            buy_id,
            status,
        )
            .await?;

        Ok(rows_affected)
    }

    ////////

    /// # 3. [SERVICE] - 管理员批量设置状态
    /// * `desc`: `批量更新购买记录状态`
    pub async fn set_record_status_by_ids(
        uid: i64,        // 操作者 ID
        buy_ids: &[i64], // 购买 IDs
        status: i16,     // 新状态码
    ) -> Result<u64, anyhow::Error> {
        if buy_ids.is_empty() {
            return Ok(0);
        }

        let rows_affected = VideoBuyManageRepo::reset_record_status_by_ids(
            uid,
            buy_ids,
            status,
        )
            .await?;

        Ok(rows_affected)
    }
}

//////// END