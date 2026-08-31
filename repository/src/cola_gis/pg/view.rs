// repository/src/cola_gis/pg/get  -- 仓储 - GIS - pg - 浏览
// 2026/7/6

use crate::pg_pool;
use cola_data::cola_gis::command::poi::PoiCommand;
use cola_data::cola_gis::entity::poi::PoiEntity;
use sqlx;

const GIS_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, title, title_at_uids, description, desc_at_uids,
    thumb, thumb_s, href, href_w, original_url, tags, lat, lng, duration,
    width, height, fps, bit, views, likes, steps, collects, comments,
    done_play_qty, visibility, allow_comment, allow_danmaku, shares,
    is_public, status, music_id, goods_id, addtime, created_at, updated_at
"#;

#[derive(Debug, sqlx::FromRow)]
pub struct GisHomeRow {
    #[sqlx(flatten)]
    pub entity: PoiEntity,
    #[sqlx(default)]
    pub distance: Option<f64>,
}

pub struct GisViewRepo;

impl GisViewRepo {
    pub async fn pg_find_new_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE status = 1 ORDER BY addtime DESC LIMIT $1 OFFSET $2",
            GIS_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn pg_find_hot_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE status = 1 ORDER BY likes DESC, views DESC, addtime DESC LIMIT $1 OFFSET $2",
            GIS_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn pg_find_nearby_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GisHomeRow>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance FROM cola_gis.cola_gis WHERE status = 1 ORDER BY distance ASC LIMIT $3 OFFSET $4",
            GIS_COLUMNS
        );
        sqlx::query_as::<_, GisHomeRow>(&query)
            .bind(lat)
            .bind(lng)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn pg_find_one_by_id(id: i64) -> Result<Option<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE id = $1 AND status = 1 LIMIT 1",
            GIS_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    pub async fn find_visited_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT gis_id FROM cola_gis.gis_visited WHERE user_id = $1 AND status = 1 ORDER BY addtime DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn find_all_batch_ids(ids: &[i64]) -> Result<Vec<PoiEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE id = ANY($1) AND status = 1",
            GIS_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    pub async fn pg_batch_uids_find_list(
        uids: Vec<i64>,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE status = 1",
            GIS_COLUMNS
        );
        if !uids.is_empty() {
            sql.push_str(" AND uid = ANY($1)");
        }
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                sql.push_str(" AND (title ILIKE $2 OR description ILIKE $2)");
            }
        }
        sql.push_str(" ORDER BY addtime DESC LIMIT $3 OFFSET $4");
        let mut query = sqlx::query_as::<_, PoiEntity>(&sql);
        query = query.bind(uids);
        query = query.bind(format!("%{}%", keyword.unwrap_or_default()));
        query = query.bind(limit).bind(offset);
        query.fetch_all(&pool).await
    }

    pub async fn pg_find_new_list_by_uid(
        uid: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!(
            "SELECT {} FROM cola_gis.cola_gis WHERE status = 1",
            GIS_COLUMNS
        );
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                sql.push_str(" AND (title ILIKE $2 OR description ILIKE $2)");
            }
        }
        sql.push_str(" ORDER BY addtime DESC LIMIT $3 OFFSET $4");
        let mut query = sqlx::query_as::<_, PoiEntity>(&sql);
        query = query.bind(uid);
        query = query.bind(format!("%{}%", keyword.unwrap_or_default()));
        query = query.bind(limit).bind(offset);
        query.fetch_all(&pool).await
    }
}
