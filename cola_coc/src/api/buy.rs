// /add  -- 接口层 - 购买
// 2026/6/10 08:41

////////

use crate::case::buy::BuyCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_video::command::buy::VideoBuyCommand;
use port::app::ctx::AppContext;

////////

/// # [APP USE CASE] -  购买接口
pub struct BuyApi;

impl BuyApi {
    ////////

    /// # 1. [API HANDLER] -  添加 - 购买
    pub async fn handler_add_buy(
        user_id: i64,
        video_id: i64,
        mut cmd: VideoBuyCommand,
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
