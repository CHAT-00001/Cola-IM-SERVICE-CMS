// repositoryo/src/gis/pg/gis.rs -- 仓储 - GIS - PG - 兴趣点
// 2026/7/6 14:10

////////

use crate::pg_pool;
use cola_data::gis::command::poi::PoiCommand;
use cola_data::gis::entity::poi::{PoiEntity, GIS_POI_COLUMNS};
use sqlx::{self, Postgres, QueryBuilder};

////////
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,
    MostViews,
    MostLikes,
    Latest,
}


/// # [POI REPOSITORY] - 兴趣点 仓储
pub struct PoiRepo;

// 构造实现
impl PoiRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 最新
    pub async fn find_new_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE status = 1 ORDER BY add_time DESC LIMIT $1 OFFSET $2",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门
    pub async fn find_hot_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE status = 1 ORDER BY likes DESC, views DESC, add_time DESC LIMIT $1 OFFSET $2",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 3. [REPOSITORY] - 推荐
    pub async fn find_recommend_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE status = 1 ORDER BY RANDOM() LIMIT $1 OFFSET $2",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 4. [REPOSITORY] - 附近
    pub async fn find_nearby_list(lat: f64, lng: f64, limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance
             FROM cola_gis.poi WHERE status = 1
             ORDER BY distance ASC LIMIT $3 OFFSET $4",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(lat).bind(lng).bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 5. [REPOSITORY] - 分类
    pub async fn find_category_list(category_id: i16, limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE status = 1 AND category_id = $1 ORDER BY likes DESC, add_time DESC LIMIT $2 OFFSET $3",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(category_id).bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 6. [REPOSITORY] - 精选
    pub async fn find_featured_list(limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE status = 1 ORDER BY likes DESC, add_time DESC LIMIT $1 OFFSET $2",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(limit).bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 7. [REPOSITORY] - 搜索
    pub async fn search_keyword_list(
        keyword: String, lat: f64, lng: f64,
        start_time: Option<i64>, end_time: Option<i64>,
        order_by: Option<SearchOrder>, limit: i64, offset: i64,
    ) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!(
            "SELECT {}, SQRT(POW(lat - $1, 2) + POW(lng - $2, 2)) AS distance FROM cola_gis.poi WHERE status = 1",
            GIS_POI_COLUMNS
        );
        let mut param_index = 3;
        sql.push_str(&format!(" AND title LIKE ${}", param_index));
        param_index += 1;
        if start_time.is_some() { sql.push_str(&format!(" AND add_time >= ${}", param_index)); param_index += 1; }
        if end_time.is_some() { sql.push_str(&format!(" AND add_time <= ${}", param_index)); param_index += 1; }
        match order_by.unwrap_or(SearchOrder::Distance) {
            SearchOrder::Distance => sql.push_str(" ORDER BY distance ASC"),
            SearchOrder::MostViews => sql.push_str(" ORDER BY views DESC, distance ASC"),
            SearchOrder::MostLikes => sql.push_str(" ORDER BY likes DESC, distance ASC"),
            SearchOrder::Latest => sql.push_str(" ORDER BY add_time DESC, distance ASC"),
        }
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_index, param_index + 1));

        let keyword_like = format!("%{}%", keyword);
        let mut query = sqlx::query_as::<_, PoiEntity>(&sql).bind(lat).bind(lng);
        query = query.bind(&keyword_like);
        if let Some(start) = start_time { query = query.bind(start); }
        if let Some(end) = end_time { query = query.bind(end); }
        query.bind(limit).bind(offset).fetch_all(&pool).await
    }

    ////////

    /// # 10. [REPOSITORY] - 根据ID单个查找兴趣点
    pub async fn find_by_id(id: i64) -> Result<Option<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE id = $1 AND status = 1 LIMIT 1",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(id).fetch_optional(&pool).await
    }

    ////////

    /// # 9. [REPOSITORY] - 根据IDs查找兴趣点
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<PoiEntity>, sqlx::Error> {
        if ids.is_empty() { return Ok(vec![]); }
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE id = ANY($1) AND status = 1",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(ids).fetch_all(&pool).await
    }

    ////////

    /// # 10. [REPOSITORY] - 💾 👤 保存新的兴趣点
    pub async fn save_poi_by_uid(uid: i64, cmd: PoiCommand, visibility: i16) -> Result<PoiEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_gis.poi (user_id, title, description, href, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) RETURNING {}",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(uid).bind(cmd.title).bind(cmd.description).bind(cmd.href).bind(visibility)
            .fetch_one(&pool).await
    }

    ////////

    /// # 11. [REPOSITORY] - 最新
    pub async fn sync_decrement_danmaku_count_by_num(video_id: i64, count: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            UPDATE cola_gis.poi SET danmaku_count = GREATEST(danmaku_count - $1, 0), updated_at = NOW()
            WHERE id = $2 RETURNING danmaku_count"#;
        sqlx::query_scalar(query).bind(count).bind(video_id).fetch_one(&pool).await
    }

    ////////

    /// # 12. [REPOSITORY] - 根据用户ID查找TA发布的兴趣点
    pub async fn find_new_list_by_user_id(user_id: i64, offset: i64, limit: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(user_id).bind(offset).bind(limit)
            .fetch_all(&pool).await
    }

    ////////

    /// # 13. [REPOSITORY] - 用户们 发布的兴趣点
    pub async fn find_list_by_uids(uids: Option<Vec<i64>>, keyword: Option<String>, limit: i64, offset: i64) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let mut sql = format!("SELECT {} FROM cola_gis.poi WHERE status = 1", GIS_POI_COLUMNS);
        if let Some(ref ids) = uids { if !ids.is_empty() { sql.push_str(" AND uid = ANY($1)"); } }
        if let Some(ref kw) = keyword { if !kw.is_empty() { sql.push_str(" AND (title ILIKE $2 OR description ILIKE $2)"); } }
        sql.push_str(" ORDER BY add_time DESC LIMIT $3 OFFSET $4");
        let mut query = sqlx::query_as::<_, PoiEntity>(&sql);
        query = query.bind(uids.unwrap_or_default());
        query = query.bind(format!("%{}%", keyword.unwrap_or_default()));
        query = query.bind(limit).bind(offset);
        query.fetch_all(&pool).await
    }
}

//////// END

