// port/src/cola_dynamic/port/mod.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 模块
// 2026/6/10 06:40 Created.

////////

use crate::cola_dynamic::buy::DynamicBuyPort;
use crate::cola_dynamic::collect::DynamicCollectPort;
use crate::cola_dynamic::comment::DynamicCommentPort;
use crate::cola_dynamic::danmaku::DynamicDanmakuPort;
use crate::cola_dynamic::dislike::DynamicDislikePort;
use crate::cola_dynamic::dynamic::DynamicPort;
use crate::cola_dynamic::hotlist::DynamicHotlistPort;
use crate::cola_dynamic::like::DynamicLikePort;
use crate::cola_dynamic::recommend::DynamicRecommendPort;
use crate::cola_dynamic::share::DynamicSharePort;
use crate::cola_dynamic::view::DynamicViewPort;
use std::sync::Arc;

////////
pub mod ban;
pub mod buy;
pub mod collect;
pub mod comment;
pub mod danmaku;
pub mod dislike;
pub mod dynamic;
pub mod hotlist;
pub mod like;
pub mod recommend;
pub mod share;
pub mod view;

////////

/// # [COLA DYNAMIC PORT] - 可乐动态
/// * `desc`: `⏹可乐动态 - Cola Dynamic Service Ports`
#[derive(Clone)]
pub struct ColaDynamicPort {
    pub add: DynamicPort,                // 动态
    pub buy: DynamicBuyPort,             // 购买
    pub collect: DynamicCollectPort,     // 收藏
    pub comment: DynamicCommentPort,     // 评论
    pub danmaku: DynamicDanmakuPort,     // 弹幕
    pub dislike: DynamicDislikePort,     // 不喜欢
    pub hotlist: DynamicHotlistPort,     // 上热门
    pub like: DynamicLikePort,           // 点赞
    pub recommend: DynamicRecommendPort, // 推荐
    pub share: DynamicSharePort,         // 分享
    pub view: DynamicViewPort,           // 浏览
}

//////// END
