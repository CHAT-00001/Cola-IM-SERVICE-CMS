// repo_adapter/src/fs/media/add.rs
// 🔌 适配器 - FS - 媒体文件 - 发布
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::command::media::CreateMediaCmd;
use cola_data::cola_fs::entity::media::MediaEntity;
use port::fs::media::add::MediaAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `FS - 媒体发布适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaAddAdapter;

#[async_trait]
impl MediaAddPort for MediaAddAdapter {
    ////////

    /// # 1. [ADAPTER] - 创建媒体资源
    /// * `desc`: `创建媒体记录（支持普通图片、视频、LivePhoto、HLS）`
    async fn create_media(
        &self,
        _uid: i64,
        _cmd: CreateMediaCmd,
    ) -> Result<MediaEntity> {
        // TODO: 实现媒体创建逻辑
        todo!("create_media")
    }

    ////////

    /// # 2. [ADAPTER] - 创建 LivePhoto 媒体
    /// * `desc`: `创建 LivePhoto 媒体，包含封面图、静态图、短视频三个文件`
    async fn create_livephoto_media(
        &self,
        _uid: i64,
        _cover_file_id: i64,
        _main_file_id: i64,
        _aux_file_id: i64,
    ) -> Result<MediaEntity> {
        // TODO: 实现 LivePhoto 媒体创建逻辑
        todo!("create_livephoto_media")
    }
}

//////// END
