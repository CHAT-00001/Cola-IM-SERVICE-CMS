// repo_adapter/src/music/view.rs
// 🔌 适配器 - MUSIC - VIEW
// 2026/8/10 20:23 Created.

////////

use async_trait::async_trait;
use port::cola_music::view::ViewPort;
use port::cola_music::view::MusicViewPort;
use std::sync::Arc;

////////

/// # [VIEW ADAPTER] - 音乐浏览
/// * `desc`: `🎶 音乐浏览适配器`
pub struct MusicViewPortAdapter;

#[async_trait]
impl ViewPort for MusicViewPortAdapter {
    ////////

    /// # [PORT] - 添加浏览
    async fn add_view_record(
        &self,
        _uid: i64,
        _music_id: i64,
        _is_liked: bool,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 完成浏览
    async fn done_view_record(
        &self,
        _uid: i64,
        _music_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 用户主动软删除浏览记录
    async fn user_delete_view_record(
        &self,
        _uid: i64,
        _music_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 同步软删除浏览记录(用户被删除/注销/永封时)
    async fn sync_delete_view_record_by_user_id(
        &self,
        _uid: i64,
        _music_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 同步软删除浏览记录(音乐被删除时)
    async fn sync_delete_view_record_by_music_id(
        &self,
        _uid: i64,
        _music_id: i64,
        _is_unliked: bool,
    ) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [PORT] - 定时任务硬删除过期的浏览记录
    async fn auto_delete_view_record_by_music_id(
        &self,
        _uid: i64,
        _music_id: i64,
        _time_range: i64,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

////////

/// # [BUILD] - 构建浏览 Port
/// * `desc`: `装配音乐浏览适配器`
pub fn build_view_port() -> MusicViewPort {
    MusicViewPort {
        manage: Arc::new(MusicViewPortAdapter),
    }
}

//////// END
