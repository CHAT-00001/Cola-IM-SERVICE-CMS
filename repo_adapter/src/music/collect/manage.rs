// repo_adapter/src/music/collect/manage.rs
// 🔌 适配器 - 可乐音乐 - 收藏 - 发布
// 2026/8/24 16:04 Created.

////////

use port::cola_music::collect::manage::MusicCollectManagePort;

////////

/// # [MUSIC COLLECT MANAGE ADAPTER] - 音乐收藏管理适配器
pub struct MusicCollectManageAdapter;
#[async_trait::async_trait]
impl MusicCollectManagePort for MusicCollectManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存收藏记录
    async fn get_admin_list(
        &self,
        uid: i64,
        user_id: Option<i64>,
        music_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
