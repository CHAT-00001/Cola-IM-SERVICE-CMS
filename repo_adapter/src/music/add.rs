// repo_adapter/src/music/add.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::video::port::add::AddPort;
use cola_data::video::command::video::VideoCommand;
use crate::video::add::AddPortAdapter;

pub struct MusicAddPortAdapter;

#[async_trait]
impl AddPort for MusicAddPortAdapter {
    async fn add_video(&self, uid: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddPortAdapter.add_video(uid, data).await
    }
    async fn edit_video(&self, uid: i64, video_id: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddPortAdapter.edit_video(uid, video_id, data).await
    }
    async fn del_one_video(&self, uid: i64, video_id: i64) -> anyhow::Result<()> {
        AddPortAdapter.del_one_video(uid, video_id).await
    }
    async fn del_many_video(&self, uid: i64, video_ids: Vec<i64>) -> anyhow::Result<()> {
        AddPortAdapter.del_many_video(uid, video_ids).await
    }
}