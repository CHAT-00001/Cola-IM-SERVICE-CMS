// repo_adapter/src/music/content/list.rs
// 🔌 适配器 - 可乐音乐 - 主内容 - 列表
// 2026/8/23 00:30 Created.

////////

/// # [MUSIC CONTENT LIST ADAPTER] - 列表
/// * `desc`: `音乐主内容列表专用适配器`
pub struct MusicContentListAdapter;

#[async_trait::async_trait]
impl port::cola_music::music::list::MusicListPort for MusicContentListAdapter {}

//////// END