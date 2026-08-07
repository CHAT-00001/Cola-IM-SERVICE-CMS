// cola_data/src/cola_gis/port/mod.rs  -- 数据中心 - GIS - port
// 2026-06-10 06:40

////////

use crate::cola_gis::port::add::AddPort;
use crate::cola_gis::port::collect::CollectRepo;
use crate::cola_gis::port::comment::CommentRepo;
use crate::cola_gis::port::danmaku::DanmakuRepo;
use crate::cola_gis::port::feed::FeedRepo;
use crate::cola_gis::port::hot::HotlistRepo;
use crate::cola_gis::port::like::LikeRepo;
use crate::cola_gis::port::report::ReportRepo;
use crate::cola_gis::port::share::ShareRepo;
use crate::cola_gis::port::view::ViewPort;
use std::sync::Arc;
use crate::cola_gis::port::buy::BuyRepo;

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

/// # [SERVICE PORT] - 地理信息服务 ServicePort
#[derive(Clone)]
pub struct ColaGisPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    pub buy: Arc<dyn BuyRepo + Send + Sync + 'static>,
    pub feed: Arc<dyn FeedRepo + Send + Sync + 'static>,
    pub hotlist: Arc<dyn HotlistRepo + Send + Sync + 'static>,
    pub collect: Arc<dyn CollectRepo + Send + Sync + 'static>,
    pub comment: Arc<dyn CommentRepo + Send + Sync + 'static>,
    pub danmaku: Arc<dyn DanmakuRepo + Send + Sync + 'static>,
    pub share: Arc<dyn ShareRepo + Send + Sync + 'static>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub report: Arc<dyn ReportRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}
