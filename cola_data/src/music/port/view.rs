// cola_data/src/music/port/view.rs  -- 数据中心 - MUSIC - port - 浏览
// 2026/6/10 07:13

////////

/// # [PORT] - 开始浏览
#[async_trait::async_trait]
pub trait ViewPort: Send + Sync {
    ////////

    /// # 1. [PORT] - ▶ 🆕 添加浏览
    async fn add_view_record(&self, uid: i64, music_id: i64, is_liked: bool) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - ▶ ✅️ 完成浏览
    async fn done_view_record(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - ❌️ 👤 用户主动软删除浏览记录(支持多条)
    async fn user_delete_view_record(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - ❌️ ⚙️ 同步软删除浏览记录(用户被删除/注销/永封时)
    async fn sync_delete_view_record_by_user_id(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - ❌️ ⚙️ 同步软删除浏览记录(音乐被删除时)
    async fn sync_delete_view_record_by_music_id(&self, uid: i64, music_id: i64, is_unliked: bool) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - ❌️ ⏰️ 定时任务硬删除过期的浏览记录(定时任务扫描)
    async fn auto_delete_view_record_by_music_id(&self, uid: i64, music_id: i64, time_range: i64) -> anyhow::Result<()>;
}
