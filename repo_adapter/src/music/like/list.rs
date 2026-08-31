// repo_adapter/src/music/like/list.rs -- 🔌 适配器 - 可乐音乐 - 点赞 - 记录列表适配器
// 2026/8/23 00:20 Created.

////////

use cola_data::music::info::like::MusicLikeInfo;
use port::cola_music::like::list::MusicLikeListPort;

////////

/// # [LIST ADAPTER] - 音乐点赞记录列表适配器
/// * `desc`: `COLA MUSIC - Like List Adapter.`
pub struct MusicLikeListAdapter;

#[async_trait::async_trait]
impl MusicLikeListPort for MusicLikeListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 列表记录
    async fn list_records(
        &self,
        operator_uid: i64,
        user_id: Option<i64>,
        music_id: Option<i64>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<MusicLikeInfo>> {
        todo!()
    }
}

//////// END
