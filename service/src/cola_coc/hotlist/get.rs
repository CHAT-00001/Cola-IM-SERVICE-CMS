// service/src/video/hotlist/get.rs -- 服务 - VIDEO - 上热门 - 获取服务
// 2026/8/2 19:08 Created.

////////

use cola_data::cola_video::command::buy::VideoBuyCommand;
use cola_data::cola_video::command::hotlist::VideoHotlistCommand;
use cola_data::cola_video::command::recommend::VideoRecommendCommand;
use cola_data::cola_video::command::report::VideoReportCommand;
use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::video::pg::video::home::VideoRepo;

////////

/// # [GET SERVICE] - 视频上热门获取服务
/// * `DESC`: `VIDEO - Hotlist Get Service.`
pub struct VideoHotlistGetService;

impl VideoHotlistGetService {
    //

    ////////

    /// # 6. [SERVICE] - 上热门 + 扣费扣积分
    pub async fn save_hotlist_and_update_count(
        _uid: i64,
        _cmd: VideoHotlistCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 生成热门加热订单，扣除对应虚拟币

        Ok(())
    }

    ////////

    /// # 7. [SERVICE] - 推荐
    pub async fn save_recommend_and_update_count(
        _uid: i64,
        _cmd: VideoRecommendCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 创作者通过特殊权益将视频送上推荐流记录

        Ok(())
    }

    ////////

    /// # 7001. [SERVICE] - 获取用户发布的视频列表
    pub async fn find_user_publish_list(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!("SERVICE: 获取用户{}发布的最新视频列表失败: {}", user_id, e)
            })
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
