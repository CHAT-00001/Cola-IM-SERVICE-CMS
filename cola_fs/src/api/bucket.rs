// cola_fs/src/api/bucket.rs
// FS - API - 桶
// 2026/7/30 21:13 Created.

////////

use crate::case::bucket::FsBucketAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::info::video::VideoSingleResponse;
use port::app::ctx::AppContext;
use service::cola_video::ban::publish_service::VideoPublishBanService;

////////

pub struct FsBucketApi;

impl FsBucketApi {
    //

    ////////

    /// # [API] - 添加存储桶
    pub async fn api_add_bucket(
        uid: i64,               // 操作者 ID
        cmd: VideoNewCommand,   // 命令
        url: ApiGatewayRequest, // 网关参数
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<VideoSingleResponse> {
        // 1. 发布权限检查：没封禁记录 = true = 可发布
        match VideoPublishBanService::check_banned(uid).await {
            Ok(true) => {} // 可发布，继续
            Ok(false) => return AppData::err(error::FORBIDDEN, "你没有发布权限", None),
            Err(e) => {
                return AppData::err(
                    error::INTERNAL_ERROR,
                    format!("权限检查失败: {:?}", e),
                    None,
                );
            }
        }

        // 2. 执行核心发布逻辑
        match FsBucketAddCase::case_add_publish(uid, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布存储桶成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("发布存储桶失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
