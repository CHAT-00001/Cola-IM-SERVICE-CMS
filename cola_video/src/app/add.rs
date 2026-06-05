// cola_video/src/app/add.rs  -- VIDEO - 应用层 - 添加
// 2026/4/12 14:45

////////

use crate::biz;
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
use crate::model::vo::video::VideoSingleResponse;

////////
pub struct CaseAdd;

impl CaseAdd {
    /// # 1. [CASE] - 发布视频（需要创作者/视频发布特定权限）
    pub async fn case_add_video(
        user_id: i64,
        cmd: VideoCommand,
    ) -> AppData<VideoSingleResponse> {

        // 1. 业务级权限检查 - 纯函数调用，不走 Trait 弯弯绕绕
        if let Err(e) = check_video_permission(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无视频发布权限: {:?}", e),
                None,
            );
        }

        // 2. 执行核心发布逻辑
        match biz::add::logic_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [CASE] -  发布评论（普通用户基础评论权限检查）
    pub async fn case_add_comment(
        user_id: i64,
        cmd: CommentCommand,
    ) -> AppData<String> {

        // 1. 业务级权限检查 - 比如检查是否被全站禁言、是否满足评论门槛
        if let Err(e) = check_comment_permission(user_id).await {
            return AppData::err(
                error::INTERNAL_ERROR,
                format!("无法发表评论: {:?}", e),
                None,
            );
        }

        // 2. 执行核心评论逻辑
        match biz::add::logic_add_comment(user_id, cmd).await {
            Ok(_) => AppData::ok("发布成功".to_string()).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////

    /// # 3. [CASE] -  发布弹幕
    pub async fn case_add_danmaku(
        user_id: i64,
        cmd: DanmakuCommand,
    ) -> AppData<String> {
        // 如果弹幕也需要独立校验，这里可以同样加上对应的纯函数
        match biz::add::logic_add_danmaku(Some(user_id), cmd).await {
            Ok(_) => AppData::ok("发布成功".to_string()).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////

    /// # 4. [CASE] -  添加收藏
    pub async fn case_add_collect(
        user_id: i64,
        video_id: i64,
        mut cmd: CollectCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match biz::add::logic_add_collect(user_id, video_id, cmd).await {
            Ok(_) => AppData::ok("收藏成功".to_string()).with_msg("收藏成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("收藏失败: {:?}", e), None),
        }
    }

    ////////

    /// # 5. [CASE] -  添加 分享
    pub async fn case_add_share(
        user_id: i64,
        video_id: i64,
        mut cmd: ShareCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match biz::add::logic_add_share(user_id, video_id, cmd).await {
            Ok(_) => AppData::ok("分享成功".to_string()).with_msg("分享成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("分享失败: {:?}", e), None),
        }
    }

    ////////

    /// # 6. [CASE] -  送上热门
    pub async fn case_add_hotlist(
        user_id: i64,
        video_id: i64,
        mut cmd: HotlistCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match biz::add::logic_add_hotlist(user_id, cmd).await {
            Ok(_) => AppData::ok("上热门成功".to_string()).with_msg("上热门成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("上热门失败: {:?}", e), None),
        }
    }

    ////////

    /// # 7. [CASE] - 添加 - 举报
    pub async fn case_add_report(
        user_id: i64,
        video_id: i64,
        mut cmd: ReportCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match biz::add::logic_add_report(user_id, cmd).await {
            Ok(_) => AppData::ok("举报成功".to_string()).with_msg("举报成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("举报失败: {:?}", e), None),
        }
    }

    ////////

    /// # 8. [CASE] -  添加 - 购买
    pub async fn case_add_buy(
        user_id: i64,
        video_id: i64,
        mut cmd: BuyCommand,
    ) -> AppData<String> {
        cmd.video_id = video_id;

        match biz::add::logic_add_buy(user_id, cmd).await {
            Ok(_) => AppData::ok("购买成功".to_string()).with_msg("购买成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("购买失败: {:?}", e), None),
        }
    }
}

////////