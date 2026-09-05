// repo_adapter/src/fs/media/add.rs -- 适配器 - FS - 媒体文件 - 发布
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::command::media::CreateMediaCmd;
use cola_data::cola_fs::entity::media::MediaEntity;
use port::fs::media::add::MediaAddPort;
use repository::cola_fs::pg::media::MediaRepo;
use repository::pg_pool;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `FS - 媒体发布适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaAddAdapter;

#[async_trait]
impl MediaAddPort for MediaAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建媒体资源
    /// * `desc`: `创建媒体记录（支持普通图片、视频、LivePhoto、HLS）`
    async fn create_media(&self, _uid: i64, cmd: CreateMediaCmd) -> Result<MediaEntity> {
        // TODO: 实现媒体创建逻辑
        let entity = MediaRepo::create(&pg_pool(), cmd).await?;
        tracing::info!("[🔌 ADAPTER] - ✅️ 媒体记录创建成功: media_id={}", entity.id);
        Ok(entity)
    }

    ////////

    /// # 2. [ADAPTER] - 创建 LivePhoto 媒体
    /// * `desc`: `创建 LivePhoto 媒体，包含封面图、静态图、短视频三个文件`
    async fn create_livephoto_media(
        &self,
        _uid: i64,
        cover_file_id: i64,
        main_file_id: i64,
        aux_file_id: i64,
    ) -> Result<MediaEntity> {
        // TODO: 实现 LivePhoto 媒体创建逻辑
        self.create_media(
            0,
            CreateMediaCmd {
                _id: None,
                app_id: None,
                media_type: 3,
                cover_file_id: Some(cover_file_id),
                main_file_id: Some(main_file_id),
                aux_file_id: Some(aux_file_id),
                hls_playlist_url: None,
                variants_meta: None,
                duration: None,
                width: None,
                height: None,
            },
        )
        .await
    }
}

//////// END
