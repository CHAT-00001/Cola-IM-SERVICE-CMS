// cola_live/src/api/stream/get.rs -- LIVE - api - 直播流 - 获取接口
// 2026/8/20 19:38 Created.

use crate::case;
use crate::case::add::AddCase;
use crate::model::vo::video::VideoSingleResponse;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use service::cola_video::video::check::VideoPermissionsCheckService;

////////

/// # [ADD API] - 获取 接口
pub struct LiveStreamGetApi;

// 构造函数
impl LiveStreamGetApi {
    //

    ////////

    /// # 1. [API HANDLER] - 开播
    /// * `desc`: `主播开始直播, 创建直播记录`
    pub(crate) async fn api_start(
        user_id: i64,
        cmd: VideoNewCommand,
    ) -> AppData<VideoSingleResponse> {
        // 1. 检查用户状态

        // 2. 检查主播状态
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(error::INTERNAL_ERROR, format!("无开播权限: {:?}", e), None);
        }

        // 3. 检查直播间状态
        // check_room_state();

        // 4. 执行核心发布逻辑
        match AddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("开播成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("开播失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    /// * `描述`: `用户修改直播记录标题等.`
    pub(crate) async fn api_edit(
        user_id: i64,
        cmd: VideoUpdateCommand,
    ) -> AppData<VideoSingleResponse> {
        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无直播记录编辑权限: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_edit_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("无直播记录编辑成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("无直播记录编辑失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 3. [API HANDLER] - 停播
    /// * `描述`: `主播下播`
    pub(crate) async fn api_stop(
        user_id: i64,
        cmd: VideoUpdateCommand,
    ) -> AppData<VideoSingleResponse> {
        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("主播关闭直播失败: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_edit_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("主播关闭直播成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("主播关闭直播失败: {:?}", e),
                None,
            ),
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
