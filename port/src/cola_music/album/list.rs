// port/src/music/album/list.rs
// ⏩️ 端口 - 可乐音乐 - 专辑 - 列表端口
// 2026/8/24 12:14 Created.

////////

use cola_data::music::info::album::MusicAlbumInfo;
use std::sync::Arc;

////////

/// # [MUSIC ALBUM LIST PORTS] - 音乐专辑列表端口
#[async_trait::async_trait]
pub trait MusicAlbumListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 👤 最新
    async fn get_new_album_infos_list(
        &self,
        uid: i64,                // 操作者 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("最新的专辑信息列表"))
    }

    ////////

    /// # 2. [PORT] - 👤 热门
    async fn get_hot_album_infos_list(
        &self,
        uid: i64,                // 操作者 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("热门的专辑信息列表"))
    }

    ////////

    /// # 3. [PORT] - 👤 推荐
    async fn get_recommend_album_infos_list(
        &self,
        uid: i64,                // 操作者 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("推荐的专辑信息列表"))
    }

    ////////

    /// # 4. [PORT] - 👤 附近
    async fn get_nearby_album_infos_list(
        &self,
        uid: i64,                // 操作者 ID
        keyword: Option<String>, // 关键词
        lat: f64,                // 纬度
        lng: f64,                // 经度
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("附近的专辑信息列表"))
    }

    ////////

    /// # 5. [PORT] - 👤 用户
    async fn get_user_album_infos_list(
        &self,
        uid: i64,                // 操作者 ID
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        Err(anyhow::anyhow!("我的专辑信息列表"))
    }
}

//////// END
