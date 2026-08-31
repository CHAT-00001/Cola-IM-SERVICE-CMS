// port/src/music/album/delete.rs
// ⏩️ 端口 - 可乐音乐 - 专辑 - 删除端口
// 2026/8/24 12:14 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC ALBUM DELETE PORTS] - 音乐专辑删除端口
#[async_trait::async_trait]
pub trait MusicAlbumDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - ❌️ 👤 批量逻辑删除
    /// * `desc`: `根据用户ID批量逻辑删除音乐专辑`
    async fn batch_delete_by_album_ids(
        &self,
        uid: i64,            // 操作者 ID
        album_ids: Vec<i64>, // 专辑 IDs
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("根据用户ID批量逻辑删除音乐专辑"))
    }

    ////////

    /// # 2. [PORT] - ❌️ ⏰️ 自动清理
    /// * `desc`: `物理删除过期失效的专辑记录`
    async fn auto_clean_album_record_by_time_range(
        &self,
        uid: i64,
        time_range: i64,
    ) -> anyhow::Result<(u64)> {
        Err(anyhow::anyhow!("物理删除过期失效的专辑记录"))
    }
}

//////// END
