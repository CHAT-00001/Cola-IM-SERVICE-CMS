// repo_adapter/src/live/stream.rs -- 🔌 适配器 - LIVE - 直播场次
// 2026/8/21 09:38 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_data::cola_live::info::record::LiveRecordInfo;
use port::cola_live::stream::add::LiveStreamAddPort;
use port::cola_live::stream::check::LiveStreamCheckPort;
use port::cola_live::stream::get::LiveStreamGetPort;
use port::cola_live::stream::list::LiveStreamListPort;
use port::cola_live::stream::manage::LiveStreamManagePort;
use repository::cola_live::pg::stream_record_repo::LiveStreamRecordRepo;
use tracing::{error, info};

////////

#[derive(Debug, Default, Clone)]
pub struct LiveStreamAdapter;

#[async_trait]
impl LiveStreamAddPort for LiveStreamAdapter {
    async fn start(&self, uid: i64, command: LiveRecordCommand) -> anyhow::Result<LiveRecordInfo> {
        Ok(LiveStreamRecordRepo::start(uid, command).await?.into())
    }
}

#[async_trait]
impl LiveStreamCheckPort for LiveStreamAdapter {
    async fn can_start(&self, uid: i64, room_id: i64) -> anyhow::Result<()> {
        if uid <= 0 || room_id <= 0 {
            anyhow::bail!("用户或直播间参数无效");
        }
        if LiveStreamRecordRepo::current(uid, room_id).await?.is_some() {
            anyhow::bail!("当前直播间已经在直播");
        }
        Ok(())
    }
}

#[async_trait]
impl LiveStreamGetPort for LiveStreamAdapter {
    async fn current(&self, uid: i64, room_id: i64) -> anyhow::Result<Option<LiveRecordInfo>> {
        Ok(LiveStreamRecordRepo::current(uid, room_id)
            .await?
            .map(Into::into))
    }
}

#[async_trait]
impl LiveStreamListPort for LiveStreamAdapter {
    async fn newest(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<LiveRecordInfo>> {
        info!(
            "[🔌 ADAPTER] - 📡 查询最新直播: limit={}, offset={}",
            limit, offset
        );
        Ok(LiveStreamRecordRepo::list(None, "new", limit, offset)
            .await
            .map_err(|err| {
                error!("[🤐 ADAPTER] - ❌️ 查询最新直播失败: {}", err);
                err
            })?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn category(
        &self,
        category_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<LiveRecordInfo>> {
        info!(
            "[🔌 ADAPTER] - 📡 查询分类直播: category_id={}, limit={}, offset={}",
            category_id, limit, offset
        );
        Ok(
            LiveStreamRecordRepo::list(Some(category_id), "new", limit, offset)
                .await
                .map_err(|err| {
                    error!(
                        "[🤐 ADAPTER] - ❌️ 查询分类直播失败: category_id={}, error={}",
                        category_id, err
                    );
                    err
                })?
                .into_iter()
                .map(Into::into)
                .collect(),
        )
    }

    async fn hot(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<LiveRecordInfo>> {
        info!(
            "[🔌 ADAPTER] - 📡 查询热门直播: limit={}, offset={}",
            limit, offset
        );
        Ok(LiveStreamRecordRepo::list(None, "hot", limit, offset)
            .await
            .map_err(|err| {
                error!("[🤐 ADAPTER] - ❌️ 查询热门直播失败: {}", err);
                err
            })?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[async_trait]
impl LiveStreamManagePort for LiveStreamAdapter {
    async fn stop(&self, uid: i64, record_id: i64) -> anyhow::Result<()> {
        LiveStreamRecordRepo::stop(uid, record_id).await
    }
}

//////// END
