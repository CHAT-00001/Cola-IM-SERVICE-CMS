// port/src/music/like/list.rs -- 端口 - 可乐音乐 - 点赞记录 - 列表端口
// 2026/8/23 00:10 Created.

////////

use cola_data::music::info::like::MusicLikeInfo;

////////

/// # [LIST PORT] - 最喜欢操作记录
/// * `desc`: `提供 favorite 操作记录，供审计和溯源使用`
#[async_trait::async_trait]
pub trait MusicLikeListPort: Send + Sync {
    async fn list_records(
        &self,
        operator_uid: i64,       // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        music_id: Option<i64>,   // 音乐 ID
        status: Option<i16>,     // 状态
        limit: i64,              // 数量
        offset: i64,             // 偏移
    ) -> anyhow::Result<Vec<MusicLikeInfo>> {
        Err(anyhow::anyhow!("音乐最喜欢审计列表适配器尚未装配"))
    }
}

//////// END