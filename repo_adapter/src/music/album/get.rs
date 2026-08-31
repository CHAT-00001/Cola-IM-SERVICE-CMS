// repo_adapter/src/music/album/get.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 检查适配器
// 2026/8/24 15:55 Created.

////////

use cola_data::music::info::album::MusicAlbumInfo;
use port::cola_music::album::get::MusicAlbumGetPort;

////////

/// # [GET ADAPTER] - 音乐专辑获取适配器
pub struct MusicAlbumGetAdapter;

#[async_trait::async_trait]
impl MusicAlbumGetPort for MusicAlbumGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    async fn get_album_info_by_id(
        &self,
        uid: i64,
        album_id: i64,
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量
    async fn batch_get_album_infos_by_ids(
        &self,
        uid: i64,
        album_ids: Vec<i64>,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }
}

//////// END
