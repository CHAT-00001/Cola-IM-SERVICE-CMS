// repository/src/new/redis/home  -- 仓储 - VIDEO - redis - 视频内容 - 缓存仓储
// 2026/6/8 23:03

////////
use app_config::DbService;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::video::VideoInfo;
use redis::AsyncCommands;

////////

/// # [CACHE] - 视频缓存
#[derive(Clone)]
pub struct VideoCache {
    db: DbService,
}

/// # [CACHE] - 视频缓存
impl VideoCache {
    //

    ////////

    /// # [DB] - 新建连接
    pub fn new(db: DbService) -> Self {
        Self { db }
    }

    fn key(video_id: i64) -> String {
        format!("new:info:{}", video_id)
    }

    ////////

    /// # 1. [CACHE] - 获取视频信息
    pub async fn get_video_info(&self, video_id: i64) -> anyhow::Result<Option<VideoInfo>> {
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

    ////////

    /// # 2. [CACHE] - 设置视频信息
    pub async fn set_video_info(&self, video: VideoInfo, ttl_secs: usize) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();

        let key = Self::key(video.id);

        let json = serde_json::to_string(&video)?;

        let _: () = conn.set_ex(key, json, ttl_secs as u64).await?;

        Ok(())
    }

    ////////

    /// # 3. [CACHE] - 数据表转换
    pub async fn set_from_entity(
        &self,
        entity: VideoEntity,
        ttl_secs: usize,
    ) -> anyhow::Result<()> {
        let info = VideoInfo::from_entity(entity);

        self.set_video_info(info, ttl_secs).await
    }

    ////////

    /// # 4. [CACHE] - 删除视频信息
    pub async fn del_video_info(&self, video_id: i64) -> anyhow::Result<()> {
        let mut conn = self.db.redis_conn.clone();

        let key = Self::key(video_id);

        let _: () = conn.del(key).await?;

        Ok(())
    }
}

//////// END
