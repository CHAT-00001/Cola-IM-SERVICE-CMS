// repo_adapter/src/cola_video/cola_video/get.rs
// 🔌 插头 - 可乐视频 - 视频 - 获取IDs
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::video::get::VideoGetPort;
use port::market::cate::get::CateGetPort;
////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct CateGetAdapter;

// 构造实现
#[async_trait]
impl CateGetPort for CateGetAdapter {
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
