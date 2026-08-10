// like/manage.rs
// 视频 - port - 点赞 - 管理
// 2026/8/5 00:04 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

/// # [STAT PORT] - 管理
/// * `desc`: `视频点赞管理端口`
#[async_trait::async_trait]
pub trait LikeManagePort: Send + Sync {
    //

    ////////
    /// # [PORT] - 全部点赞记录
    /// * `desc`: `⏹ 可乐动态` - `点赞记录`
    /// * `condition`: `⚠️ ADMIN` - `仅限管理员`
    async fn admin_get_hotlist_infos(
        &self,
        uid: i64,                 // UID
        dynamic_id: i64,          // 动态 ID
        status_code: Option<i16>, // 状态码
        is_deleted: Option<bool>, // 是否删除
        limit: i64,               // 数量
        offset: i64,              // 页码
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;
}

//////// END
