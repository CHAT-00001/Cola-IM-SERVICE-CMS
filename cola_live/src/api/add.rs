// cola_live/src/api/active
// core - LIVE - api - 添加
// 2026/4/12 14:45

////////

use crate::case;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::video::command::video::edit::VideoUpdateCommand;
use cola_data::video::command::video::new::VideoNewCommand;
use repository::video::service::permission_check::VideoPermissionsCheckService;
use crate::case::add::AddCase;
use crate::model::vo::video::VideoSingleResponse;

////////

/// # [APP USE CASE] - 发布 接口
pub struct AddApi;

// 构造函数
impl AddApi {

    ////////

    /// # 1. [API HANDLER] - 发布
    pub(crate) async fn handler_add_video(
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


    ////////

    /// # 2. [API HANDLER] - 编辑
    /// * `描述` （需要创作者/视频发布特定权限）
    pub(crate) async fn handler_edit_video(
        user_id: i64,
        cmd: VideoUpdateCommand,
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
        match AddCase::case_edit_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////
}

//////// END