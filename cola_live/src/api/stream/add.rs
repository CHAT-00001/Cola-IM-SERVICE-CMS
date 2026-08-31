// cola_live/src/api/stream/add.rs
// LIVE - API - 直播场次 - 开播接口
// 2026/8/21 09:42 Created.

////////

use crate::case::stream::add::LiveStreamAddCase;
use cola_data::app::data::AppData;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_data::cola_live::info::record::LiveRecordInfo;
use port::app::ctx::AppContext;
use serde_json::Value;
use tracing::{error, info};

////////

/// # 1. [API HANDLER] - 直播场次开播
pub struct LiveStreamAddApi;

impl LiveStreamAddApi {
    /// # 1. [API HANDLER] - 开播
    /// * `desc`: `创建直播场次并返回推流/播放地址`
    pub async fn start(
        auth: AuthContext,
        command: LiveRecordCommand,
        ctx: &AppContext,
    ) -> AppData<Value> {
        match LiveStreamAddCase::start(&auth, command, ctx).await {
            Ok(info) => {
                info!("[🗣️ API] - ✅️ 开播成功: uid={}", auth.uid);
                AppData::ok(serde_json::to_value(info).unwrap_or_default()).with_msg("开播成功")
            }
            Err(err) => {
                error!("[🤐 API] - ❌️ 开播失败: uid={}, error={}", auth.uid, err);
                AppData::err(4000, "开播失败", Some(err.to_string()))
            }
        }
    }

    /// # 2. [API HANDLER] - 停播
    pub async fn stop(auth: AuthContext, record_id: i64, ctx: &AppContext) -> AppData<Value> {
        match LiveStreamAddCase::stop(&auth, record_id, ctx).await {
            Ok(()) => AppData::ok(serde_json::json!({ "record_id": record_id, "status": 0 }))
                .with_msg("停播成功"),
            Err(err) => {
                error!("[🤐 API] - ❌️ 停播失败: uid={}, error={}", auth.uid, err);
                AppData::err(4000, "停播失败", Some(err.to_string()))
            }
        }
    }
}

//////// END
