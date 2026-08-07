// cola_video/src/api/active
// core - VIDEO - api - add 发布
// 2026/4/12 14:45

////////

use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use repository::cola_video::service::ban::publish_service::VideoPublishBanService;
use crate::case::add::AddCase;
use crate::model::vo::video::VideoSingleResponse;

////////

/// # [ADD HANDLER] - 发布 接口
pub struct AddApi;

// 构造函数
impl AddApi {

    ////////

    /// # 1. [API HANDLER] - 发布视频
    pub async fn add_publish(
        user_id: i64,
        cmd: VideoNewCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(user_id).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None),
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布视频成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布视频失败: {:?}", e), None),
        }
    }


    ////////

    /// # 2. [API HANDLER] - 编辑内容
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn add_edit(
        user_id: i64,
        cmd: VideoUpdateCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(user_id).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None),
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_edit_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("编辑内容成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("编辑内容失败: {:?}", e), None),
        }
    }

    ////////

    /// # 3. [API HANDLER] - 修改状态
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn add_status(
        user_id: i64,
        cmd: VideoNewCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(user_id).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None),
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("修改状态成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("修改状态失败: {:?}", e), None),
        }
    }

    ////////

    /// # 4. [API HANDLER] - 修改权限
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn add_permission(
        user_id: i64,
        cmd: VideoUpdatePermissionCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(user_id).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None),
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_change_permission(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("修改权限成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("修改权限失败: {:?}", e), None),
        }
    }

    ////////

    /// # 5. [API HANDLER] - 修改LBS
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn add_lbs(
        user_id: i64,
        cmd: VideoNewCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(user_id).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None),
        }

        // 2. 执行核心发布逻辑
        match AddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("修改LBS成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("修改LBS失败: {:?}", e), None),
        }
    }

}

//////// END