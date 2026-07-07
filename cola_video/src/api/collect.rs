// cola_video/src/api/collect.rs  -- 可乐短视频 - 接口层 - 收藏
// 2026/6/9 11:02

////////

use crate::case;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::video::command::buy::VideoBuyCommand;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::report::VideoReportCommand;
use crate::case::collect::CollectCase;

////////

/// # [API HANDLER] - 收藏 接口
pub struct CollectApi;

// 构造函数
impl CollectApi {
    //

    ////////

    /// # 1. [CASE] -  添加
    pub async fn handler_add_collect(
        user_id: i64,
        video_id: i64,
        mut cmd: CollectCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match CollectCase::case_add_collect(user_id, video_id, cmd).await {
            Ok(_) => AppData::ok("收藏成功".to_string()).with_msg("收藏成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("收藏失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [CASE] -  编辑
    pub async fn handler_edit_collect(
        user_id: i64,
        video_id: i64,
        mut cmd: CollectCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match CollectCase::case_set_collect(user_id, video_id, cmd).await {
            Ok(_) => AppData::ok("修改成功".to_string()).with_msg("修改成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("修改失败: {:?}", e), None),
        }
    }

    ////////

    /// # 4. [CASE] -  删除
    pub async fn handler_del_collect(
        user_id: i64,
        video_id: i64,
        //collect_id: i64,
    ) -> AppData<String> {

        // Call Case
        match CollectCase::case_del_collect(user_id, video_id).await {
            Ok(_) => AppData::ok("收藏成功".to_string()).with_msg("删除成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("删除失败: {:?}", e), None),
        }
    }

    ////////



    ////////



    ////////


}

////////