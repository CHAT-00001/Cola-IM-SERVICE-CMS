// repo/src/video/redis/home  -- 仓储中心 - 短视频 - Redis - 视频缓存
// 2026/6/8 23:03

////////
use redis::AsyncCommands;
use app_config::DbService;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::entity::video::VideoEntity;

#[derive(Clone)]
pub struct VideoCache {
    db: DbService,
}

impl VideoCache {
    pub fn new(db: DbService) -> Self {
        Self { db }
    }

    fn key(video_id: i64) -> String {
        format!("video:info:{}", video_id)
    }

    // =========================
    // 1. GET
    // =========================
    pub async fn get_video_info(
        &self,
        video_id: i64,
    ) -> anyhow::Result<Option<VideoInfo>> {

        let mut conn = self.db.redis_conn.clone();

        let key = Self::key(video_id);

        let val: Option<String> = conn.get(&key).await?;

        match val {
            Some(json) => {
                let data: VideoInfo = serde_json::from_str(&json)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    // =========================
    // 2. SET（直接用 VideoInfo）
    // =========================
    pub async fn set_video_info(
        &self,
        video: VideoInfo,
        ttl_secs: usize,
    ) -> anyhow::Result<()> {

        let mut conn = self.db.redis_conn.clone();

        let key = Self::key(video.id);

        let json = serde_json::to_string(&video)?;

        let _: () = conn.set_ex(key, json, ttl_secs as u64).await?;

        Ok(())
    }

    // =========================
    // 3. 从 Entity 写入（推荐入口）
    // =========================
    pub async fn set_from_entity(
        &self,
        entity: VideoEntity,
        ttl_secs: usize,
    ) -> anyhow::Result<()> {

        let info = VideoInfo::from_entity(entity);

        self.set_video_info(info, ttl_secs).await
    }

    // =========================
    // 4. DELETE
    // =========================
    pub async fn del_video_info(
        &self,
        video_id: i64,
    ) -> anyhow::Result<()> {

        let mut conn = self.db.redis_conn.clone();

        let key = Self::key(video_id);

        let _: () = conn.del(key).await?;

        Ok(())
    }
}