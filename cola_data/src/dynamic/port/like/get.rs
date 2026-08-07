// video/port/like/get.rs
// 视频 - port - 点赞 - 获取
// 2026/8/5 01:57 Created.

////////

use crate::video::command::share::ShareCommand;

////////

/// # [GET PORTS] - 获取
/// `desc`: `视频点赞获取端口`
#[async_trait::async_trait]
pub trait LikeGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取我的点赞记录IDs
    /// * `desc`: `用户批量获取点赞的视频IDs`
    async fn get_my_like_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 获取TA的点赞记录IDs
    /// * `desc`: `用户批量获取点赞的视频IDs`
    async fn get_he_like_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(u16)>;
}

//////// END
