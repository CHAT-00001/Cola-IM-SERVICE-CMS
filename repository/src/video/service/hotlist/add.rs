// repository/src/video/service/hotlist/active
// 仓储 - VIDEO - service - hotlist - add 发布 服务
// 2026/8/2 19:07 Created.

////////

use cola_data::video::command::buy::VideoBuyCommand;
use cola_data::video::command::hotlist::HotlistCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::command::report::VideoReportCommand;
use cola_data::video::entity::video::video::VideoEntity;
use crate::video::pg::video::video::VideoRepo;

////////

/// # [SERVICE] - 视频 上热门 发布 服务
pub struct VideoHotlistAddService;

// 构造实现
impl VideoHotlistAddService {
    //

    ////////

    /// # 1. [SERVICE] - 上热门 + 扣费扣积分
    pub async fn save_hotlist_and_update_count(
        _uid: i64,
        _cmd: HotlistCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 生成热门加热订单，扣除对应虚拟币

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 推荐
    pub async fn save_recommend_and_update_count(
        _uid: i64,
        _cmd: RecommendCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 创作者通过特殊权益将视频送上推荐流记录

        Ok(())
    }

}

//////// END
