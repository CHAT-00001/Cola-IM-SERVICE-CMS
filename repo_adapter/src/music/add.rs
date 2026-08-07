// repo_adapter/src/cola_music/active -- 适配器 - MUSIC - ADD
// 2026-06-12

////////

use crate::video::add::AddPortAdapter;
use async_trait::async_trait;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::port::add::AddPort;

////////

/// # [🔌 ADAPTER] - 音乐 适配器
pub struct MusicAddPortAdapter;

// 构造实现
#[async_trait]
impl AddPort for MusicAddPortAdapter {
    //

    ////////

    /// # 1. 💾 SAVE 保存
    async fn add_video(&self, uid: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddPortAdapter.add_video(uid, data).await
    }

    ////////

    /// # 2. 🍅 EDIT 保存
    async fn edit_video(&self, uid: i64, video_id: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddPortAdapter.edit_video(uid, video_id, data).await
    }

    ////////

    /// # 3. ❌️ 单个删除
    async fn del_one_video(&self, uid: i64, video_id: i64) -> anyhow::Result<()> {
        AddPortAdapter.del_one_video(uid, video_id).await
    }

    ////////

    /// # 4. ❌️ 批量软删除
    async fn del_many_video(&self, uid: i64, video_ids: Vec<i64>) -> anyhow::Result<()> {
        AddPortAdapter.del_many_video(uid, video_ids).await
    }
}

//////// END
