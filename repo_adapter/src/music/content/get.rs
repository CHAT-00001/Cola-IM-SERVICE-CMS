// repo_adapter/src/music/content/get.rs
// 🔌 适配器 - 可乐音乐 - 主内容 - 获取
// 2026/8/23 00:30 Created.

////////

/// # [MUSIC CONTENT GET ADAPTER] - 获取
/// * `desc`: `音乐主内容获取专用适配器`
pub struct MusicContentGetAdapter;

#[async_trait::async_trait]
impl port::cola_music::music::get::MusicGetPort for MusicContentGetAdapter {}

//////// END