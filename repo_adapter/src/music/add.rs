// repo_adapter/src/music/add.rs
// 🔌 适配器 - MUSIC - ADD
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::cola_music::command::music::new::MusicCommand;
use port::cola_music::add::AddPort;
////////

/// # [🔌 ADAPTER] - 音乐 适配器
pub struct MusicAddPortAdapter;

// 构造实现
#[async_trait]
impl AddPort for MusicAddPortAdapter {
    //

    ////////

    /// # 1. 💾 SAVE 保存
    async fn save_music_record(&self, uid: i64, data: MusicCommand) -> anyhow::Result<()> {
        todo!()
    }

    async fn edit_music(&self, uid: i64, music_id: i64, data: MusicCommand) -> anyhow::Result<()> {
        todo!()
    }

    async fn user_delete_by_music_ids(&self, uid: i64, music_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }

    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
