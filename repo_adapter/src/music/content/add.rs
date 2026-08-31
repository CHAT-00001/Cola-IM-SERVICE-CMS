// repo_adapter/src/music/add.rs -- 🔌 适配器 - MUSIC - 内容 - ADD 适配器
// 2026-06-12 18:40

////////

use async_trait::async_trait;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::info::music::MusicInfo;
use port::cola_music::music::add::MusicAddPort;
use repository::music::pg::music::add::MusicAddRepo;

////////

/// # [ADD ADAPTER] - 音乐内容发布适配器
/// * `desc`: `COLA MUSIC - Content Add Adapter.`
pub struct MusicAddAdapter;

// 构造实现
#[async_trait]
impl MusicAddPort for MusicAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建音乐
    async fn create_music(
        &self,
        uid: i64,                // 操作者 ID
        cmd: MusicCreateCommand, // 命令
        visibility: i16,         // 可见度
    ) -> anyhow::Result<MusicInfo> {
        // Call REPO ..
        let entity = MusicAddRepo::save_music_by_uid(uid, cmd, visibility)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 保存音乐失败: {error}"))?;
        Ok(MusicInfo::from_music_entity(&entity))
    }

    ////////

    /// # 2. [ADAPTER] - 更新音乐
    async fn update_music(
        &self,
        uid: i64,                // 操作者 ID
        music_id: i64,           // 音乐 ID
        cmd: MusicUpdateCommand, // 更新命令
        visibility: i16,         // 可见度
    ) -> anyhow::Result<MusicInfo> {
        // 👈 签名改为返回 MusicInfo
        // Call REPO ..
        let entity = MusicAddRepo::update_music_by_id(uid, music_id, cmd, visibility)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 ADAPTER] - ❌️ 更新音乐失败: {error}"))?;

        Ok(MusicInfo::from_music_entity(&entity))
    }

    ////////

    /// # 3. [ADAPTER] - 更新音乐
    async fn user_delete_by_music_ids(&self, uid: i64, music_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 更新音乐
    async fn auto_delete_music_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
