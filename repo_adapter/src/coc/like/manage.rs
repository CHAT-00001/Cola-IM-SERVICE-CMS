// repo_adapter/src/video/like/manage.rs
// 🔌 适配器 - ▶ 视频 - 点赞 - 管理
// 2026/8/6 18:58 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::like::VideoLikeInfo;
use port::cola_video::like::manage::VideoLikeManagePort;
use repository::video::pg::like::manage::VideoLikeManageRepo;

////////

/// # [MANAGE ADAPTER] - like manage
/// * `desc`: `▶ 视频 - 视频点赞管理适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoLikeManageAdapter;

// 构造实现
#[async_trait]
impl VideoLikeManagePort for VideoLikeManageAdapter {
    //

    ////////

    /// 1. [ADAPTER] - 管理员列表
    async fn admin_get_recommends_infos(
        &self,
        _uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoLikeInfo>, u64)> {
        // 假设仓库层返回的是 (Vec<Entity>, u64) 元组
        let (entities, total) = VideoLikeManageRepo::find_admin_list(
            user_id,
            video_id,
            start_time,
            end_time,
            status_code,
            limit,
            offset,
        )
        .await?;

        // 批量转换
        let infos: Vec<VideoLikeInfo> = entities
            .into_iter()
            .map(VideoLikeInfo::from_entity)
            .collect();

        Ok((infos, total))
    }
}

//////// END
