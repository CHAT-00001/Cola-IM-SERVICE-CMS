// repository/src/cola_user/pg/role/get.rs
// 仓储 - USER - pg - role - get 获取
// 2026/8/3 12:58 Created.

////////

use crate::pg_pool;
use cola_data::cola_user::entity::role::UserRoleEntity;
use sqlx::{self, PgPool};

////////

/// # [GET REPOSITORY] - 用户 角色 获取 仓储
pub struct UserRoleGetRepo;

// 构造函数
impl UserRoleGetRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 最新
    /// * `desc`: 主动关注
    /// * `sort`: 更新时间降序
    pub async fn pg_find_new_role_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserRoleEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 修复 SQL 语法错误：补充完整查询字段，并修正表名和返回值类型匹配
        let query = "SELECT * FROM \"cola_user\".\"role\" WHERE status = 1 ORDER BY id DESC LIMIT $1 OFFSET $2";

        sqlx::query_as::<_, UserRoleEntity>(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END