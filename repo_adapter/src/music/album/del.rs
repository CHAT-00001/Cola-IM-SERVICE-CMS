// repo_adapter/src/music/album/del.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 删除适配器
// 2026/8/24 13:32 Created.

////////

use port::cola_music::album::delete::MusicAlbumDeletePort;

////////

/// # [DELETE ADAPTER] - 音乐专辑删除适配器
pub struct MusicAlbumDeleteAdapter;

#[async_trait::async_trait]
impl MusicAlbumDeletePort for MusicAlbumDeleteAdapter {}

//////// END