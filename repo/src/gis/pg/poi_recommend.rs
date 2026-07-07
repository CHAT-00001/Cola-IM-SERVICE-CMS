// repo/src/gis/pg/recommend.rs  -- 浠撳偍 - GIS - PG - 鎺ㄨ崘
// 2026/7/6

use crate::pg_pool;
use cola_data::gis::command::recommend::RecommendCommand;
use sqlx;

pub struct RecommendRepository;

impl RecommendRepository {
    pub async fn save_recommend_record(uid: i64, gis_id: i64, cmd: &RecommendCommand) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now();
        let query = "
            INSERT INTO cola_gis.gis_recommend (user_id, gis_id, remark, add_time, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id, gis_id) DO NOTHING";
        let result = sqlx::query(query)
            .bind(uid).bind(gis_id).bind(&cmd.remark)
            .bind(now.timestamp()).bind(now.naive_utc())
            .execute(&pool).await?;
        Ok(result.rows_affected())
    }
}


