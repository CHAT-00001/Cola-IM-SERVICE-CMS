// repo_adapter/src/music/like/manage.rs -- 🔌 适配器 - 可乐音乐 - 点赞记录 - 管理
// 2026/8/23 00:20 Created.

////////

use cola_data::music::info::like::MusicLikeInfo;
use port::cola_music::like::manage::MusicLikeManagePort;

////////

/// # [MANAGE ADAPTER] - 音乐点赞记录管理适配器
/// * `desc`: `COLA MUSIC - Like Manage Adapter.`
pub struct MusicLikeManageAdapter;

#[async_trait::async_trait]
impl MusicLikeManagePort for MusicLikeManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 管理列表
    async fn admin_list_records(
        &self,
        operator_uid: i64,
        user_id: Option<i64>,
        music_id: Option<i64>,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicLikeInfo>, u64)> {
        todo!()
    }
}

//////// END
