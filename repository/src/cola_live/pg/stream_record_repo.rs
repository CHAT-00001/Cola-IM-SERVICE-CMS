// repository/src/cola_live/pg/stream_record_repo.rs
// 仓储 - LIVE - PG - 直播场次记录
// 2026/8/21 09:35 Created.

////////

use crate::pg_pool;
use anyhow::{Context, Result, anyhow};
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_data::cola_live::entity::stream::stream_record::LiveStreamRecordEntity;

////////

const RECORD_COLUMNS: &str = "id, _id, uid, room_id, show_id, live_type, nums, title, province, city, thumb, pull, stream, channel_id, push_url, pull_flv, pull_hls, is_mic, is_hot, is_recommend, likes, recommends, is_off, anyway, pk_uid, pk_stream, video_url, address, lng, lat, type_val, device_info, game_action, voice_type, sw_player_status, sw_player_id, sw_pull_url, recommend_time, status, is_deleted, start_at, end_at, deleted_at";
const INSERT_COLUMNS: &str = "_id, uid, room_id, show_id, live_type, nums, title, province, city, thumb, pull, stream, channel_id, push_url, pull_flv, pull_hls, is_mic, is_hot, is_recommend, likes, recommends, is_off, anyway, pk_uid, pk_stream, video_url, address, lng, lat, type_val, device_info, game_action, voice_type, sw_player_status, sw_player_id, sw_pull_url, recommend_time, status, is_deleted, start_at, end_at, deleted_at";

////////

/// # 1. [REPOSITORY] - 直播场次仓储
pub struct LiveStreamRecordRepo;

impl LiveStreamRecordRepo {
    /// # 1. [REPOSITORY] - 开播
    /// * `desc`: `校验房间归属、选择启用推流节点并写入直播场次`
    pub async fn start(uid: i64, command: LiveRecordCommand) -> Result<LiveStreamRecordEntity> {
        if uid <= 0 || command.room_id <= 0 {
            return Err(anyhow!("用户ID或直播间ID无效"));
        }
        let pool = pg_pool();
        let room_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM cola_live.live_room WHERE id=$1 AND uid=$2 AND status=1 AND COALESCE(is_deleted,false)=false)")
            .bind(command.room_id).bind(uid).fetch_one(&pool).await.context("检查直播间失败")?;
        if !room_exists {
            return Err(anyhow!("直播间不存在、未启用或不属于当前用户"));
        }
        let provider = sqlx::query_as::<_, (String, String, String, String)>("SELECT push_domain, play_flv_domain, play_hls_domain, app_name FROM cola_live.live_stream_provider WHERE status=1 ORDER BY sort ASC, id ASC LIMIT 1")
            .fetch_optional(&pool).await.context("选择推流节点失败")?.ok_or_else(|| anyhow!("没有可用的推流服务器"))?;
        let entity = command.into_entity(uid, &provider.0, &provider.1, &provider.2);
        let sql = format!(
            "INSERT INTO cola_live.stream_record ({}) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36,$37,$38,$39,$40,$41) RETURNING {}",
            INSERT_COLUMNS, RECORD_COLUMNS
        );
        sqlx::query_as::<_, LiveStreamRecordEntity>(&sql)
            .bind(entity._id)
            .bind(entity.uid)
            .bind(entity.room_id)
            .bind(entity.show_id)
            .bind(entity.live_type)
            .bind(entity.nums)
            .bind(entity.title)
            .bind(entity.province)
            .bind(entity.city)
            .bind(entity.thumb)
            .bind(entity.pull)
            .bind(entity.stream)
            .bind(entity.channel_id)
            .bind(entity.push_url)
            .bind(entity.pull_flv)
            .bind(entity.pull_hls)
            .bind(entity.is_mic)
            .bind(entity.is_hot)
            .bind(entity.is_recommend)
            .bind(entity.likes)
            .bind(entity.recommends)
            .bind(entity.is_off)
            .bind(entity.anyway)
            .bind(entity.pk_uid)
            .bind(entity.pk_stream)
            .bind(entity.video_url)
            .bind(entity.address)
            .bind(entity.lng)
            .bind(entity.lat)
            .bind(entity.type_val)
            .bind(entity.device_info)
            .bind(entity.game_action)
            .bind(entity.voice_type)
            .bind(entity.sw_player_status)
            .bind(entity.sw_player_id)
            .bind(entity.sw_pull_url)
            .bind(entity.recommend_time)
            .bind(entity.status)
            .bind(entity.is_deleted)
            .bind(entity.start_at)
            .bind(entity.end_at)
            .bind(entity.deleted_at)
            .fetch_one(&pool)
            .await
            .context("写入直播场次失败")
    }

    /// # 2. [REPOSITORY] - 停播
    pub async fn stop(uid: i64, record_id: i64) -> Result<()> {
        let pool = pg_pool();
        let result = sqlx::query("UPDATE cola_live.stream_record SET status=0, is_off=1, end_at=NOW() WHERE id=$1 AND uid=$2 AND status=1")
            .bind(record_id).bind(uid).execute(&pool).await.context("停止直播失败")?;
        if result.rows_affected() == 0 {
            return Err(anyhow!("直播场次不存在或已停止"));
        }
        Ok(())
    }

    /// # 3. [REPOSITORY] - 当前场次
    pub async fn current(uid: i64, room_id: i64) -> Result<Option<LiveStreamRecordEntity>> {
        let pool = pg_pool();
        Ok(sqlx::query_as::<_, LiveStreamRecordEntity>(&format!("SELECT {} FROM cola_live.stream_record WHERE uid=$1 AND room_id=$2 AND status=1 AND COALESCE(is_deleted,false)=false ORDER BY id DESC LIMIT 1", RECORD_COLUMNS)).bind(uid).bind(room_id).fetch_optional(&pool).await.context("查询当前直播失败")?)
    }

    /// # 4. [REPOSITORY] - 前台列表
    pub async fn list(
        filter: Option<i64>,
        order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LiveStreamRecordEntity>> {
        let pool = pg_pool();
        let order_sql = if order == "hot" {
            "likes DESC, recommends DESC, id DESC"
        } else {
            "start_at DESC, id DESC"
        };
        let sql = format!(
            "SELECT {} FROM cola_live.stream_record WHERE status=1 AND COALESCE(is_deleted,false)=false AND ($1::BIGINT IS NULL OR channel_id=$1) ORDER BY {} LIMIT $2 OFFSET $3",
            RECORD_COLUMNS, order_sql
        );
        Ok(sqlx::query_as::<_, LiveStreamRecordEntity>(&sql)
            .bind(filter)
            .bind(limit.clamp(1, 50))
            .bind(offset.max(0))
            .fetch_all(&pool)
            .await
            .context("查询直播列表失败")?)
    }
}

//////// END
