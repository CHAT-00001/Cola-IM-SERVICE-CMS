// repo_adapter/src/music/user/stat.rs -- 🔌 适配器 - MUSIC - 用户资料 - 统计适配器
// 2026/8/31 19:40 Created.

////////

use cola_data::music::command::user::MusicUserCreateCommand;
use cola_data::music::info::user::MusicUserInfo;
use port::cola_music::user::stat::MusicUserStatPort;

////////

/// # [STAT ADAPTER] - 音乐用户资料统计适配器
pub struct MusicUserStatAdapter;

#[async_trait::async_trait]
impl MusicUserStatPort for MusicUserStatAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建资料
    async fn stat_profile_count(&self, uid: i64) -> anyhow::Result<u64> {
        todo!()
    }

    async fn update_profile(
        &self,
        uid: i64,
        music_id: i64,
        cmd: MusicUserCreateCommand,
    ) -> anyhow::Result<(MusicUserInfo)> {
        todo!()
    }
}

//////// END
