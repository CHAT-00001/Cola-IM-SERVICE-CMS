// repo_adapter/src/music/album/manage.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 管理适配器
// 2026/8/24 13:32 Created.

////////

use chrono::{DateTime, Utc};
use cola_data::music::info::album::MusicAlbumInfo;
use port::cola_music::album::manage::MusicAlbumManagePort;

////////

/// # [MANAGE ADAPTER] - 音乐专辑管理适配器
/// * `desc`: `MUSIC - album manage adapter`
pub struct MusicAlbumManageAdapter;

#[async_trait::async_trait]
impl MusicAlbumManagePort for MusicAlbumManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 管理列表
    async fn get_mange_list(
        &self,
        uid: i64,
        user_id: Option<i64>,
        cate_id: Option<i64>,
        channel_id: Option<i64>,
        keyword: Option<String>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicAlbumInfo>)> {
        todo!()
    }
}

//////// END
