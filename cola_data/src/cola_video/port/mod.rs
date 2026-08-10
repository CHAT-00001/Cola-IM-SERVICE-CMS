// data/src/cola_video/port/mod.rs
// 🗄 数据 - ▶ 可乐视频 - port - 模块
// 2026/6/10 06:40 Created.

////////

use crate::cola_gis::port::add::AddPort;
use crate::cola_video::port::buy::BuyPort;
use crate::cola_video::port::collect::VideoCollectPort;
use crate::cola_video::port::danmaku::DanmakuPort;
use crate::cola_video::port::dislike::DislikePort;
use crate::cola_video::port::hotlist::HotlistPort;
use crate::cola_video::port::like::LikePort;
use crate::cola_video::port::recommend::RecommendPort;
use crate::cola_video::port::report::ReportPort;
use crate::cola_video::port::share::VideoSharePort;
use crate::cola_video::port::video::VideoPort;
use crate::cola_video::port::view::VideoViewPort;
use std::sync::Arc;
use crate::cola_video::port::comment::VideoCommentPort;
////////

pub mod ban; // 封禁
pub mod buy; // 购买
pub mod collect; // 收藏
pub mod comment; // 评论
pub mod danmaku; // 弹幕
pub mod dislike; // 不喜欢
pub mod hotlist; // 上热门
pub mod like; // 点赞
pub mod recommend; // 推荐
pub mod report; // 举报
pub mod share; // 分享
pub mod video; // 视频(主内容)
pub mod view; // 浏览

////////

/// # [COLA VIDEO PORTS] - 视频
/// * `desc`: `可乐视频 Cola Video Service Port`
#[derive(Clone)]
pub struct ColaVideoPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    pub buy: BuyPort,
    pub collect: VideoCollectPort,
    pub comment: VideoCommentPort,
    pub danmaku: DanmakuPort,
    pub dislike: DislikePort,
    pub hotlist: HotlistPort,
    pub like: LikePort,
    pub recommend: RecommendPort,
    pub report: ReportPort,
    pub share: VideoSharePort,
    pub video: VideoPort,
    pub view: VideoViewPort,
}

//////// END
