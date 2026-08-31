// port/src/music/like/manage.rs -- 端口 - 可乐音乐 - 点赞记录 - 管理端口
// 2026/8/23 00:10 Created.

////////

use cola_data::music::info::like::MusicLikeInfo;

////////

/// # [MANAGE PORT] - 最喜欢管理
/// * `desc`: `管理员或运营人员查询 favorite 记录`
#[async_trait::async_trait]
pub trait MusicLikeManagePort: Send + Sync {
    async fn admin_list_records(
        &self,
        operator_uid: i64,     // 管理员 ID
        user_id: Option<i64>,  // 用户 ID
        music_id: Option<i64>, // 音乐 ID
        status: Option<i16>,   // 状态
        limit: i64,            // 数量
        offset: i64,           // 偏移
    ) -> anyhow::Result<(Vec<MusicLikeInfo>, u64)> {
        Err(anyhow::anyhow!("音乐最喜欢管理适配器尚未装配"))
    }
}

//////// END
