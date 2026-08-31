// port/src/music/collect/delete.rs -- 端口 - MUSIC - 收藏 - 删除端口
// 2026/8/24 12:06 Created.

////////

////////

/// # [MUSIC COLLECT DELETE PORTS] - 音乐收藏删除端口
/// * `desc`: `COLA MUSIC - Collect Delete Ports.`
#[async_trait::async_trait]
pub trait MusicCollectDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - ❌️ 👤 用户软删除音乐收藏记录(支持批量)
    async fn user_delete_collect_record(
        &self,
        uid: i64,
        music_ids: Vec<i64>,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐收藏适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - ❌️ 👤 根据用户ID软删除音乐收藏记录(用户注销/永封/删除时)
    async fn sync_delete_collect_record_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("用户注销 / 删除 时, 同步删除TA的收藏记录"))
    }

    ////////

    /// # 3. [PORT] - ❌️ 🎶 根据音乐ID软删除音乐收藏记录(音乐失效时)
    async fn sync_delete_collect_record_by_music_id(
        &self,
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("音乐删除时, 同步删除TA被收藏的记录"))
    }
}

//////// END
