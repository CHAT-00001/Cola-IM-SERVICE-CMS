// cola_music/port/add.rs
// 音乐 - port - 添加
// 2026/6/10 06:35

////////

use crate::cola_music::command::music::new::MusicCommand;

////////

/// # [PORT] - 添加
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 💾 保存
    async fn save_music_record(&self, uid: i64, data: MusicCommand) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - ⚙️ 编辑
    async fn edit_music(&self, uid: i64, music_id: i64, data: MusicCommand) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - ❌️ 👤 用户批量软删除音乐(支持批量)
    async fn user_delete_by_music_ids(&self, uid: i64, music_ids: Vec<i64>) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - ❌️ ⏰️ 自动任务硬删除过期的项目
    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()>;
}

//////// END
