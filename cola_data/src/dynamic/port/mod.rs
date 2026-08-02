// cola_cola_data/src/new/port/mod.rs  -- 数据中心 - AUTH - port
// 2026-06-10 06:40

////////

use crate::video::port::add::AddPort;
use crate::video::port::collect::CollectRepo;
use crate::video::port::comment::CommentRepo;
use crate::video::port::danmaku::DanmakuRepo;
use crate::video::port::feed::FeedRepo;
use crate::video::port::hot::HotlistRepo;
use crate::video::port::like::LikeRepo;
use crate::video::port::report::ReportRepo;
use crate::video::port::share::ShareRepo;
use crate::video::port::view::ViewPort;
use std::sync::Arc;
use crate::video::port::buy::BuyRepo;

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

/// # [S] - 短视频 ServicePort
#[derive(Clone)]
pub struct ColaVideoPort {
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
