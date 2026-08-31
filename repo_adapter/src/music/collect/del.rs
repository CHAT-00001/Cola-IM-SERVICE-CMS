// repo_adapter/src/music/collect/del.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/24 16:04 Created.

////////

use port::cola_music::collect::delete::MusicCollectDeletePort;

////////

/// # [MUSIC COLLECT DELETE ADAPTER] - 音乐收藏删除适配器
pub struct MusicCollectDeleteAdapter;
#[async_trait::async_trait]
impl MusicCollectDeletePort for MusicCollectDeleteAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏记录
    async fn sync_delete_collect_record_by_user_id(&self, user_id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn sync_delete_collect_record_by_music_id(&self, music_id: i64) -> anyhow::Result<()> {
        todo!()
    }

    async fn user_delete_collect_record(&self, uid: i64, music_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }

}

//////// END
