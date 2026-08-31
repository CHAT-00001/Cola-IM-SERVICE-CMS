// repo_adapter/src/video/dislike/stat.rs -- 🔌 插头 - VIDEO - 不喜欢 - 统计适配器
// 2026/8/9 22:28 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::dislike::stat::VideoDislikeStatPort;

////////

/// # [STAT ADAPTER] - dislike stat
/// * `desc`: `▶ 视频 - 不喜欢记录统计适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDislikeStatAdapter;

#[async_trait]
impl VideoDislikeStatPort for VideoDislikeStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
