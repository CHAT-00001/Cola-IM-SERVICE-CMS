// port/src/music/music/manage.rs
// ⏩️ 端口 - 可乐音乐 - 音乐 - 列表
// 2026/8/22 23:30 Created.

////////

use chrono::{DateTime, Utc};
use cola_data::music::info::music::MusicInfo;

////////

/// # [MANAGE PORTS] - 管理
#[async_trait::async_trait]
pub trait MusicUserManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取管理员视角的音乐信息列表
    async fn get_admin_list_infos(
        &self,
        uid: Option<i64>,                  // 作者 ID
        key: Option<String>,               // 关键词
        start_time: Option<DateTime<Utc>>, // 开始时间
        end_time: Option<DateTime<Utc>>,   // 结束时间
        limit: i64,                        // 数量
        offset: i64,                       // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐管理适配器尚未装配"))
    }
}

//////// END
