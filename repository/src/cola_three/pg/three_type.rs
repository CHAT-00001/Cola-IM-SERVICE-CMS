// repository/src/cola_three/pg/cola_fs  -- 仓储 - THREE - 服务类型 PG
// 2026/6/30 05:00

////////

use crate::pg_pool;
use cola_data::cola_three::entity::server_type::{ThreeServerTypeEntity, THREE_SERVER_TYPE_COLUMNS};

////////

/// # [REPOSITPRY] - 服务类型 仓储
pub struct ServerTypeRepo;

impl ServerTypeRepo {
    /////////

    /// 1. #[REPOSITORY] - 插入或更新
    pub async fn upsert(
        code: &str,
        name: &str,
        sort: i16,
        status: i16,
    ) -> Result<ThreeServerTypeEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO cola_three.server_type (code, name, sort, status)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (code) DO UPDATE SET name=$2, sort=$3, status=$4, updated_at=NOW()
               RETURNING {}"#,
            THREE_SERVER_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeServerTypeEntity>(&query)
            .bind(code)
            .bind(name)
            .bind(sort)
            .bind(status)
            .fetch_one(&pool)
            .await
    }

    /////////

    /// 2. #[REPOSITORY] - 列表
    pub async fn list() -> Result<Vec<ThreeServerTypeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_three.server_type ORDER BY sort ASC",
            THREE_SERVER_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeServerTypeEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 code 查询
    pub async fn find_by_code(code: &str) -> Result<Option<ThreeServerTypeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_three.servier_type WHERE code = $1 LIMIT 1",
            THREE_SERVER_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeServerTypeEntity>(&query)
            .bind(code)
            .fetch_optional(&pool)
            .await
    }
}

//////// END
