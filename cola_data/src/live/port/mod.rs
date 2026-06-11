// /mod.rs  -- 
// 2026/6/10 07:44

////////

use std::sync::Arc;
use crate::video::port::add::AddPort;
use crate::video::port::like::LikeRepo;
use crate::video::port::view::ViewPort;

#[derive(Clone)]
pub struct ColaLivePort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    // pub feed: Arc<dyn FeedPort>,
    // pub collect: Arc<dyn CollectPort>,
    // pub state: Arc<dyn StatePort>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}
