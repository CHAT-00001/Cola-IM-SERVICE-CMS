// cola_video/src/api/active  -- VIDEO - 应用层 - 添加
// 2026/4/12 14:45

////////

use crate::case;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use repository::cola_video::service::permission_check::VideoPermissionsCheckService;
use crate::case::add::AddCase;
use crate::case::like::LikeCase;
use crate::model::vo::video::VideoSingleResponse;

////////

/// # [USE CASE] -  点赞接口
pub struct LikeApi;

impl LikeApi {

    ////////

    /// # 1. [API HANDLER] - 点赞视频
    pub async fn handler_add_video_like(
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> AppData<()> {

        // // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        // if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
        //     return AppData::err(
        //         error::INTERNAL_ERROR,
        //         format!("无视频发布权限: {:?}", e),
        //         None,
        //     );
        // }

        // 2. 执行核心发布逻辑
        match LikeCase::case_add_video_like(uid, video_id, is_liked,).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }


    ////////

    /// # 2. [API HANDLER] - 不喜欢视频
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn handler_add_video_unlike(
        user_id: i64,
        cmd: VideoNewCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无视频发布权限: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

}

//////// END