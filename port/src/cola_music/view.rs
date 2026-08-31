// port/src/cola_music/view.rs
// ⏩️ 端口 - 可乐音乐 - 浏览
// 2026/8/22 23:45 Created.

////////

#[async_trait::async_trait]
pub trait ViewPort: Send + Sync {
    async fn add_view_record(&self, uid: i64, music_id: i64, is_liked: bool) -> anyhow::Result<()>;
    async fn done_view_record(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;
    async fn user_delete_view_record(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;
    async fn sync_delete_view_record_by_user_id(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;
    async fn sync_delete_view_record_by_music_id(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;
    async fn auto_delete_view_record_by_music_id(&self, uid: i64, music_id: i64, time_range: i64) -> anyhow::Result<()>;
}

////////

/// # [VIEW PORTS] - 音乐浏览
/// * `desc`: `音乐浏览记录端口`
#[derive(Clone)]
pub struct MusicViewPort {
    pub manage: std::sync::Arc<dyn ViewPort + Send + Sync + 'static>,
}

//////// END