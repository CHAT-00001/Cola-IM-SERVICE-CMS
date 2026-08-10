// port/src/cola_music/mod.rs
// ⏩️ 端口 - 可乐音乐 - mod
// 2026-06-10 06:40 Created.

////////

use std::sync::Arc;
use crate::cola_music::add::AddPort;
use crate::cola_music::like::LikeRepo;
use crate::cola_music::view::ViewPort;

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
/// * `desc`: `🎶 可乐音乐 - Cola Music Service Ports`
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
