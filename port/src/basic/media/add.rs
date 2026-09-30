// port/src/fs/media/add.rs
// 🔌 端口 - FS - 媒体 - 新增
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::command::media::CreateMediaCmd;
use cola_data::cola_fs::entity::media::MediaEntity;

////////

/// # [ADD PORT] - 媒体新增
/// * `desc`: `FS - 媒体新增端口（支持 LivePhoto）`
#[async_trait::async_trait]
pub trait MediaAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 创建媒体资源
    /// * `desc`: `创建媒体记录（支持普通图片、视频、LivePhoto、HLS）`
    async fn create_media(&self, uid: i64, cmd: CreateMediaCmd) -> Result<MediaEntity>;

    ////////

    /// # 2. [PORT] - 创建 LivePhoto 媒体
    /// * `desc`: `创建 LivePhoto 媒体，包含封面图、静态图、短视频三个文件`
    async fn create_livephoto_media(
        &self,
        uid: i64,
        cover_file_id: i64,
        main_file_id: i64,
        aux_file_id: i64,
    ) -> Result<MediaEntity>;
}

//////// END
