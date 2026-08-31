// repo_adapter/src/music/album/list.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 列表适配器
// 2026/8/24 13:32 Created.

////////

use cola_data::music::info::album::MusicAlbumInfo;
use port::cola_music::album::list::MusicAlbumListPort;

////////

/// # [LIST ADAPTER] - 音乐专辑列表适配器
pub struct MusicAlbumListAdapter;

#[async_trait::async_trait]
impl MusicAlbumListPort for MusicAlbumListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 最新
    async fn get_new_album_infos_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 热门
    async fn get_hot_album_infos_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 推荐
    async fn get_recommend_album_infos_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 附近
    async fn get_nearby_album_infos_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }

    ////////

    /// # 5. [ADAPTER] - 用户
    async fn get_user_album_infos_list(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }
}

//////// END
