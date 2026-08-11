// service/src/cola_video/report/del.rs
// 👤 服务 - ▶ 可乐视频 - 举报 - 软删除
// 2026/8/9 01:51 Created.

////////

use anyhow::Result;
use tracing::log;

////////

/// # [DELETE SERVICE] - 软删除
/// * `desc`: `▶ 可乐视频 - 视频举报列表服务`
pub struct VideoReportDelService;

// 构造实现
impl VideoReportDelService {
    //

    ////////

    /// # 1. [SERVICE] - 视频的
    /// * `desc`: `根据视频ID` - 删除举报记录
    pub async fn delete_record_by_video_id(video_id: i64) -> Result<i32> {
        let _ = video_id;
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 2. [SERVICE] - 用户的
    /// * `desc`: `根据用户ID` - 删除举报记录
    pub async fn delete_record_by_user_id(user_id: i64) -> Result<i32> {
        let _ = user_id;
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 3. [SERVICE] - 单个软删除
    /// * `desc`: `单个删除`
    pub async fn delete_single(
        uid: i64,       // UID
        report_id: i64, // 举报 ID
    ) -> Result<u64> {
        let _ = (uid, report_id);
        Ok(1)
    }

    ////////

    /// # 4. [SERVICE] - 批量软删除
    // * `desc`: `批量删除`
    pub async fn delete_batch(
        uid: i64,             // UID
        report_ids: Vec<i64>, // 举报 IDs
    ) -> Result<u64> {
        let _ = (uid, report_ids);
        Ok(1)
    }
}

//////// END