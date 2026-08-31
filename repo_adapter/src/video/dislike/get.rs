// repo_adapter/src/cola_video/dislike/get.rs -- 🔌 适配器 - VIDEO - 不喜欢 - 获取适配器
// 2026/8/6 18:58 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::dislike::VideoDislikeInfo;
use port::cola_video::dislike::get::VideoDislikeGetPort;
use port::cola_video::dislike::list::VideoDislikeListPort;

////////

/// # [GET ADAPTER] - dislike list
/// * `desc`: `视频不喜欢记录列表适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDislikeGetAdapter;

#[async_trait]
impl VideoDislikeGetPort for VideoDislikeGetAdapter {
    //

    ////////
    async fn get_my_dislike_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_dislike_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
