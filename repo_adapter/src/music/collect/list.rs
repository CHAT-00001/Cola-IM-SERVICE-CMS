// repo_adapter/src/music/collect/list.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 列表适配器
// 2026/8/24 16:04 Created.

////////

use port::cola_music::collect::list::MusicCollectListPort;
////////

/// # [MUSIC COLLECT CHECK ADAPTER] - 音乐收藏检查适配器
pub struct MusicCollectListAdapter;
#[async_trait::async_trait]
impl MusicCollectListPort for MusicCollectListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏记录
    async fn get_collect_record_by_user_id(&self, uid: i64, user_id: i64, limit: i64, offset: i64) -> anyhow::Result<()> {
        todo!()
    }


    async fn get_collect_record_by_music_id(&self, uid: i64, music_id: i64, limit: i64, offset: i64) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
