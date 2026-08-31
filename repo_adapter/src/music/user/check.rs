// repo_adapter/src/music/user/check.rs -- 🔌 适配器 - MUSIC - 用户资料 - 检查适配器
// 2026/8/31 19:38 Created.

////////

use port::cola_music::user::check::MusicUserCheckPort;

////////

/// # [CHECK ADAPTER] - 音乐用户资料检查适配器
pub struct MusicUserCheckAdapter;

#[async_trait::async_trait]
impl MusicUserCheckPort for MusicUserCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn health(&self, music_id: i64) -> anyhow::Result<i16> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 状态
    async fn state(&self, music_id: i64) -> anyhow::Result<(i16)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 归属
    async fn is_owner(&self, user_id: i64, music_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 备用
    async fn is_xxxx(&self, user_id: i64, music_id: i64) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
