// port/src/cola_music/music/check.rs
// ⏩️ 端口 - 可乐音乐 - 添加
// 2026/8/24 11:01 Created.

////////

use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::info::music::MusicInfo;

////////

/// # [CHECK PORTS] - 检查 端口
#[async_trait::async_trait]
pub trait MusicCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查健康分
    /// * `desc`: `返回健康分`
    async fn health(
        &self,
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<i16>;

    ////////

    /// # 2. [PORT] - 检查状态
    /// * `desc`: `返回状态码`
    async fn state(
        &self,
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<(i16)>;

    ////////

    /// # 3. [PORT] - 检查归属权
    /// * `desc`: `返回是非`
    async fn is_owner(
        &self,
        user_id: i64,  // 用户 ID
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 4. [PORT] - 备用
    /// * `desc`: `返回是非`
    async fn is_xxxx(
        &self,
        user_id: i64,  // 用户 ID
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<()>;
}

//////// END
