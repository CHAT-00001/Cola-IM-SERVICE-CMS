//  bridge/src/video/adapter/view.rs  --
// 2026/5/26 11:31 by wx: cestbon10080

////////

use app_core::video::port::view::ViewPort;
use async_trait::async_trait;
use data::video::entity::video::VideoEntity;
use std::sync::Arc;
use repository::video::pg::view2::VideoViewRepo;

pub struct ViewAdapter {
    repo: Arc<VideoViewRepo>,
}

impl ViewAdapter {
    pub fn new(repo: Arc<VideoViewRepo>) -> Self {
        Self { repo }
    }
}
#[async_trait]
impl ViewPort for ViewAdapter {

    async fn find_by_id(&self, video_id: i64) -> Result<VideoEntity, String> {
        self.repo
            .find_by_id(video_id)
            .await
            .map_err(|e| format!("Bridge 查询视频失败: {}", e))
    }

    async fn find_by_ids(&self, ids: Vec<i64>) -> Result<VideoEntity, String> {
        self.repo
            .find_by_id(ids)
            .await
            .map_err(|e| format!("Bridge 查询视频列表失败: {}", e))
    }
}