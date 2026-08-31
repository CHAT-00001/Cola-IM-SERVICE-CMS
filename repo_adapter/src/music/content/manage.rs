// repo_adapter/src/music/content/manage.rs
// 🔌 适配器 - 可乐音乐 - 主内容 - 管理
// 2026/8/23 00:30 Created.

////////

/// # [MUSIC CONTENT MANAGE ADAPTER] - 管理
/// * `desc`: `音乐主内容管理专用适配器`
pub struct MusicContentManageAdapter;

#[async_trait::async_trait]
impl port::cola_music::music::manage::MusicManagePort for MusicContentManageAdapter {}

//////// END