// /buy.rs  -- 接口层 - 购买
// 2026/6/10 08:41

////////

use cola_data::app::ctx::AppContext;
use crate::case;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::video::command::buy::BuyCommand;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_data::video::command::hotlist::HotlistCommand;
use cola_data::video::command::report::ReportCommand;
use cola_data::video::command::share::ShareCommand;
use cola_data::video::command::video::VideoCommand;
use repo::video::service::permission_change::PermissionsChangeService;
use repo::video::service::permission_check::VideoPermissionsCheckService;
use crate::case::add::AddCase;
use crate::case::buy::BuyCase;
use crate::case::follow::LikeCase;
use crate::model::vo::video::VideoSingleResponse;

////////

/// # [APP USE CASE] -  购买接口
pub struct BuyApi;

impl BuyApi {

    ////////

    /// # 1. [API HANDLER] -  添加 - 购买
    pub async fn handler_add_buy(
        user_id: i64,
        video_id: i64,
        mut cmd: BuyCommand,
        ctx: &AppContext,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match BuyCase::case_add_video_buy(user_id, video_id, cmd, ctx).await {
            Ok(_) => AppData::ok("购买成功".to_string()).with_msg("购买成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("购买失败: {:?}", e), None),
        }
    }
}

////////