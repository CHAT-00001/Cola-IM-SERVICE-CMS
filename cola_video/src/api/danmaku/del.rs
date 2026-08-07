// /del.rs
// 
// 2026/8/4 18:48 Created.

////////


// /danmaku  -- VIDEO - 接口层 - 弹幕
// 2026/4/24 18:51

////////

use crate::case;
use crate::case::danmaku::DanmakuCase;
use crate::model::vo::danmaku::{DanmakuListResponse, DanmakuSingleResponse};
use cola_data::app::data::AppData;
use cola_data::app::{data, error};
use cola_data::app::ctx::AppContext;
use cola_data::auth::info::auth::AuthContext;
use cola_data::video::command::danmaku::DanmakuCommand;
////////

/// # [API] - 弹幕 接口
pub struct DanmakuApi;

//
impl DanmakuApi {
    ////////

    /// # 3. [API HANDLER] -  发布弹幕
    pub async fn handler_add_danmaku(
        auth: AuthContext,
        video_id: i64,
        cmd: DanmakuCommand,
    ) -> AppData<String> {
        // 如果弹幕也需要独立校验，这里可以同样加上对应的纯函数
        match DanmakuCase::case_add_danmaku(auth.uid, video_id, cmd).await {
            Ok(_) => AppData::ok("发布弹幕成功".to_string()).with_msg("发布成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("发布弹幕失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 获取弹幕列表
    pub async fn handler_get_danmaku(
        uid: i64,
        video_id: i64,
        play_time: i32,
        ctx: &mut AppContext,
    ) -> AppData<DanmakuListResponse> { // ✅ 改为具体的结构体类型
        let segment_size = 5000;

        match DanmakuCase::case_video_danmakus(
            uid,
            video_id,
            play_time,
            play_time + segment_size,
            20,
            ctx
        ).await
        {
            Ok(list) => AppData::ok(list), // ✅ 现在 list 的类型匹配了！
            Err(e) => {
                tracing::error!("Get Danmaku Error: {:?}", e);
                // 提示：因为 err() 返回 AppData<T>，这里它会自动推导为 AppData<DanmakuListResponse>
                AppData::err(5002, "获取弹幕列表失败", None)
            }
        }
    }
}

//////// END
