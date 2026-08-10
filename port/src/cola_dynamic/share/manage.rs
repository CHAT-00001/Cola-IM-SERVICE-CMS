// port/src/cola_dynamic/share/manage.rs
// ⏩️ 端口 - ⏹ 可乐动态视频 - 分享 - 管理
// 2026/8/5 00:00 Created.

////////

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;

////////

/// # [MANAGE PORST] - 管理
/// `desc`: `⏹ 可乐动态 - 动态分享管理端口`
#[async_trait::async_trait]
pub trait DynamicShareManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 管理员列表
    /// * `desc`: `查看所有的分享记录`
    /// * `condition`: `⚠️ ADMIN` - `仅限管理员 / 内容审核员`
    async fn admin_get_share_infos(
        &self,
        uid: i64,                // UID
        user_id: Option<i64>,    // 用户 ID
        dynamic_id: Option<i64>, // 动态 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;
}

//////// END
