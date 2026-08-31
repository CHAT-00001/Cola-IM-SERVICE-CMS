// repo_adapter/src/video/dislike/add.rs
// 🔌 适配器 - ▶ 视频 - 不喜欢 - 发布
// 2026/8/9 20:52 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::dislike::add::VideoDislikeAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `▶ 视频 - 不喜欢记录发布`
#[derive(Debug, Default, Clone)]
pub struct VideoDislikeAddAdapter;

#[async_trait]
impl VideoDislikeAddPort for VideoDislikeAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布
    async fn add_dislike(
        &self,
        uid: i64,
        user_id: i64,  // 用户 ID
        video_id: i64, // 视频 ID
        dislike: bool, // 不喜欢
    ) -> Result<(bool)> {
        todo!()
    }
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
