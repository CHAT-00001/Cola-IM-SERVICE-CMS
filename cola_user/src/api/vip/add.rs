// cola_user/src/api/vip/add.rs
// 用户 - api - 贵宾 - 发布接口
// 2026/6/10 08:41

////////

use crate::case::vip::add::UserVipAddCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use tracing::{error, info};

////////

/// # [ADD API] - 发布
/// * `desc`: `开通贵宾会员接口`
pub struct VipAddApi;

impl VipAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 开通
    /// * `desc`: `开通贵宾会员`
    pub async fn api_add_vip(
        uid: i64,                 // 操作者ID
        query: ApiGatewayRequest, //  网关
        ctx: &AppContext,         // 全局上下文
    ) -> AppData<String> {
        match UserVipAddCase::case_add_new(uid, query.id, ctx).await {
            Ok(_) => {
                info!("[🗣️ API] - ✅️ 用户充值贵宾卡成功!");
                AppData::ok("购买成功".to_string()).with_msg("用户充值贵宾卡成功")
            }
            Err(e) => {
                error!("[🤐 API] - ❌️ 用户充值贵宾卡失败!");
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("用户充值贵宾卡失败: {:?}", e),
                    None,
                )
            }
        }
    }
}

//////// END
