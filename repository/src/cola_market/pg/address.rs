// repository/src/cola_market/pg/address.rs  -- 仓储 - MARKET - pg - 地址
// 2026/6/18 14:10

////////

use cola_data::cola_market::entity::address::AddressEntity;
use crate::pg_pool;

////////

/// # [ADDRESS REPOSITORY] - 地址簿 仓储
pub struct AddressRepo;

impl AddressRepo {
    // 💡

    const COLUMNS: &'static str = r#"
        id, uid, name, country, province, city, area,
        address, area_code, phone, is_default,
        add_time, upd_time, create_at, update_at, id_del, deleted_at
    "#;

    ////////

    /// # 1. [REPOSITORY] - 插入地址
    pub async fn insert(
        entity: &AddressEntity,
    ) -> Result<AddressEntity, sqlx::Error> {
        let pool = pg_pool();
        let row = sqlx::query_as::<_, AddressEntity>(
            &format!(
                "INSERT INTO shop_address ({}) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17) RETURNING *",
                Self::COLUMNS
            )
        )
        .bind(&entity.uid)
        .bind(&entity.name)
        .bind(&entity.country)
        .bind(&entity.province)
        .bind(&entity.city)
        .bind(&entity.area)
        .bind(&entity.address)
        .bind(&entity.area_code)
        .bind(&entity.phone)
        .bind(&entity.is_default)
        .bind(&entity.add_time)
        .bind(&entity.upd_time)
        .bind(&entity.create_at)
        .bind(&entity.update_at)
        .bind(&entity.id_del)
        .bind(&entity.deleted_at)
        .fetch_one(&pool)
        .await?;
        Ok(row)
    }

    ////////

    /// # 2. [REPOSITORY] - 更新地址
    pub async fn update(
        id: i64,
        entity: &AddressEntity,
    ) -> Result<AddressEntity, sqlx::Error> {
        let pool = pg_pool();
        let row = sqlx::query_as::<_, AddressEntity>(
            "UPDATE shop_address SET name=$1, country=$2, province=$3, city=$4, area=$5, address=$6, area_code=$7, phone=$8, is_default=$9, upd_time=$10 WHERE id=$11 RETURNING *"
        )
        .bind(&entity.name)
        .bind(&entity.country)
        .bind(&entity.province)
        .bind(&entity.city)
        .bind(&entity.area)
        .bind(&entity.address)
        .bind(&entity.area_code)
        .bind(&entity.phone)
        .bind(&entity.is_default)
        .bind(&entity.upd_time)
        .bind(id)
        .fetch_one(&pool)
        .await?;
        Ok(row)
    }

    ////////

    /// # 3. [REPOSITORY] - 按用户ID查地址列表
    pub async fn find_by_uid(
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<AddressEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_address WHERE uid = $1 AND id_del = 0 ORDER BY is_default DESC, add_time DESC LIMIT $2 OFFSET $3",
            Self::COLUMNS
        );
        sqlx::query_as::<_, AddressEntity>(&query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 按ID查地址
    pub async fn find_by_id(
        uid: i64,
        id: i64,
    ) -> Result<Option<AddressEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_address WHERE uid = $1 AND id = $2 AND id_del = 0 LIMIT 1",
            Self::COLUMNS
        );
        sqlx::query_as::<_, AddressEntity>(&query)
            .bind(uid)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 按ID删除（软删除）
    pub async fn soft_delete(id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("UPDATE shop_address SET id_del = 1 WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 6. [REPOSITORY] -  设置默认
    pub async fn set_default(uid: i64, id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        // 先清除所有默认
        sqlx::query("UPDATE shop_address SET is_default = 0 WHERE uid = $1 AND id_del = 0")
            .bind(uid)
            .execute(&pool)
            .await?;
        // 再设置目标为默认
        sqlx::query("UPDATE shop_address SET is_default = 1 WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 7. [REPOSITORY] - 按用户ID删除所有地址
    pub async fn delete_by_uid(uid: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("UPDATE shop_address SET id_del = 1 WHERE uid = $1")
            .bind(uid)
            .execute(&pool)
            .await?;
        Ok(())
    }
}

//////// END
