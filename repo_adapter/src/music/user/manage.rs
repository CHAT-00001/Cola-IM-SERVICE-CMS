// repo_adapter/src/music/like/manage.rs -- 🔌 适配器 - 可乐音乐 - 用户资料 - 管理适配器
// 2026/8/23 00:20 Created.

////////

use chrono::{DateTime, Utc};
use cola_data::music::info::music::MusicInfo;
use port::cola_music::user::manage::MusicUserManagePort;

////////

/// # [MANAGE ADAPTER] - 音乐用户资料管理适配器
/// * `desc`: `COLA MUSIC - User Profile Manage Adapter`
pub struct MusicUserManageAdapter;

#[async_trait::async_trait]
impl MusicUserManagePort for MusicUserManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn get_admin_list_infos(
        &self,
        uid: Option<i64>,
        key: Option<String>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        todo!()
    }
}

//////// END
