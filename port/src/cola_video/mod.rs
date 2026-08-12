// port/src/cola_video/mod.rs
// ⏩️ 端口 - ▶ 可乐视频 -  mod
// 2026/6/10 06:40 Created.

////////

use crate::cola_video::buy::VideoBuyPort;
use crate::cola_video::collect::VideoCollectPort;
use crate::cola_video::comment::VideoCommentPort;
use crate::cola_video::danmaku::VideoDanmakuPort;
use crate::cola_video::dislike::VideoDislikePort;
use crate::cola_video::hotlist::VideoHotlistPort;
use crate::cola_video::like::VideoLikePort;
use crate::cola_video::recommend::VideoRecommendPort;
use crate::cola_video::share::VideoSharePort;
use crate::cola_video::video::VideoPort;
use crate::cola_video::view::VideoViewPort;
use std::sync::Arc;

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
pub mod share; // 分享
pub mod video; // 视频(主内容)
pub mod view; // 浏览

////////

/// # [COLA VIDEO PORTS] - 视频
/// * `desc`: `▶ 可乐视频 - Cola Video Service Port`
#[derive(Clone)]
pub struct ColaVideoPort {
    pub buy: VideoBuyPort,
    pub collect: VideoCollectPort,
    pub comment: VideoCommentPort,
    pub danmaku: VideoDanmakuPort,
    pub dislike: VideoDislikePort,
    pub hotlist: VideoHotlistPort,
    pub like: VideoLikePort,
    pub recommend: VideoRecommendPort,
    pub share: VideoSharePort,
    pub video: VideoPort,
    pub view: VideoViewPort,
}

//////// END
