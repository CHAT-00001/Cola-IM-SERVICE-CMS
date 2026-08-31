// repository/src/cola_gis/pg/active -- 仓储 - GIS - PG - add
// 2026/7/6 14:10

////////

use crate::pg_pool;
use cola_data::cola_gis::command::poi::PoiCommand;
use cola_data::cola_gis::entity::poi::{GIS_POI_COLUMNS, PoiEntity};
use sqlx;

////////

#[derive(Debug, sqlx::FromRow)]
pub struct GisHomeRow {
    #[sqlx(flatten)]
    pub entity: PoiEntity,
    #[sqlx(default)]
    pub distance: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,
    MostViews,
    MostLikes,
    Latest,
}

/// # [ADD REPOSITORY] - POI REPO
pub struct AddRepository;

impl AddRepository {
    // 💡

    ////////

    /// # 1. [REPOSITORY] - ✅️ 👤 用户保存一个新的兴趣点
    pub async fn pg_save_gis_by_uid(
        uid: i64,
        cmd: PoiCommand,
        visibility: i16,
    ) -> Result<PoiEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_gis.poi (user_id, title, description, href, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) RETURNING {}",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(uid)
            .bind(cmd.title)
            .bind(cmd.description)
            .bind(cmd.href)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 🌬️ 👤 用户编辑一个兴趣点
    pub async fn pg_update_gis_by_id(
        poi_id: i64,
        cmd: PoiCommand,
        visibility: i16,
    ) -> Result<PoiEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE cola_gis.poi SET title=$2, description=$3, href=$4, visibility=$5, updated_at=NOW() \
             WHERE id=$1 RETURNING {}",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(poi_id)
            .bind(cmd.title)
            .bind(cmd.description)
            .bind(cmd.href)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # x. [REPOSITORY] - 用户软删除兴趣点(支持多个)
    pub async fn pg_delete_poi_by_ids(poi_ids: Vec<i64>) -> Result<PoiEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = r#"
            UPDATE cola_gis.poi SET is_del = 1, del_time = EXTRACT(EPOCH FROM NOW())::BIGINT, deleted_at = NOW()
            WHERE id = $1 AND is_del = 0
            RETURNING id, title, description, href, visibility, status, is_deleted, del_time, deleted_at, created_at, updated_at"#;
        sqlx::query_as::<_, PoiEntity>(query)
            .bind(poi_ids)
            .fetch_one(&pool)
            .await
    }

    pub async fn find_new_list_by_user_id(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
            GIS_POI_COLUMNS
        );
        sqlx::query_as::<_, PoiEntity>(&query)
            .bind(user_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool)
            .await
    }
}

//////// END
