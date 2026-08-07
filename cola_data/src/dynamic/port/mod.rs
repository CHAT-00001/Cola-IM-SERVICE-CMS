// dynamic/port/mod.rs
// 动态 - port - 模块
// 2026/6/10 06:40 Created.

////////

use crate::dynamic::port::buy::DynamicBuyPort;
use crate::dynamic::port::collect::CollectPort;
use crate::dynamic::port::comment::{DynamicCommentPort};
use crate::dynamic::port::danmaku::DanmakuPort;
use crate::dynamic::port::dislike::DislikePort;
use crate::dynamic::port::dynamic::DynamicPort;
use crate::dynamic::port::hotlist::HotlistPort;
use crate::dynamic::port::like::DynamicLikePort;
use crate::dynamic::port::recommend::RecommendPort;
use crate::dynamic::port::report::ReportPort;
use crate::dynamic::port::share::{DynamicSharePort};
use crate::dynamic::port::view::ViewPort;
use std::sync::Arc;

////////
mod ban;
mod buy;
mod collect;
mod comment;
mod danmaku;
mod dislike;
mod dynamic;
mod hotlist;
mod like;
mod recommend;
mod report;
mod share;
mod view;

////////

/// # [COLA DYNAMIC PORT] - 动态
/// * `desc`: `可乐动态 Cola Dynamic Service Port`
#[derive(Clone)]
pub struct ColaDynamicPort {
    pub add: DynamicPort,            // 动态端口
    pub buy: DynamicBuyPort,         // 购买端口
    pub collect: CollectPort,        // 收藏端口
    pub comment: DynamicCommentPort, // 评论端口
    pub danmaku: DanmakuPort,        // 弹幕端口
    pub dislike: DislikePort,        // 不喜欢端口
    pub hotlist: HotlistPort,        // 上热门
    pub like: DynamicLikePort,       // 点赞端口
    pub recommend: RecommendPort,    // 推荐端口
    pub report: ReportPort,          // 举报端口
    pub share: DynamicSharePort,     // 分享端口
    pub view: ViewPort,              // 浏览端口
}

//////// END
