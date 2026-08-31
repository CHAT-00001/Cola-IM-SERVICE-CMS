// port/src/music/add.rs -- 端口 - 可乐音乐 - 内容 - 添加
// 2026/6/10 06:35 Created.

////////

use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::info::music::MusicInfo;

////////

/// # [ADD PORTS] - 发布
#[async_trait::async_trait]
pub trait MusicAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 创建音乐
    /// * `desc`: `返回 - 音乐信息`
    async fn create_music(
        &self,
        uid: i64,                // 操作者 ID
        cmd: MusicCreateCommand, // 新建命令
        visibility: i16,         // 风控生成的可见范围
    ) -> anyhow::Result<MusicInfo>;

    ////////

    /// # 2. [PORT] - 更新音乐
    /// * `desc`: `返回 - 音乐信息`
    async fn update_music(
        &self,
        uid: i64,                // 操作者 ID
        music_id: i64,           // 音乐 ID
        cmd: MusicUpdateCommand, // 更新命令
        visibility: i16,         // 风控生成的可见范围
    ) -> anyhow::Result<(MusicInfo)>;

    ////////

    /// # 3. [PORT] - ❌️ 👤 用户批量软删除音乐(支持批量)
    async fn user_delete_by_music_ids(
        &self,
        uid: i64,            // 操作者 ID
        music_ids: Vec<i64>, // 音乐 IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - ❌️ ⏰️ 自动任务硬删除过期的项目
    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()>;
}

//////// END
