// repo_adapter/src/gis/mod.rs
// 🔌 插头 - 可乐 GIS - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_gis::ColaGisPort;
use std::sync::Arc;

////////

pub mod add;
pub mod buy;
pub mod collect;
pub mod comment;
pub mod danmaku;
pub mod feed;
pub mod hot;
pub mod like;
pub mod report;
pub mod share;
pub mod view;

////////

/// # [BUILD] - 构建 GIS Port
/// * `desc`: 构建 GIS Port 聚合体，包含所有位置相关功能
pub fn build_gis_port() -> ColaGisPort {
    ColaGisPort {
        add: Arc::new(add::AddPortAdapter),
        buy: Arc::new(buy::BuyPortAdapter),
        feed: Arc::new(feed::FeedPortAdapter),
        hotlist: Arc::new(hot::HotlistPortAdapter),
        collect: Arc::new(collect::CollectPortAdapter),
        comment: Arc::new(comment::CommentPortAdapter),
        danmaku: Arc::new(danmaku::DanmakuPortAdapter),
        share: Arc::new(share::SharePortAdapter),
        like: Arc::new(like::LikePortAdapter),
        report: Arc::new(report::ReportPortAdapter),
        view: Arc::new(view::ViewPortAdapter),
    }
}

//////// END
