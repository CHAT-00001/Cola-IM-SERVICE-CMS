// service/src/video/hotlist/add.rs -- 服务 - VIDEO - 上热门 - 发布服务
// 2026/8/2 19:07 Created.

////////

use cola_data::cola_video::command::hotlist::VideoHotlistCommand;
use cola_data::cola_video::command::recommend::VideoRecommendCommand;

////////

/// # [ADD SERVICE] -  视频上热门发布服务
/// * `desc`: `VIDEO Hotlist Add Service.`
pub struct VideoHotlistAddService;

// 构造实现
impl VideoHotlistAddService {
    //

    ////////

    /// # 1. [SERVICE] - 上热门 + 扣费扣积分
    pub async fn save_hotlist_and_update_count(
        _uid: i64,
        _cmd: VideoHotlistCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 生成热门加热订单，扣除对应虚拟币

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 推荐
    pub async fn save_recommend_and_update_count(
        _uid: i64,
        _cmd: VideoRecommendCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 创作者通过特殊权益将视频送上推荐流记录

        Ok(())
    }
}

//////// END
