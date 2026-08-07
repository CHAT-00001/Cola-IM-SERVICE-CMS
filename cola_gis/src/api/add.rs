// cola_gis/src/api/poi_add.rs  -- GIS - api - 添加
// 2026/4/12 14:45

////////

use crate::case;
use crate::case::add::PoiAddCase;
use crate::model::vo::poi::PoiSingleResponse;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_gis::command::poi::PoiCommand;
use service::cola_gis::permission_check::VideoPermissionsCheckService;
////////

/// # [CASE] - 发布 接口
pub struct PoiAddApi;

// 构造函数
impl PoiAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn handler_add(user_id: i64, cmd: PoiCommand) -> AppData<PoiSingleResponse> {
        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无兴趣点发布权限: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match PoiAddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    /// * `描述` （需要创作者/视频发布特定权限）
    pub async fn handler_edit(user_id: i64, cmd: PoiCommand) -> AppData<PoiSingleResponse> {
        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = VideoPermissionsCheckService::check_video_publish_perm(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无兴趣点发布权限: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match PoiAddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////
}

//////// END
