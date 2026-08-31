// repo_adapter/src/music/content/del.rs
// 🔌 适配器 - 可乐音乐 - 主内容 - 删除
// 2026/8/23 00:30 Created.

////////

use port::cola_music::music::del::MusicDelPort;

////////

/// # [MUSIC CONTENT DEL ADAPTER] - 删除
/// * `desc`: `音乐主内容删除专用适配器`
pub struct MusicContentDelAdapter;

#[async_trait::async_trait]
impl MusicDelPort for MusicContentDelAdapter {}

//////// END