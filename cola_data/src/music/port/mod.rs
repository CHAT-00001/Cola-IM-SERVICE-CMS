// music/port/mod.rs
// 音乐 - port - 模块
// 2026-06-10 06:40

////////

use crate::gis::port::add::AddPort;
use crate::gis::port::like::LikeRepo;
use crate::gis::port::view::ViewPort;
use std::sync::Arc;

////////
pub mod add;
pub mod collect;
pub mod favorites;
pub mod feed;
pub mod home;
pub mod like;
pub mod playlist;
pub mod view;

////////

/// # [COLA MUSIC PORT] - 音乐
/// * `desc`: `可乐音乐 Cola Music Service Ports`
#[derive(Clone)]
pub struct ColaMusicPort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    // pub favorites: Arc<dyn FavoritesPort>,
    // pub collect: Arc<dyn CollectPort>,
    // pub playlist: Arc<dyn PlaylistPort>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}

//////// END
