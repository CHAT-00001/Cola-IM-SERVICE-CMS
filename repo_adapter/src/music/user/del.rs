// repo_adapter/src/music/user/del.rs -- 🔌 适配器 - MUSIC - 用户资料 - 删除适配器
// 2026/8/31 19:40 Created.

////////

use port::cola_music::user::del::MusicUserDelPort;

////////

/// # [DELETE ADAPTER] - 音乐用户资料删除适配器
/// * `desc`: `COLA MUSIC - Profile Delete Adapter.`
pub struct MusicUserDelAdapter;

#[async_trait::async_trait]
impl MusicUserDelPort for MusicUserDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户删除
    async fn user_delete_by_music_ids(&self, uid: i64, music_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 自动删除
    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
