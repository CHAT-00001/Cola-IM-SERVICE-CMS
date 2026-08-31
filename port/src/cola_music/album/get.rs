// port/src/music/album/get.rs
// ⏩️ 端口 - 可乐音乐 - 专辑 - 获取端口
// 2026/8/24 12:14 Created.

////////

use cola_data::music::info::album::MusicAlbumInfo;
use std::sync::Arc;

////////

/// # [MUSIC ALBUM GET PORTS] - 音乐专辑获取端口
#[async_trait::async_trait]
pub trait MusicAlbumGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 👤 单个
    async fn get_album_info_by_id(
        &self,
        uid: i64,      // 操作者 ID
        album_id: i64, // 专辑 ID
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        Err(anyhow::anyhow!("单个获取专辑信息"))
    }

    ////////

    /// # 1. [PORT] - 👤👤 批量
    async fn batch_get_album_infos_by_ids(
        &self,
        uid: i64,            // 操作者 ID
        album_ids: Vec<i64>, // 专辑 IDs
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("批量获取专辑信息列表"))
    }
}

//////// END
