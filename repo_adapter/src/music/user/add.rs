// repo_adapter/src/music/user/add.rs -- 🔌 适配器 - MUSIC - 用户资料 - 发布适配器
// 2026/8/24 23:10 Created.

////////

use cola_data::music::command::user::MusicUserCreateCommand;
use cola_data::music::info::user::MusicUserInfo;
use port::cola_music::user::add::MusicUserAddPort;

////////

/// # [ADD ADAPTER] - 音乐用户资料发布适配器
/// * `desc`: `COLA MUSIC - User Profile Add Adapter.`
pub struct MusicUserAddAdapter;

#[async_trait::async_trait]
impl MusicUserAddPort for MusicUserAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建资料
    async fn create_profile(
        &self,
        uid: i64,
        cmd: MusicUserCreateCommand,
        visibility: i16,
    ) -> anyhow::Result<MusicUserInfo> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 更新资料
    async fn update_profile(
        &self,
        uid: i64,
        music_id: i64,
        cmd: MusicUserCreateCommand,
    ) -> anyhow::Result<(MusicUserInfo)> {
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
