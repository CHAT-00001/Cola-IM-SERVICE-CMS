// port/src/music/user/stat.rs -- 端口 - 可乐音乐 - 用户 - 添加端口
// 2026/8/31 19:46 Created.

////////

use cola_data::music::command::user::MusicUserCreateCommand;
use cola_data::music::info::user::MusicUserInfo;

////////

/// # [STAT PORTS] - 音乐用户资料统计端口
#[async_trait::async_trait]
pub trait MusicUserStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 统计所有用户资料数量
    /// * `desc`: `返回 - 音乐信息`
    async fn stat_profile_count(
        &self,
        uid: i64, // 操作者 ID
    ) -> anyhow::Result<u64>;

    ////////

    /// # 2. [PORT] - 更新音乐
    /// * `desc`: `返回 - 音乐信息`
    async fn update_profile(
        &self,
        uid: i64,                    // 操作者 ID
        music_id: i64,               // 音乐 ID
        cmd: MusicUserCreateCommand, // 更新命令
    ) -> anyhow::Result<(MusicUserInfo)>;
}

//////// END
