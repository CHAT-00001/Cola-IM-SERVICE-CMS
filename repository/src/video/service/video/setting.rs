// service/setting  - 服务层 - 用户设置
// 2026/6/9 19:15

////////

use cola_data::video::command::report::VideoReportCommand;
use cola_data::video::command::buy::VideoBuyCommand;
use cola_data::video::info::video::VideoInfo;
use crate::video::pg::video::video::VideoRepo;
////////

/// # [SETTING SERVICE] - 设置服务
pub struct VideoUserSettingService;

impl VideoUserSettingService {

    /// # 7001. [SERVICE] - 获取用户发布的视频列表
    pub async fn find_user_publish_list(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> { // 🚀 纯净升级：返回值改为 Vec<VideoInfo>

        // 1. 从 DB 捞出原始 Entity 列表
        let db_videos = VideoRepo::find_new_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取用户{}发布的最新视频列表失败: {}", user_id, e))?;

        // 2. ✅ 纯内存无损转换，不带任何缓存花活
        let video_infos = db_videos
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect();

        Ok(video_infos)
    }

    ////////

    ////////

    /// # 8. [SERVICE] - 记录举报信息
    pub async fn save_report_info(
        _uid: i64,
        _cmd: VideoReportCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 写入后台内容风控待人工审核表

        Ok(())
    }

    ////////

    /// # 9. [SERVICE] - 购买内容
    pub async fn save_buy_and_update_count(
        _uid: i64,
        _cmd: VideoBuyCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 购买付费视频/电商挂载商品落单逻辑

        Ok(())
    }

}

//////// END