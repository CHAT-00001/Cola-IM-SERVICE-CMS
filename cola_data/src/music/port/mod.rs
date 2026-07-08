// cola_cola_data/src/video/port/mod.rs  -- 数据中心 - AUTH - port
// 2026-06-10 06:40

////////

use std::sync::Arc;
use crate::video::port::add::AddPort;
use crate::video::port::like::LikeRepo;
use crate::video::port::view::ViewPort;

////////
pub mod add;
pub mod like;
pub mod view;
mod feed;
mod home;
mod collect;
////////

/// # [SERVICE PORT] - 音乐服务端口
#[derive(Clone)]
pub struct MusicServicePorts {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
   // pub feed: Arc<dyn FeedPort>,
   // pub collect: Arc<dyn CollectPort>,
   // pub state: Arc<dyn StatePort>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}
